// Lightweight config-generator macro for in-repo demos.
// Not a full proc-macro; a pragmatic `macro_rules!` that generates a struct,
// a builder, Default impl and a basic validate() implementation. This is
// intended as an initial, in-repo approximation of an annotation processor
// without creating a new crate.

#[macro_export]
macro_rules! generate_config {
    (
        $name:ident, $builder:ident,
        $( validate = $validator:expr, )?
        {
            $(
                $( #[container(builder = $container_builder:path, method = $container_method:ident)] )?
                $field:ident : $ty:ty = $default:expr ;
            )*
        }
    ) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            $( pub $field: $ty, )*
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    $( $field: $default, )*
                }
            }
        }

        #[derive(Debug, Default)]
        pub struct $builder {
            $( $field: Option<$ty>, )*
        }

        impl $builder {
            $(
                $crate::generate_config!(@builder_methods $field, $ty $(, $container_builder, $container_method )?);
            )*

            pub fn build(self) -> Result<$name, $crate::config::validation::ConfigError> {
                let defaults = $name::default();
                let cfg = $name {
                    $( $field: self.$field.unwrap_or(defaults.$field), )*
                };
                cfg.validate()?;
                Ok(cfg)
            }
        }

        impl $name {
            pub fn builder() -> $builder { $builder::default() }

            pub fn validate(&self) -> Result<(), $crate::config::validation::ConfigError> {
                $( $validator(self)?; )?
                Ok(())
            }
        }

        impl $crate::config::base_types::Config for $name {}
    };

    (@builder_methods $field:ident, $ty:ty) => {
        pub fn $field(mut self, v: $ty) -> Self {
            self.$field = Some(v);
            self
        }
    };

    (@builder_methods $field:ident, $ty:ty, $container_builder:path, $container_method:ident) => {
        pub fn $field(mut self, v: $ty) -> Self {
            self.$field = Some(v);
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
}
