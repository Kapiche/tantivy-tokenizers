mod constants;
pub mod outer_punctuation;
pub mod possessive_contraction;

pub use outer_punctuation::OuterPunctuationFilter;
pub use possessive_contraction::PossessiveContractionFilter;

use constants::STOPWORDS_EN_BASE;

/// Unicode apostrophe characters to expand stopwords with.
const APOSTROPHES: [char; 8] = [
    '\u{0027}', // ' - Apostrophe
    '\u{2019}', // ' - Right single quotation mark
    '\u{02BC}', // ʼ - Modifier letter apostrophe
    '\u{02BB}', // ʻ - Modifier letter turned comma
    '\u{055A}', // ՚ - Armenian apostrophe
    '\u{A78B}', // Ꞌ - Latin capital letter saltillo
    '\u{A78C}', // ꞌ - Latin small letter saltillo
    '\u{FF07}', // ＇ - Fullwidth apostrophe
];

/// Check if a string contains any apostrophe character.
fn contains_apostrophe(s: &str) -> bool {
    s.chars().any(|c| APOSTROPHES.contains(&c))
}

/// Replace all apostrophe variants with a specific apostrophe character.
fn replace_apostrophes(s: &str, replacement: char) -> String {
    s.chars()
        .map(|c| {
            if APOSTROPHES.contains(&c) {
                replacement
            } else {
                c
            }
        })
        .collect()
}

/// Expand a stopword list to include all apostrophe variants.
///
/// For each stopword containing an apostrophe, generates versions
/// with every unicode apostrophe variant.
///
/// Example: "don't" becomes ["don't", "don't", "donʼt", "donʻt", ...]
pub fn expand_stopwords_with_apostrophe_variants(base_stopwords: &[&str]) -> Vec<String> {
    let mut expanded = Vec::new();

    for word in base_stopwords {
        if contains_apostrophe(word) {
            // Add a version with each apostrophe variant
            for &apos in &APOSTROPHES {
                expanded.push(replace_apostrophes(word, apos));
            }
        } else {
            expanded.push(word.to_string());
        }
    }

    expanded
}

/// Get the Kapiche custom English stopwords list with apostrophe variants expanded.
pub fn get_stopwords_filter_en() -> Vec<String> {
    expand_stopwords_with_apostrophe_variants(&STOPWORDS_EN_BASE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_apostrophe_with_standard_apostrophe() {
        assert!(contains_apostrophe("don't"));
    }

    #[test]
    fn test_contains_apostrophe_with_right_single_quotation() {
        assert!(contains_apostrophe("don't")); // U+2019
    }

    #[test]
    fn test_contains_apostrophe_without_apostrophe() {
        assert!(!contains_apostrophe("hello"));
    }

    #[test]
    fn test_contains_apostrophe_with_modifier_letter_apostrophe() {
        assert!(contains_apostrophe("donʼt")); // U+02BC
    }

    #[test]
    fn test_replace_apostrophes_with_standard_apostrophe() {
        assert_eq!(replace_apostrophes("don't", '\''), "don't");
    }

    #[test]
    fn test_replace_apostrophes_replaces_right_single_quotation() {
        assert_eq!(replace_apostrophes("don't", '\''), "don't"); // U+2019 -> U+0027
    }

    #[test]
    fn test_replace_apostrophes_replaces_modifier_letter() {
        assert_eq!(replace_apostrophes("donʼt", '\''), "don't"); // U+02BC -> U+0027
    }

    #[test]
    fn test_replace_apostrophes_no_apostrophe() {
        assert_eq!(replace_apostrophes("hello", '\''), "hello");
    }

    #[test]
    fn test_replace_apostrophes_multiple_apostrophes() {
        assert_eq!(replace_apostrophes("don't can't", '\''), "don't can't");
    }

    #[test]
    fn test_expand_stopwords_with_no_apostrophes() {
        let base = vec!["hello", "world"];
        let expanded = expand_stopwords_with_apostrophe_variants(&base);
        assert_eq!(expanded, vec!["hello", "world"]);
    }

    #[test]
    fn test_expand_stopwords_with_single_apostrophe_word() {
        let base = vec!["don't"];
        let expanded = expand_stopwords_with_apostrophe_variants(&base);
        // Should expand to 8 variants (one for each apostrophe type)
        assert_eq!(expanded.len(), 8);
        // Verify each variant exists
        assert!(expanded.contains(&"don't".to_string())); // U+0027
        assert!(expanded.contains(&"don't".to_string())); // U+2019
        assert!(expanded.contains(&"donʼt".to_string())); // U+02BC
    }

    #[test]
    fn test_expand_stopwords_with_mixed_words() {
        let base = vec!["hello", "don't", "world"];
        let expanded = expand_stopwords_with_apostrophe_variants(&base);
        // Should have: hello + 8 variants of don't + world = 10 words
        assert_eq!(expanded.len(), 10);
        assert!(expanded.contains(&"hello".to_string()));
        assert!(expanded.contains(&"world".to_string()));
        assert!(expanded.contains(&"don't".to_string()));
    }

    #[test]
    fn test_expand_stopwords_preserves_order() {
        let base = vec!["can't", "hello", "don't"];
        let expanded = expand_stopwords_with_apostrophe_variants(&base);
        // Should have: 8 variants of can't + hello + 8 variants of don't = 17 words
        assert_eq!(expanded.len(), 17);
        // First 8 should be can't variants
        assert_eq!(expanded[0], "can't"); // U+0027
                                          // Then hello
        assert_eq!(expanded[8], "hello");
        // Then 8 don't variants
        assert_eq!(expanded[9], "don't"); // U+0027
    }

    #[test]
    fn test_get_stopwords_filter_en_contains_apostrophe_variants() {
        let stopwords = get_stopwords_filter_en();

        // The stopwords list contains "don't" (U+0027), so after expansion
        // it should also contain other apostrophe variants
        assert!(stopwords.contains(&"don't".to_string())); // U+0027
        assert!(stopwords.contains(&"don't".to_string())); // U+2019
        assert!(stopwords.contains(&"donʼt".to_string())); // U+02BC

        // Same for "can't"
        assert!(stopwords.contains(&"can't".to_string())); // U+0027
        assert!(stopwords.contains(&"can't".to_string())); // U+2019
    }
}
