//! Configurations for Morph container logic built with `generate_config!`.
//! Demonstrates nested Shape/Context containers with compile-time builder wiring.

use crate::config::validation::ConfigValidation;

crate::generate_config!(
    ShapeConfig, ShapeConfigBuilder,
    validate = |shape: &ShapeConfig| {
        ConfigValidation::validate_positive(shape.width as f64, "shape.width")?;
        ConfigValidation::validate_positive(shape.height as f64, "shape.height")?;
        Ok(())
    },
    {
        width: usize = 1;
        height: usize = 1;
        label: String = String::from("shape");
    }
);

crate::generate_config!(
    ContextConfig, ContextConfigBuilder,
    {
        locale: String = String::from("en-US");
        timezone: String = String::from("UTC");
    }
);

crate::generate_config!(
    MorphConfig, MorphConfigBuilder,
    validate = |morph: &MorphConfig| {
        ConfigValidation::validate_property_key(&morph.morph_key)?;
        Ok(())
    },
    {
    #[container(builder = ShapeConfigBuilder, method = with_shape)]
        shape: ShapeConfig = ShapeConfig::default();
    #[container(builder = ContextConfigBuilder, method = with_context)]
        context: ContextConfig = ContextConfig::default();
        morph_key: String = String::from("default_morph");
    }
);

impl MorphConfig {
    /// Convenience accessor returning the contextualized Shape tuple used by the engine.
    pub fn morph_tuple(&self) -> (&ShapeConfig, &ContextConfig) {
        (&self.shape, &self.context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_nested_morph_config() {
        let config = MorphConfig::builder()
            .with_shape(|builder| builder.width(42).height(16))
            .unwrap()
            .with_context(|builder| builder.locale("fr-FR".to_string()))
            .unwrap()
            .morph_key("custom".to_string())
            .build()
            .expect("morph config should build");

        assert_eq!(config.shape.width, 42);
        assert_eq!(config.context.locale, "fr-FR");
        assert_eq!(config.morph_key, "custom");
    }
}
