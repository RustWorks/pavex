use pavex_bp_schema::Location;

use crate::rustdoc::Crate;

/// The location where a component was registered.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Registration {
    pub location: Location,
    pub kind: RegistrationKind,
}

/// The kind of registration that was used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegistrationKind {
    /// The component was registered using a blueprint method.
    Blueprint,
    /// The component was registered via a proc macro attribute.
    Attribute,
}

impl RegistrationKind {
    /// Returns `true` if the registration was performed using a blueprint method.
    pub fn is_blueprint(&self) -> bool {
        matches!(self, RegistrationKind::Blueprint)
    }

    /// Returns `true` if the registration was performed via a proc macro attribute.
    pub fn is_attribute(&self) -> bool {
        matches!(self, RegistrationKind::Attribute)
    }
}

impl Registration {
    /// The component was registered using a blueprint method.
    ///
    /// The method captures the location of the invocation
    /// via `std::panic::Location`.
    pub fn blueprint(location: Location) -> Self {
        Self {
            location,
            kind: RegistrationKind::Blueprint,
        }
    }

    /// The component was registered via a proc macro attribute.
    ///
    /// We use the span attached to its `rustdoc` item to determine the location of
    /// the attribute.
    ///
    /// Panics if there is no span for the item.
    pub fn annotated_item(item: &rustdoc_types::Item, krate: &Crate) -> Self {
        let Some(span) = item.span.as_ref() else {
            // TODO: We have empirically verified that this shouldn't happen for components annotated with our own macros,
            //   but it may happen for components that are generated from other macros or tools.
            //   In the future, we should handle this case more gracefully.
            unreachable!(
                "There is no span attached to the item for `{}` in the JSON documentation for {}",
                item.name.as_deref().unwrap_or(""),
                krate.crate_name()
            );
        };
        Self::attribute(span)
    }

    /// The component was registered via a proc macro attribute.
    ///
    /// We can use the span provided in the JSON output of `rustdoc` to
    /// determine the (approximate) location of the attribute.
    pub fn attribute(span: &rustdoc_types::Span) -> Self {
        Self {
            location: Location {
                line: (span.begin.0) as u32,
                column: (span.begin.1) as u32,
                file: span
                    .filename
                    .to_str()
                    .expect("Non UTF-8 path for one of your source files")
                    .to_owned(),
            },
            kind: RegistrationKind::Attribute,
        }
    }
}

impl From<Location> for Registration {
    fn from(location: Location) -> Self {
        Self::blueprint(location)
    }
}
