// Copyright (c) 2025 Rust-GDS Contributors
//
// Translated from Neo4j Graph Data Science:
// https://github.com/neo4j/graph-data-science
// pipeline/src/main/java/org/neo4j/gds/ml/pipeline/ResultToModelConverter.java

use crate::types::schema::GraphSchema;

/// Converter from training results to catalog models.
///
/// This trait converts raw training results (metrics, parameters, etc.)
/// into catalog-ready model containers that can be stored and retrieved.
///
/// # Type Parameters
///
/// * `MODEL` - The catalog model container type
/// * `RESULT` - The training result type
///
/// # Java Source (ResultToModelConverter.java)
/// ```java
/// public interface ResultToModelConverter<MODEL, RESULT> {
///     MODEL toModel(RESULT result, GraphSchema originalSchema);
/// }
/// ```
pub trait ResultToModelConverter<MODEL, RESULT> {
    /// Convert training result to a catalog model.
    ///
    /// # Arguments
    ///
    /// * `result` - The training result (trained model, metrics, etc.)
    /// * `original_schema` - The graph schema before node property steps
    ///
    /// Java: `MODEL toModel(RESULT result, GraphSchema originalSchema)`
    fn to_model(&self, result: RESULT, original_schema: &GraphSchema) -> MODEL;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::schema::GraphSchema;

    struct MockModel {
        name: String,
        schema: GraphSchema,
    }

    struct MockResult {
        accuracy: f64,
    }

    struct MockConverter;

    impl ResultToModelConverter<MockModel, MockResult> for MockConverter {
        fn to_model(&self, result: MockResult, original_schema: &GraphSchema) -> MockModel {
            MockModel {
                name: format!("model_with_accuracy_{}", result.accuracy),
                schema: original_schema.clone(),
            }
        }
    }

    #[test]
    fn test_result_to_model_conversion() {
        let converter = MockConverter;
        let result = MockResult { accuracy: 0.95 };
        let schema = GraphSchema::empty();

        let model = converter.to_model(result, &schema);

        assert_eq!(model.name, "model_with_accuracy_0.95");
    }
}
