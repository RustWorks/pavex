use pavex_bp_schema::RawIdentifiers;
use pavex_cli_diagnostic::CompilerDiagnostic;
use pavexc_attr_parser::AnnotationProperties;

use crate::{
    compiler::analyses::{
        computations::ComputationDb,
        user_components::{ErrorHandlerTarget, UserComponent},
    },
    rustdoc::{AnnotatedItem, CrateCollection},
};

use super::{AuxiliaryData, DiagnosticSink, Registration};

/// For each registered component, check if the item it refers to has been annotated with one of
/// Pavex's macros.
///
/// If that's the case, update its configuration accordingly.
/// Properties specified directly on the blueprint have higher priority than ones specified
/// in the annotation.
pub fn augment_from_annotation(
    aux: &mut AuxiliaryData,
    computation_db: &ComputationDb,
    krate_collection: &CrateCollection,
    diagnostics: &DiagnosticSink,
) {
    let component_ids: Vec<_> = aux.iter().map(|(id, _)| id).collect();
    for id in component_ids {
        if !matches!(
            &aux[id],
            UserComponent::WrappingMiddleware { .. }
                | UserComponent::PreProcessingMiddleware { .. }
                | UserComponent::PostProcessingMiddleware { .. }
        ) {
            continue;
        }
        let Some(source_id) = &computation_db[id].source_coordinates else {
            continue;
        };
        let Some(annotation) = krate_collection.annotation(source_id) else {
            continue;
        };
        let AnnotatedItem { properties, .. } = &annotation;
        let error_handler = match properties {
            AnnotationProperties::ErrorObserver
            | AnnotationProperties::ErrorHandler { .. }
            | AnnotationProperties::Constructor { .. }
            | AnnotationProperties::Methods
            | AnnotationProperties::Prebuilt { .. }
            | AnnotationProperties::Route { .. }
            | AnnotationProperties::Config { .. } => {
                panic!("Unexpected annotation kind")
            }
            AnnotationProperties::WrappingMiddleware { error_handler }
            | AnnotationProperties::PreProcessingMiddleware { error_handler }
            | AnnotationProperties::Fallback { error_handler }
            | AnnotationProperties::PostProcessingMiddleware { error_handler } => error_handler,
        };
        let Some(error_handler) = error_handler else {
            continue;
        };
        // The user may have used the `.error_handler` method on the `Blueprint` to override
        // the error handler provided by the annotation.
        if aux.fallible_id2error_handler_id.contains_key(&id) {
            // If that's the case, nothing to do here.
            continue;
        }

        let krate = match krate_collection.get_or_compute_crate_by_package_id(&source_id.package_id)
        {
            Ok(k) => k,
            Err(e) => {
                diagnostics.push(CompilerDiagnostic::builder(e).build());
                continue;
            }
        };

        // If that's not the case, we must process it!
        let identifiers = RawIdentifiers {
            created_at: annotation
                .created_at(krate, krate_collection.package_graph())
                .expect("Failed to determine `CreatedAt` for an annotated item"),
            created_by: annotation.created_by(),
            import_path: error_handler.to_owned(),
        };
        let identifiers_id = aux.identifiers_interner.get_or_intern(identifiers);
        let component = UserComponent::ErrorHandler {
            source: identifiers_id.into(),
            target: ErrorHandlerTarget::FallibleComponent { fallible_id: id },
        };
        let registration = {
            let item = krate.get_item_by_local_type_id(&source_id.rustdoc_item_id);
            Registration::annotated_item(&item, krate)
        };
        let error_handler_id = aux.intern_component(
            component,
            aux.id2scope_id[id],
            aux.id2lifecycle[id],
            registration,
        );
        aux.fallible_id2error_handler_id
            .insert(id, error_handler_id);
    }
}
