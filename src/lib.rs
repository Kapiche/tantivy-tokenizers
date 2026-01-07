//! Custom tokenizers and filters for Tantivy
//!
//! This crate provides Kapiche-specific tokenization filters and analyzers
//! for use with the Tantivy search engine.
//!
//! # Features
//!
//! - **OuterPunctuationFilter**: Removes leading and trailing punctuation from tokens,
//!   with configurable exceptions for specific characters (e.g., '#', '@')
//! - **PossessiveContractionFilter**: Removes possessive contractions (apostrophe-s variants)
//!   using Unicode-aware matching
//! - **Pre-built analyzers**: Ready-to-use analyzer configurations combining filters
//! - **Token counting utility**: Fast streaming token counter without memory allocation
//!
//! # Examples
//!
//! ```
//! use tantivy_tokenizers::{kapiche_analyzer, kapiche_analyzer_lower, count_tokens};
//!
//! // Create an analyzer
//! let mut analyzer = kapiche_analyzer();
//!
//! // Count tokens efficiently
//! let count = count_tokens(&mut analyzer, "John's #hashtag @mention");
//! assert_eq!(count, 3); // ["John", "#hashtag", "@mention"]
//! ```

pub mod analyzers;
pub mod filters;
pub mod utils;

// Re-export commonly used items for convenience
pub use analyzers::{kapiche_analyzer, kapiche_analyzer_lower, kapiche_analyzer_lower_with_stopwords};
pub use filters::{OuterPunctuationFilter, PossessiveContractionFilter};
pub use utils::count_tokens;
