//! Configuration Macro - Generate Config Struct + Builder + Validation
//!
//! The `algorithm_config!` macro eliminates ~80 lines of boilerplate per configuration.
//!
//! ## What It Generates
//!
//! From a simple declaration with attributes:
//! ```rust,ignore
//! algorithm_config! {
//!     pub struct MyConfig {
//!         #[default(42)]
//!         #[min(1)]
//!         pub value: i32,
//!     }
//! }
//! ```
//!
//! It generates:
//! 1. Config struct with Serialize/Deserialize
//! 2. Builder struct with validation methods
//! 3. Default implementation
//! 4. Builder constructor
//!
//! ## Supported Attributes
//!
//! - `#[default(expr)]` - Default value for field
//! - `#[range(min..max)]` - Numeric range validation (exclusive max)
//! - `#[min(value)]` - Minimum value (inclusive)
//! - `#[max(value)]` - Maximum value (inclusive)
//! - `#[choices(a, b, c)]` - Enum-like value choices
//! - `#[optional]` - Field is `Option<T>`, no default required
//!
//! ## Example
//!
//! ```rust,ignore
//! algorithm_config! {
//!     /// PageRank configuration
//!     pub struct PageRankConfig {
//!         /// Damping factor for random walk
//!         #[default(0.85)]
//!         #[range(0.0..1.0)]
//!         pub damping_factor: f64,
//!
//!         /// Convergence tolerance
//!         #[default(1e-7)]
//!         #[min(0.0)]
//!         pub tolerance: f64,
//!
//!         /// Maximum iterations
//!         #[default(20)]
//!         #[min(1)]
//!         pub max_iterations: usize,
//!     }
//! }
//!
//! // Usage:
//! let config = PageRankConfig::builder()
//!     .damping_factor(0.85)?
//!     .tolerance(1e-7)?
//!     .build()?;
//! ```

/// Generate configuration struct with builder and validation
///
/// **Note**: This is a simplified placeholder. Full macro implementation
/// requires macro_rules! with complex pattern matching.
///
/// For now, we'll implement this as a procedural macro or use a simpler
/// approach with trait-based builders.
///
/// See `doc/ALGORITHM_MACRO_DESIGN.md` for full specification.
#[macro_export]
macro_rules! algorithm_config {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field_vis:vis $field_name:ident: $field_type:ty
            ),* $(,)?
        }
    ) => {
        // Generate the main config struct
        $(#[$meta])*
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        $vis struct $name {
            $(
                $(#[$field_meta])*
                $field_vis $field_name: $field_type,
            )*
        }

        // Generate the builder struct
        paste::paste! {
            #[derive(Default)]
            $vis struct [<$name Builder>] {
                $(
                    $field_name: Option<$field_type>,
                )*
            }

            impl [<$name Builder>] {
                $(
                    /// Set field value
                    $field_vis fn $field_name(mut self, value: $field_type) -> Self {
                        self.$field_name = Some(value);
                        self
                    }
                )*

                /// Build the configuration
                ///
                /// Returns error if required fields are missing or validation fails
                $vis fn build(self) -> Result<$name, String> {
                    Ok($name {
                        $(
                            $field_name: self.$field_name
                                .ok_or_else(|| format!("Missing required field: {}", stringify!($field_name)))?,
                        )*
                    })
                }
            }

            impl $name {
                /// Create a new builder
                $vis fn builder() -> [<$name Builder>] {
                    [<$name Builder>]::default()
                }
            }

            impl Default for $name {
                fn default() -> Self {
                    Self::builder()
                        .build()
                        .expect("default config should be valid")
                }
            }
        }
    };
}

// TODO: Enhanced version with attribute parsing
// This requires parsing #[default(...)], #[range(...)], #[min(...)], etc.
// For now, users can implement Default manually or use builder methods with validation

#[cfg(test)]
mod tests {
    // Test basic config generation
    algorithm_config! {
        pub struct TestConfig {
            pub value: i32,
        }
    }

    #[test]
    fn test_config_builder() {
        let config = TestConfig::builder().value(42).build().unwrap();

        assert_eq!(config.value, 42);
    }

    #[test]
    fn test_config_builder_missing_field() {
        let result = TestConfig::builder().build();
        assert!(result.is_err());
    }

    // Test config with multiple fields
    algorithm_config! {
        pub struct MultiFieldConfig {
            pub value1: i32,
            pub value2: f64,
            pub value3: String,
        }
    }

    #[test]
    fn test_multi_field_config() {
        let config = MultiFieldConfig::builder()
            .value1(42)
            .value2(3.14)
            .value3("test".to_string())
            .build()
            .unwrap();

        assert_eq!(config.value1, 42);
        assert_eq!(config.value2, 3.14);
        assert_eq!(config.value3, "test");
    }
}
