//! Define Config - Pure FormShape Definition Macro
//!
//! This macro defines Container-Level FormShapes that represent the essential
//! structure of configuration objects. The macro operates on Pure Forms that
//! can be projected into various concrete implementations.
//!
//! ## Usage
//!
//! ```rust
//! define_config! {
//!     pub struct MyConfig {
//!         validate = |cfg| {
//!             ConfigValidation::validate_positive(cfg.count as f64, "count")?;
//!             Ok(())
//!         },
//!         name: String = "default".to_string(),
//!         count: usize = 42,
//!         enabled: bool = true,
//!     }
//! }
//! ```
//!
//! This generates:
//! - The config struct with Pure FormShape
//! - Builder pattern for Container+Contained unity
//! - Validation system
//! - Default implementations

#[macro_export]
macro_rules! define_config {
    // With validation
    (
        $( #[$struct_attr:meta] )*
        pub struct $name:ident {
            validate = $validator:expr,
            $(
                $( #[$field_attr:meta] )*
                $field:ident : $ty:ty = $default:expr $(,)?
            )*
        }
    ) => {
        define_config!(@internal $name, $validator, { $($(#[$field_attr])* $field : $ty = $default;)* });
    };
    
    // Without validation
    (
        $( #[$struct_attr:meta] )*
        pub struct $name:ident {
            $(
                $( #[$field_attr:meta] )*
                $field:ident : $ty:ty = $default:expr $(,)?
            )*
        }
    ) => {
        define_config!(@internal $name, (), { $($(#[$field_attr])* $field : $ty = $default;)* });
    };
    
    (@internal $name:ident, (), { $($(#[$field_attr:meta])* $field:ident : $ty:ty = $default:expr;)* }) => {
        define_config!(@generate $name, { $($(#[$field_attr])* $field : $ty = $default;)* });
        
        impl $name {
            // Validate the Pure FormShape (no custom validation)
            pub fn validate(&self) -> Result<(), $crate::config::validation::ConfigError> {
                Ok(())
            }
        }
    };
    
    (@internal $name:ident, $validator:expr, { $($(#[$field_attr:meta])* $field:ident : $ty:ty = $default:expr;)* }) => {
        define_config!(@generate $name, { $($(#[$field_attr])* $field : $ty = $default;)* });
        
        impl $name {
            // Validate the Pure FormShape (with custom validation)
            pub fn validate(&self) -> Result<(), $crate::config::validation::ConfigError> {
                $validator(self)?;
                Ok(())
            }
        }
    };
    
    (@generate $name:ident, { $($(#[$field_attr:meta])* $field:ident : $ty:ty = $default:expr;)* }) => {
        // Container: The Pure FormShape struct
        #[derive(Debug, Clone)]
        pub struct $name {
            $(
                $( #[$field_attr] )*
                pub $field: $ty,
            )*
        }

        // Default implementation for Pure FormShape
        impl Default for $name {
            fn default() -> Self {
                Self {
                    $(
                        $field: $default,
                    )*
                }
            }
        }

        // Container+Contained: Builder pattern for Organic Unity
        // Use paste crate to generate builder name
        paste::paste! {
            #[derive(Debug, Default)]
            pub struct [<$name Builder>] {
                $(
                    $field: Option<$ty>,
                )*
            }

            impl [<$name Builder>] {
                // Builder methods for each field
                $(
                    define_config!(@builder_method $field, $ty, $(#[$field_attr])*);
                )*

                // Build the Pure FormShape
                pub fn build(self) -> Result<$name, $crate::config::validation::ConfigError> {
                    let defaults = $name::default();
                    let config = $name {
                        $(
                            $field: self.$field.unwrap_or(defaults.$field),
                        )*
                    };
                    
                    // Validate the Pure FormShape
                    config.validate()?;
                    Ok(config)
                }
            }

            impl $name {
                // Create a new builder for the Pure FormShape
                pub fn builder() -> [<$name Builder>] {
                    [<$name Builder>]::default()
                }
            }
        }

        // Implement Config trait for the Pure FormShape
        impl $crate::config::base_types::Config for $name {}
    };
    
    // Builder method for container fields (with #[container(...)] attribute)
    (@builder_method $field:ident, $ty:ty, #[container(builder = $container_builder:path, method = $container_method:ident)]) => {
        pub fn $field(mut self, value: $ty) -> Self {
            self.$field = Some(value);
            self
        }

        pub fn $container_method<F>(mut self, build: F) -> Result<Self, $crate::config::validation::ConfigError>
        where
            F: FnOnce($container_builder) -> $container_builder,
        {
            let nested_builder = build(<$ty>::builder());
            let nested_value = nested_builder.build()?;
            self.$field = Some(nested_value);
            Ok(self)
        }
    };
    
    // Builder method for regular fields (fallback)
    (@builder_method $field:ident, $ty:ty, $(#[$field_attr:meta])*) => {
        pub fn $field(mut self, value: $ty) -> Self {
            self.$field = Some(value);
            self
        }
    };
}