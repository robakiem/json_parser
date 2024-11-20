//! # JSON Validator
//!
//! This library provides a parser for validating JSON structures.

use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

/// Parser for JSON documents.
#[derive(Parser)]
#[grammar = "json.pest"] 
pub struct JsonParser;

/// Represents a parsed JSON document.
#[derive(Debug)]
pub struct JsonDocument {
    /// Raw content of the JSON document
    pub content: String,
    /// Type of the root JSON element (object, array)
    pub root_type: JsonRootType,
}

/// Represents the type of the root JSON element
#[derive(Debug, PartialEq, Eq)]
pub enum JsonRootType {
    Object,
    Array,
}

/// Custom error for parsing failures.
#[derive(Error, Debug)]
pub enum JsonParseError {
    /// Error returned when parsing fails.
    #[error("Parsing error: {source}")]
    PestError {
        #[from]
        source: pest::error::Error<Rule>,
    },
    /// Error for an empty JSON input.
    #[error("Empty JSON input provided.")]
    EmptyJson,
    /// Error for unexpected root type.
    #[error("Unexpected root type: {0}")]
    UnexpectedRootType(String),
}

impl JsonDocument {
    /// Parses a JSON string and returns a `JsonDocument`.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice that holds the JSON to parse.
    ///
    /// # Errors
    ///
    /// Returns a `JsonParseError` if the input does not conform to JSON grammar.
    pub fn parse(input: &str) -> Result<Self, JsonParseError> {
        let trimmed_input = input.trim();
        if trimmed_input.is_empty() {
            return Err(JsonParseError::EmptyJson);
        }

        let pairs = JsonParser::parse(Rule::json, trimmed_input)?;

        // Get the first pair (root)
        let root = pairs.into_iter().next().ok_or(JsonParseError::EmptyJson)?;

        // Drill down to the actual root element inside the `json` rule
        let root = root.into_inner().next().ok_or(JsonParseError::EmptyJson)?;

        let root_type = match root.as_rule() {
            Rule::object => JsonRootType::Object,
            Rule::array => JsonRootType::Array,
            _ => {
                return Err(JsonParseError::UnexpectedRootType(format!(
                    "{:?}",
                    root.as_rule()
                )));
            }
        };

        Ok(JsonDocument {
            content: trimmed_input.to_string(),
            root_type,
        })
    }

    /// Checks if the JSON document is valid without creating a full document.
    pub fn is_valid(input: &str) -> bool {
        !input.trim().is_empty() && JsonParser::parse(Rule::json, input.trim()).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_object_json() {
        let json = r#"{"name": "test", "value": 42}"#;
        let doc = JsonDocument::parse(json);
        assert!(doc.is_ok());
        if let Ok(doc) = doc {
            assert_eq!(doc.root_type, JsonRootType::Object);
        }
    }

    #[test]
    fn test_valid_array_json() {
        let json = r#"[1, 2, 3, "test"]"#;
        let doc = JsonDocument::parse(json);
        assert!(doc.is_ok());
        if let Ok(doc) = doc {
            assert_eq!(doc.root_type, JsonRootType::Array);
        }
    }

    #[test]
    fn test_empty_json() {
        let json = r#"  "#;
        assert!(matches!(
            JsonDocument::parse(json),
            Err(JsonParseError::EmptyJson)
        ));
    }

    #[test]
    fn test_invalid_json() {
        let json = r#"{"name": "test", "value": }"#;
        assert!(matches!(
            JsonDocument::parse(json),
            Err(JsonParseError::PestError { .. })
        ));
    }

    #[test]
    fn test_unexpected_root_type() {
        // Assuming `Rule::json` doesn't accept primitive values like `true` as root
        let json = r#"true"#;
        assert!(matches!(
            JsonDocument::parse(json),
            Err(JsonParseError::UnexpectedRootType(_))
        ));
    }

    #[test]
    fn test_is_valid() {
        let valid_json = r#"{"key": "value"}"#;
        let invalid_json = r#"{"key": "value",}"#;
        let empty_json = r#"   "#;

        assert!(JsonDocument::is_valid(valid_json));
        assert!(!JsonDocument::is_valid(invalid_json));
        assert!(!JsonDocument::is_valid(empty_json));
    }
}
