use crate::filters::{get_stopwords_filter_en, OuterPunctuationFilter, PossessiveContractionFilter};
use tantivy::tokenizer::{LowerCaser, StopWordFilter, TextAnalyzer, WhitespaceTokenizer};

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

/// Creates the Kapiche tokenizer analyzer with lowercasing and stopword filtering.
///
/// This analyzer:
/// - Tokenizes on whitespace
/// - Converts to lowercase
/// - Removes leading/trailing punctuation (except '#' and '@' at the start)
/// - Removes stopwords (using Kapiche's custom 334-word English stopword list)
/// - Removes possessive contractions (e.g., "John's" -> "john")
///
/// Used for token counting and topic modeling where stopwords should be excluded.
/// For search indexing, use `kapiche_analyzer_lower()` instead to preserve stopwords.
pub fn kapiche_analyzer_lower_with_stopwords() -> TextAnalyzer {
    let stopwords_en = get_stopwords_filter_en();
    TextAnalyzer::builder(WhitespaceTokenizer::default())
        .filter(LowerCaser)
        .filter(OuterPunctuationFilter::new(vec!['#', '@']))
        .filter(StopWordFilter::remove(stopwords_en))
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

    #[test]
    fn test_kapiche_analyzer_lower_with_stopwords_english() {
        let mut analyzer = kapiche_analyzer_lower_with_stopwords();
        let mut token_stream = analyzer.token_stream("The quick brown fox");
        let mut tokens = vec![];
        let mut add_token = |token: &Token| {
            tokens.push(token.text.clone());
        };
        token_stream.process(&mut add_token);

        // "the" is removed (stopword), "quick", "brown", "fox" remain
        assert_eq!(tokens, vec!["quick", "brown", "fox"]);
    }

    #[test]
    fn test_stopwords_removed_after_lowercasing() {
        let mut analyzer = kapiche_analyzer_lower_with_stopwords();
        let mut token_stream = analyzer.token_stream("THE QUICK");
        let mut tokens = vec![];
        let mut add_token = |token: &Token| {
            tokens.push(token.text.clone());
        };
        token_stream.process(&mut add_token);

        // "THE" lowercased to "the", then removed as stopword
        assert_eq!(tokens, vec!["quick"]);
    }

    #[test]
    fn test_punctuation_and_possessive_with_stopwords() {
        let mut analyzer = kapiche_analyzer_lower_with_stopwords();
        let mut token_stream = analyzer.token_stream("John's the best!");
        let mut tokens = vec![];
        let mut add_token = |token: &Token| {
            tokens.push(token.text.clone());
        };
        token_stream.process(&mut add_token);

        // "John's" -> "john" (lowercased, possessive removed)
        // "the" -> removed (stopword)
        // "best!" -> "best" (punctuation removed)
        assert_eq!(tokens, vec!["john", "best"]);
    }
}
