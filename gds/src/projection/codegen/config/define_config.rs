//! Define Config Macro
//!
//! Generates configuration structs with builders, validation, and JSON parsing.
//!
//! ## Usage
//!
//! ```rust
//! define_config! {
//!     name: MyConfig,
//!     fields: {
//!         value: f64 = 1.0,
//!         enabled: bool = true,
//!     },
//!     validation: |cfg| {
//!         if cfg.value <= 0.0 {
//!             return Err(ConfigError::FieldValidation { 
//!                 field: "value".to_string(), 
//!                 message: "Must be positive".to_string() 
//!             });
//!         }
//!         Ok(())
//!     }
//! }
//! ```

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
            // Validate the config (no custom validation)
            pub fn validate(&self) -> Result<(), $crate::config::validation::ConfigError> {
                Ok(())
            }
        }
    };
    
    (@internal $name:ident, $validator:expr, { $($(#[$field_attr:meta])* $field:ident : $ty:ty = $default:expr;)* }) => {
        define_config!(@generate $name, { $($(#[$field_attr])* $field : $ty = $default;)* });
        
        impl $name {
            // Validate the config (with custom validation)
            pub fn validate(&self) -> Result<(), $crate::config::validation::ConfigError> {
                $validator(self)?;
                Ok(())
            }
        }
    };
    
    (@generate $name:ident, { $($(#[$field_attr:meta])* $field:ident : $ty:ty = $default:expr;)* }) => {
        // Config struct
        #[derive(Debug, Clone)]
        pub struct $name {
            $(
                $( #[$field_attr] )*
                pub $field: $ty,
            )*
        }

        // Default implementation
        impl Default for $name {
            fn default() -> Self {
                Self {
                    $(
                        $field: $default,
                    )*
                }
            }
        }

        // Builder pattern
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

                // Build the config
                pub fn build(self) -> Result<$name, $crate::config::validation::ConfigError> {
                    let defaults = $name::default();
                    let config = $name {
                        $(
                            $field: self.$field.unwrap_or(defaults.$field),
                        )*
                    };
                    
                    // Validate the config
                    config.validate()?;
                    Ok(config)
                }
            }

            impl $name {
                // Create a new builder
                pub fn builder() -> [<$name Builder>] {
                    [<$name Builder>]::default()
                }
            }
        }

        // Implement Config trait
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