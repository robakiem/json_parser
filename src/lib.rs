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
#[derive(Debug)]
pub enum JsonRootType {
    Object,
    Array,
}

/// Custom error for parsing failures.
#[derive(Error, Debug)]
pub enum JsonParseError {
    /// Error returned when parsing fails.
    #[error("Parsing error: {0}")]
    PestError(#[from] pest::error::Error<Rule>),
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
        let pairs = JsonParser::parse(Rule::json, input.trim())?;
        
        // Get the first pair (root)
        let root = pairs.into_iter().next().ok_or_else(|| {
            pest::error::Error::new_from_span(
                pest::error::ErrorVariant::CustomError { message: "Empty JSON".to_string() },
                pest::Span::new(input, 0, 0).unwrap()
            )
        })?;

        let root_type = match root.as_rule() {
            Rule::object => JsonRootType::Object,
            Rule::array => JsonRootType::Array,
            _ => return Err(pest::error::Error::new_from_span(
                pest::error::ErrorVariant::CustomError { 
                    message: format!("Unexpected root type: {:?}", root.as_rule()) 
                },
                root.as_span()
            ).into()),
        };

        Ok(JsonDocument {
            content: input.trim().to_string(),
            root_type,
        })
    }

    /// Checks if the JSON document is valid without creating a full document
    pub fn is_valid(input: &str) -> bool {
        JsonParser::parse(Rule::json, input.trim()).is_ok()
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
            assert!(matches!(doc.root_type, JsonRootType::Object));
        }
    }

    #[test]
    fn test_valid_array_json() {
        let json = r#"[1, 2, 3, "test"]"#;
        let doc = JsonDocument::parse(json);
        assert!(doc.is_ok());
        if let Ok(doc) = doc {
            assert!(matches!(doc.root_type, JsonRootType::Array));
        }
    }

    #[test]
    fn test_invalid_json() {
        let json = r#"{"name": "test", "value": }"#;
        assert!(JsonDocument::parse(json).is_err());
    }
}