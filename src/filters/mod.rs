mod constants;
pub mod outer_punctuation;
pub mod possessive_contraction;

pub use outer_punctuation::OuterPunctuationFilter;
pub use possessive_contraction::PossessiveContractionFilter;

use constants::STOPWORDS_EN;

/// Get the Kapiche custom English stopwords list.
pub fn get_stopwords_filter_en() -> Vec<String> {
    let mut words = Vec::new();
    for word in STOPWORDS_EN {
        words.push(String::from(word))
    }
    words
}
