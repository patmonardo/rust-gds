use std::cmp::Ordering;

// Constants
const REQUIRED_SIMILARITY: f64 = 0.8;
const MAX_SCORE: f64 = 1.0;
const MIN_SCORE: f64 = 0.0;
const WINKLER_SCALING: f64 = 0.1;
const MAX_PREFIX_LENGTH_BOOST: usize = 4;

/// Format suggestions with a prefix message and similar candidates.
pub fn pretty_suggestions(prefix: &str, value: &str, candidates: &[String]) -> String {
    let mut result = String::from(prefix);
    let similar_candidates = similar_strings(value, candidates);

    let suffix = if similar_candidates.is_empty() {
        None
    } else if similar_candidates.len() == 1 {
        Some(format!("Did you mean `{}`?", similar_candidates[0]))
    } else {
        let joined = similar_candidates.join("`, `");
        Some(format!("Did you mean one of [`{}`]?", joined))
    };

    if let Some(s) = suffix {
        result.push(' ');
        result.push_str(&s);
    }

    result
}

/// Find strings in candidates that are similar to the given value.
pub fn similar_strings(value: &str, candidates: &[String]) -> Vec<String> {
    similar_strings_with_converter(value, candidates, case_sensitive)
}

/// Find strings in candidates that are similar to the given value (case insensitive).
pub fn similar_strings_ignore_case(value: &str, candidates: &[String]) -> Vec<String> {
    similar_strings_with_converter(value, candidates, case_insensitive)
}

/// Calculate Jaro similarity between two strings.
pub fn jaro(s1: &str, s2: &str) -> f64 {
    jaro_with_converter(s1, s2, case_sensitive)
}

/// Calculate Jaro-Winkler similarity between two strings.
pub fn jaro_winkler(s1: &str, s2: &str) -> f64 {
    jaro_winkler_with_converter(s1, s2, case_sensitive)
}

// Private implementation functions

fn similar_strings_with_converter(
    value: &str,
    candidates: &[String],
    converter: fn(char) -> char,
) -> Vec<String> {
    let mut scored: Vec<(&String, f64)> = candidates
        .iter()
        .map(|candidate| {
            let score = jaro_winkler_with_converter(value, candidate, converter);
            (candidate, score)
        })
        .filter(|(_, score)| *score > REQUIRED_SIMILARITY)
        .collect();

    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
    scored.into_iter().map(|(s, _)| s.clone()).collect()
}

fn jaro_with_converter(s1: &str, s2: &str, converter: fn(char) -> char) -> f64 {
    let chars1: Vec<char> = s1.chars().map(converter).collect();
    let chars2: Vec<char> = s2.chars().map(converter).collect();
    let len1 = chars1.len();
    let len2 = chars2.len();

    if len1 == 0 && len2 == 0 {
        return MAX_SCORE;
    }
    if len1 == 0 || len2 == 0 {
        return MIN_SCORE;
    }
    if len1 == 1 && len2 == 1 {
        return if chars1[0] == chars2[0] {
            MAX_SCORE
        } else {
            MIN_SCORE
        };
    }

    let search_range = (len1.max(len2) / 2).saturating_sub(1);
    let mut consumed2 = vec![false; len2];

    let mut number_of_matches = 0;
    let mut number_of_transpositions = 0;
    let mut match_index2 = 0;

    for (i, &ch1) in chars1.iter().enumerate().take(len1) {
        let min_bound = if i > search_range {
            i.saturating_sub(search_range)
        } else {
            0
        };
        let max_bound = (len2 - 1).min(i + search_range);

        if min_bound > max_bound {
            continue;
        }

        for j in min_bound..=max_bound {
            let ch2 = chars2[j];
            if ch1 == ch2 && !consumed2[j] {
                consumed2[j] = true;
                number_of_matches += 1;
                if j < match_index2 {
                    number_of_transpositions += 1;
                }
                match_index2 = j;
                break;
            }
        }
    }

    if number_of_matches == 0 {
        return MIN_SCORE;
    }

    let matches = number_of_matches as f64;
    let len1_f = len1 as f64;
    let len2_f = len2 as f64;
    ((matches / len1_f)
        + (matches / len2_f)
        + ((matches - number_of_transpositions as f64) / matches))
        / 3.0
}

fn jaro_winkler_with_converter(s1: &str, s2: &str, converter: fn(char) -> char) -> f64 {
    let jaro = jaro_with_converter(s1, s2, converter);

    let common_length = s1.len().min(s2.len()).min(MAX_PREFIX_LENGTH_BOOST + 1);

    let mut prefix_length = 0;
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();

    for i in 0..common_length {
        let ch1 = converter(chars1[i]);
        let ch2 = converter(chars2[i]);
        if ch1 != ch2 {
            break;
        }
        prefix_length += 1;
    }

    let jaro_winkler = jaro + (WINKLER_SCALING * prefix_length as f64 * (1.0 - jaro));
    jaro_winkler.min(MAX_SCORE)
}

// Character converters
fn case_sensitive(c: char) -> char {
    c
}

fn case_insensitive(c: char) -> char {
    c.to_lowercase().next().unwrap_or(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jaro_identical() {
        assert_eq!(jaro("test", "test"), 1.0);
    }

    #[test]
    fn test_jaro_different() {
        assert_eq!(jaro("test", "abcd"), 0.0);
    }

    #[test]
    fn test_jaro_similar() {
        let score = jaro("martha", "marhta");
        assert!(score > 0.9 && score < 1.0);
    }

    #[test]
    fn test_jaro_winkler() {
        let score = jaro_winkler("martha", "marhta");
        assert!(score > 0.9);
    }

    #[test]
    fn test_similar_strings() {
        let candidates = vec!["test".to_string(), "best".to_string(), "rest".to_string()];
        let similar = similar_strings("test", &candidates);
        assert!(similar.contains(&"test".to_string()));
        assert!(similar.contains(&"best".to_string()));
    }

    #[test]
    fn test_pretty_suggestions_empty() {
        let candidates = vec!["foo".to_string(), "bar".to_string()];
        let result = pretty_suggestions("Invalid value:", "xyz", &candidates);
        assert_eq!(result, "Invalid value:");
    }

    #[test]
    fn test_pretty_suggestions_single() {
        let candidates = vec!["test".to_string(), "foo".to_string()];
        let result = pretty_suggestions("Invalid value:", "tset", &candidates);
        assert!(result.contains("Did you mean"));
    }
}
