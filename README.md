# tantivy-tokenizers

Custom tokenizers and filters for [Tantivy](https://github.com/quickwit-oss/tantivy), used by Kapiche.

## Overview

This crate provides specialized text analysis components for the Tantivy search engine:

- **OuterPunctuationFilter**: Removes leading and trailing punctuation from tokens, with configurable exceptions
- **PossessiveContractionFilter**: Removes possessive contractions using Unicode-aware apostrophe matching
- **Pre-built analyzers**: Ready-to-use configurations combining tokenizers and filters
- **Token counting utility**: Fast streaming token counter without memory allocation

## Features

### Custom Filters

#### OuterPunctuationFilter

Strips leading and trailing punctuation from tokens while preserving specified characters at the beginning.

- Handles Unicode punctuation categories
- Allows exceptions for specific leading characters (e.g., '#' for hashtags, '@' for mentions)
- Preserves emojis and symbols

#### PossessiveContractionFilter

Removes possessive contractions (apostrophe-s variants) from tokens.

- Supports 8 Unicode apostrophe variants (U+0027, U+2019, U+02BC, etc.)
- Robust handling of edge cases

### Pre-built Analyzers

```rust
use tantivy_tokenizers::{kapiche_analyzer, kapiche_analyzer_lower};

// Standard Kapiche analyzer
let analyzer = kapiche_analyzer();
// Input: "#HashTag @mention test's"
// Output: ["#HashTag", "@mention", "test"]

// Lowercase variant
let analyzer = kapiche_analyzer_lower();
// Input: "#HashTag @Mention Test's"
// Output: ["#hashtag", "@mention", "test"]
```

### Token Counting

Fast streaming token counter that avoids materializing tokens into collections:

```rust
use tantivy_tokenizers::{kapiche_analyzer, count_tokens};

let mut analyzer = kapiche_analyzer();
let count = count_tokens(&mut analyzer, "John's #hashtag @mention");
assert_eq!(count, 3);
```

This is significantly faster than creating a Vec/Set of all tokens and counting them, especially for large texts.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
tantivy-tokenizers = { git = "https://github.com/Kapiche/tantivy-tokenizers", tag = "v0.1.0" }
```

### With tantivy-py

This crate is designed to be used with Kapiche's fork of tantivy-py. The analyzers are automatically registered as:

- `"kapiche_tokenizer"` - Standard analyzer
- `"kapiche_tokenizer_lower"` - Lowercase variant

```python
from tantivy import Index

index = Index.open("path/to/index")
# "kapiche_tokenizer" and "kapiche_tokenizer_lower" are pre-registered
```

### Standalone Rust Usage

```rust
use tantivy::tokenizer::{TextAnalyzer, Token, TokenStream};
use tantivy_tokenizers::kapiche_analyzer;

let mut analyzer = kapiche_analyzer();
let mut token_stream = analyzer.token_stream("Test text");

while token_stream.advance() {
    let token = token_stream.token();
    println!("{}", token.text);
}
```

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

All tests include comprehensive Unicode edge cases for punctuation handling and emoji support.

### Documentation

```bash
cargo doc --open
```

## License

MIT

## Credits

Developed by Kapiche for use with the Tantivy search engine.

Filter implementations based on Kapiche's production tokenization requirements for text analysis and search.
