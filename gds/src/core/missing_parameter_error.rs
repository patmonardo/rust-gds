use crate::core::string_similarity;
use std::collections::HashSet;
use thiserror::Error;

/// Custom error for missing mandatory configuration parameters.
#[derive(Debug, Error)]
#[error("{0}")]
pub struct MissingParameterError(String);

impl MissingParameterError {
    /// Creates a MissingParameterError for a missing parameter value.
    pub fn missing_value_for(key: &str, candidates: &[String]) -> Self {
        Self(Self::missing_value_for_message(key, candidates))
    }

    /// Creates a MissingParameterError from a HashSet of candidates.
    pub fn missing_value_for_set(key: &str, candidates: &HashSet<String>) -> Self {
        let candidate_vec: Vec<String> = candidates.iter().cloned().collect();
        Self::missing_value_for(key, &candidate_vec)
    }

    /// Generates a message for a missing parameter value, including suggestions.
    fn missing_value_for_message(key: &str, candidates: &[String]) -> String {
        let suggestions = string_similarity::similar_strings_ignore_case(key, candidates);
        Self::missing_value_message(key, &suggestions)
    }

    /// Formats the error message based on the key and any suggestions.
    fn missing_value_message(key: &str, suggestions: &[String]) -> String {
        if suggestions.is_empty() {
            format!(
                "No value specified for the mandatory configuration parameter `{}`",
                key
            )
        } else if suggestions.len() == 1 {
            format!(
                "No value specified for the mandatory configuration parameter `{}` (a similar parameter exists: [{}])",
                key, suggestions[0]
            )
        } else {
            format!(
                "No value specified for the mandatory configuration parameter `{}` (similar parameters exist: [{}])",
                key,
                suggestions.join(", ")
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_parameter_no_suggestions() {
        let err = MissingParameterError::missing_value_for("unknownKey", &[]);
        assert_eq!(
            err.to_string(),
            "No value specified for the mandatory configuration parameter `unknownKey`"
        );
    }

    #[test]
    fn test_missing_parameter_single_suggestion() {
        let candidates = vec!["configValue".to_string()];
        let err = MissingParameterError::missing_value_for("configValu", &candidates);
        assert!(err.to_string().contains("similar parameter exists"));
        assert!(err.to_string().contains("configValue"));
    }

    #[test]
    fn test_missing_parameter_multiple_suggestions() {
        let candidates = vec![
            "configValue".to_string(),
            "configVal".to_string(),
            "testParam".to_string(),
        ];
        let err = MissingParameterError::missing_value_for("configValu", &candidates);
        assert!(err.to_string().contains("similar parameters exist"));
    }
}
