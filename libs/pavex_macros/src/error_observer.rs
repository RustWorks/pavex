use convert_case::{Case, Casing};
use quote::{format_ident, quote, quote_spanned};
use syn::Ident;

use crate::{
    fn_like::{Callable, CallableAnnotation, ImplContext},
    utils::AnnotationCodegen,
};

#[derive(darling::FromMeta, Debug, Clone)]
/// The available options for the error observer macro.
pub struct InputSchema {
    pub id: Option<syn::Ident>,
}

impl TryFrom<InputSchema> for Properties {
    type Error = darling::Error;

    fn try_from(input: InputSchema) -> Result<Self, Self::Error> {
        let InputSchema { id } = input;
        Ok(Properties { id })
    }
}

#[derive(darling::FromMeta, Debug, Clone, PartialEq, Eq)]
pub struct Properties {
    pub id: Option<syn::Ident>,
}

pub struct ErrorObserverAnnotation;

impl CallableAnnotation for ErrorObserverAnnotation {
    const PLURAL_COMPONENT_NAME: &str = "Error observers";

    const ATTRIBUTE: &str = "#[pavex::error_observer]";

    type InputSchema = InputSchema;

    fn codegen(
        _impl_: Option<ImplContext>,
        metadata: Self::InputSchema,
        item: Callable,
    ) -> Result<AnnotationCodegen, proc_macro::TokenStream> {
        let properties = metadata
            .try_into()
            .map_err(|e: darling::Error| e.write_errors())?;
        Ok(emit(item.sig.ident, properties))
    }
}

/// Decorate the input with a `#[diagnostic::pavex::wrap]` attribute
/// that matches the provided properties.
fn emit(name: Ident, properties: Properties) -> AnnotationCodegen {
    let Properties { id } = properties;
    // Use the span of the function name if no identifier is provided.
    let id_span = id.as_ref().map(|id| id.span()).unwrap_or(name.span());

    let name = name.to_string();

    // If the user didn't specify an identifier, generate one based on the function name.
    let id = id.unwrap_or_else(|| format_ident!("{}", name.to_case(Case::Constant)));
    let properties = quote! {
        id = #id,
    };

    let id_docs = format!(
        r#"A strongly-typed id to add [`{name}`] as an error observer to your Pavex application.

# Example

```rust,ignore
use pavex::blueprint::Blueprint;
// [...]
// ^ Import `{id}` here

let mut bp = Blueprint::new();
// Add `{name}` as an error observer to your application.
bp.error_observer({id});
```"#
    );
    let id_def = quote_spanned! { id_span =>
        #[doc = #id_docs]
        pub const #id: ::pavex::blueprint::reflection::WithLocation<::pavex::blueprint::reflection::RawIdentifiers> =
            ::pavex::with_location!(::pavex::blueprint::reflection::RawIdentifiers {
                import_path: concat!(module_path!(), "::", #name),
                macro_name: "error_observer",
            });
    };

    AnnotationCodegen {
        id_def: Some(id_def),
        new_attributes: vec![
            syn::parse_quote! { #[diagnostic::pavex::error_observer(#properties)] },
        ],
    }
}
