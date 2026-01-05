use tantivy::tokenizer::TextAnalyzer;

/// Count non-stopped tokens in text without allocating a collection.
///
/// This is significantly faster than materializing all tokens into a Vec/Set
/// and then counting them, especially for large texts.
///
/// # Arguments
/// * `analyzer` - The TextAnalyzer to use for tokenization
/// * `text` - The text to analyze
///
/// # Returns
/// The count of tokens (excluding stopped tokens)
///
/// # Example
/// ```
/// use tantivy::tokenizer::{SimpleTokenizer, TextAnalyzer};
/// use tantivy_tokenizers::count_tokens;
///
/// let mut analyzer = TextAnalyzer::builder(SimpleTokenizer::default()).build();
/// let count = count_tokens(&mut analyzer, "hello world");
/// assert_eq!(count, 2);
/// ```
pub fn count_tokens(analyzer: &mut TextAnalyzer, text: &str) -> usize {
    let mut token_stream = analyzer.token_stream(text);
    let mut count = 0;

    while token_stream.advance() {
        // In tantivy, stopped tokens are marked with position == usize::MAX
        if token_stream.token().position != usize::MAX {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use tantivy::tokenizer::{Language, SimpleTokenizer, StopWordFilter, TextAnalyzer};

    #[test]
    fn test_count_tokens_basic() {
        let mut analyzer = TextAnalyzer::builder(SimpleTokenizer::default()).build();
        assert_eq!(count_tokens(&mut analyzer, "hello world"), 2);
        assert_eq!(count_tokens(&mut analyzer, ""), 0);
        assert_eq!(count_tokens(&mut analyzer, "single"), 1);
    }

    #[test]
    fn test_count_tokens_with_stopwords() {
        let mut analyzer = TextAnalyzer::builder(SimpleTokenizer::default())
            .filter(StopWordFilter::new(Language::English).unwrap())
            .build();

        // "the" and "a" are stop words
        assert_eq!(count_tokens(&mut analyzer, "the cat"), 1);
        assert_eq!(count_tokens(&mut analyzer, "a dog"), 1);
        assert_eq!(count_tokens(&mut analyzer, "the quick brown fox"), 3);
    }

    #[test]
    fn test_count_tokens_with_kapiche_analyzer() {
        let mut analyzer = crate::analyzers::kapiche_analyzer();

        assert_eq!(count_tokens(&mut analyzer, "test's example"), 2);
        assert_eq!(count_tokens(&mut analyzer, "#hashtag @mention"), 2);
        assert_eq!(count_tokens(&mut analyzer, "...word..."), 1);
    }
}
