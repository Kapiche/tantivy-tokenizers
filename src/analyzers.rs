use tantivy::tokenizer::{LowerCaser, TextAnalyzer, WhitespaceTokenizer};
use crate::filters::{OuterPunctuationFilter, PossessiveContractionFilter};

/// Creates the Kapiche tokenizer analyzer.
/// Combines WhitespaceTokenizer with OuterPunctuationFilter and PossessiveContractionFilter.
///
/// This analyzer:
/// - Tokenizes on whitespace
/// - Removes leading/trailing punctuation (except '#' and '@' at the start)
/// - Removes possessive contractions (e.g., "John's" -> "John")
pub fn kapiche_analyzer() -> TextAnalyzer {
    TextAnalyzer::builder(WhitespaceTokenizer::default())
        .filter(OuterPunctuationFilter::new(vec!['#', '@']))
        .filter(PossessiveContractionFilter)
        .build()
}

/// Creates the Kapiche tokenizer analyzer with lowercasing.
/// Combines WhitespaceTokenizer with LowerCaser, OuterPunctuationFilter,
/// and PossessiveContractionFilter.
///
/// This analyzer:
/// - Tokenizes on whitespace
/// - Converts to lowercase
/// - Removes leading/trailing punctuation (except '#' and '@' at the start)
/// - Removes possessive contractions (e.g., "John's" -> "john")
pub fn kapiche_analyzer_lower() -> TextAnalyzer {
    TextAnalyzer::builder(WhitespaceTokenizer::default())
        .filter(LowerCaser)
        .filter(OuterPunctuationFilter::new(vec!['#', '@']))
        .filter(PossessiveContractionFilter)
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tantivy::tokenizer::{Token, TokenStream};

    #[test]
    fn test_kapiche_analyzer() {
        let mut analyzer = kapiche_analyzer();
        let mut token_stream = analyzer.token_stream("#HashTag @mention test's");
        let mut tokens = vec![];
        let mut add_token = |token: &Token| {
            tokens.push(token.text.clone());
        };
        token_stream.process(&mut add_token);

        assert_eq!(tokens, vec!["#HashTag", "@mention", "test"]);
    }

    #[test]
    fn test_kapiche_analyzer_lower() {
        let mut analyzer = kapiche_analyzer_lower();
        let mut token_stream = analyzer.token_stream("#HashTag @Mention Test's");
        let mut tokens = vec![];
        let mut add_token = |token: &Token| {
            tokens.push(token.text.clone());
        };
        token_stream.process(&mut add_token);

        assert_eq!(tokens, vec!["#hashtag", "@mention", "test"]);
    }
}
