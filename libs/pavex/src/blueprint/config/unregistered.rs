use pavex_bp_schema::Type;

use crate::blueprint::Blueprint;
use crate::blueprint::constructor::CloningStrategy;
use crate::blueprint::conversions::raw_identifiers2type;
use crate::blueprint::reflection::{RawIdentifiers, WithLocation};

use super::RegisteredConfigType;

/// A configuration type that has not yet been registered with a [`Blueprint`].
///
/// # Guide
///
/// Check out the ["Configuration"](https://pavex.dev/docs/guide/configuration)
/// section of Pavex's guide for a thorough introduction to Pavex's configuration system.
#[derive(Clone, Debug)]
pub struct ConfigType {
    pub(in crate::blueprint) type_: Type,
    pub(in crate::blueprint) key: String,
    pub(in crate::blueprint) cloning_strategy: Option<CloningStrategy>,
    pub(in crate::blueprint) default_if_missing: Option<bool>,
    pub(in crate::blueprint) include_if_unused: Option<bool>,
}

impl ConfigType {
    /// Create a new (unregistered) configuration type.
    ///
    /// Check out the documentation of [`Blueprint::config`] for more details
    /// on configuration types.
    #[track_caller]
    pub fn new(key: &str, type_: WithLocation<RawIdentifiers>) -> Self {
        Self {
            type_: raw_identifiers2type(type_),
            key: key.into(),
            cloning_strategy: None,
            default_if_missing: None,
            include_if_unused: None,
        }
    }

    /// Set the cloning strategy for this configuration type.
    ///
    /// Check out the documentation of [`CloningStrategy`] for more details.
    pub fn cloning(mut self, cloning_strategy: CloningStrategy) -> Self {
        self.cloning_strategy = Some(cloning_strategy);
        self
    }

    /// Set the cloning strategy to [`CloningStrategy::CloneIfNecessary`].
    /// Check out [`cloning`](Self::cloning) for more details.
    pub fn clone_if_necessary(self) -> Self {
        self.cloning(CloningStrategy::CloneIfNecessary)
    }

    /// Set the cloning strategy to [`CloningStrategy::NeverClone`].
    /// Check out [`cloning`](Self::cloning) for more details.
    pub fn never_clone(self) -> Self {
        self.cloning(CloningStrategy::NeverClone)
    }

    /// Use the default configuration values returned by [`Default::default`]
    /// if the user has not specified its own configuration for this type.
    ///
    /// # Requirements
    ///
    /// The configuration type *must* implement the [`Default`] trait
    /// to support this option.
    ///
    /// # Implementation notes
    ///
    /// `default_if_missing` adds a `#[serde(default)]` attribute on the corresponding
    /// configuration key in the code-generated `ApplicationConfig` struct.
    pub fn default_if_missing(mut self) -> Self {
        self.default_if_missing = Some(true);
        self
    }

    /// Include this configuration entry in the generated `ApplicationConfig` struct
    /// even if the type is never used by the application.
    pub fn include_if_unused(mut self) -> Self {
        self.include_if_unused = Some(true);
        self
    }

    /// Register this configuration type with a [`Blueprint`].
    ///
    /// Check out the documentation of [`Blueprint::config`] for more details.
    pub fn register(self, bp: &mut Blueprint) -> RegisteredConfigType {
        bp.register_config_type(self)
    }
}
