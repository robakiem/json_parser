use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;
use json_parser::JsonDocument;
use anyhow::{Context, Result};


#[derive(Parser)]
#[grammar = "json.pest"]
pub struct JsonParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    // Test parsing valid JSON objects
    #[test]
    fn test_valid_object() {
        let valid_objects = vec![
            r#"{}"#,
            r#"{"key": "value"}"#,
            r#"{"name": "John", "age": 30}"#,
            r#"{"nested": {"inner": "value"}}"#,
        ];

        for obj in valid_objects {
            assert!(JsonParser::parse(Rule::object, obj).is_ok(), "Failed to parse: {}", obj);
        }
    }

    // Test parsing invalid JSON objects
    #[test]
    fn test_invalid_object() {
        let invalid_objects = vec![
            r#"{key: "value"}"#,  // Missing quotes
            r#"{"key": }"#,        // Incomplete value
            r#"{,}"#,              // Invalid syntax
            r#"{"key"}"#,          // Missing value
        ];

        for obj in invalid_objects {
            assert!(JsonParser::parse(Rule::object, obj).is_err(), "Should fail parsing: {}", obj);
        }
    }

    #[test]
    fn test_parse_minimal_cases() -> anyhow::Result<()> {
        let minimal_object = "{}";
        let minimal_array = "[]";
        let minimal_string = r#""""#;
        let minimal_number = "0";

        let parsed_object =JsonParser::parse(Rule::object, minimal_object)?;
        let parsed_array = JsonParser::parse(Rule::array, minimal_array)?;
        let parsed_string =JsonParser::parse(Rule::string, minimal_string)?;
        let parsed_number =JsonParser::parse(Rule::number, minimal_number)?;

        assert_eq!(parsed_object.as_str(), minimal_object);
        assert_eq!(parsed_array.as_str(), minimal_array);
        assert_eq!(parsed_string.as_str(), minimal_string);
        assert_eq!(parsed_number.as_str(), minimal_number);

        Ok(())
    }

    #[test]
    fn test_parse_invalid_json() {
        let invalid_data1 = r#""unterminated string"#;
        let invalid_data2 = r#"{missing: "key quotes"}"#;
        let invalid_data3 = r#"[unquoted, elements]"#;

        assert!(JsonParser::parse(Rule::json, invalid_data1).is_err());
        assert!(JsonParser::parse(Rule::json, invalid_data2).is_err());
        assert!(JsonParser::parse(Rule::json, invalid_data3).is_err());
    }

    // Test parsing valid JSON arrays
    #[test]
    fn test_valid_arrays() {
        let valid_arrays = vec![
            r#"[]"#,
            r#"[1, 2, 3]"#,
            r#"["string", 42, true, null]"#,
            r#"[[1, 2], [3, 4]]"#,
        ];

        for arr in valid_arrays {
            assert!(JsonParser::parse(Rule::array, arr).is_ok(), "Failed to parse array: {}", arr);
        }
    }

    // Test parsing invalid JSON arrays
    #[test]
    fn test_invalid_arrays() {
        let invalid_arrays = vec![
            r#"[,]"#,              // Empty elements
            r#"[1, ]"#,            // Trailing comma
            r#"[1 2]"#,            // Missing comma
            r#"[{"key": }]"#,      // Invalid nested object
        ];

        for arr in invalid_arrays {
            assert!(JsonParser::parse(Rule::array, arr).is_err(), "Should fail parsing array: {}", arr);
        }
    }
    #[test]
    fn test_parse_rule_escaped_string() -> anyhow::Result<()> {
        let data_to_parse1 = r#""hello \"world\"""#;
        let data_to_parse2 = r#""line1\nline2""#;
        let data_to_parse3 = r#""unicode \\u1234""#;

        let parsed_data1 = JsonParser::parse(Rule::string, data_to_parse1)?;
        let parsed_data2 = JsonParser::parse(Rule::string, data_to_parse2)?;
        let parsed_data3 = JsonParser::parse(Rule::string, data_to_parse3)?;

        assert_eq!(parsed_data1.as_str(), data_to_parse1);
        assert_eq!(parsed_data2.as_str(), data_to_parse2);
        assert_eq!(parsed_data3.as_str(), data_to_parse3);

        Ok(())
    }

    // Test parsing valid JSON strings
    #[test]
    fn test_valid_strings() {
        let valid_strings = vec![
            r#""""#,               // Empty string
            r#""hello world""#,    // Simple string
            r#""with \"quotes\"""#, // Escaped quotes
            r#""unicode: \u00A9""#, // Unicode escape
        ];

        for s in valid_strings {
            assert!(JsonParser::parse(Rule::string, s).is_ok(), "Failed to parse string: {}", s);
        }
    }

    // Test parsing invalid JSON strings
    #[test]
    fn test_invalid_strings() {
        let invalid_strings = vec![
            r#"'single quotes'"#,  // Wrong quotes
            r#""unclosed string"#, // Unclosed string
            r#""invalid \x escape""#, // Invalid escape
        ];

        for s in invalid_strings {
            assert!(JsonParser::parse(Rule::string, s).is_err(), "Should fail parsing string: {}", s);
        }
    }

    // Test parsing valid JSON numbers
    #[test]
    fn test_valid_numbers() {
        let valid_numbers = vec![
            r#"0"#,
            r#"-42"#,
            r#"3.14"#,
            r#"2.5e+3"#,
            r#"-0.001"#,
            r#"6.022e-23"#,
        ];

        for num in valid_numbers {
            assert!(JsonParser::parse(Rule::number, num).is_ok(), "Failed to parse number: {}", num);
        }
    }

    // Test parsing valid and invalid complete JSON documents
    #[test]
    fn test_complete_json_documents() {
        let valid_docs = vec![
            r#"{"key": "value"}"#,
            r#"[1, 2, 3]"#,
            r#"{"nested": {"array": [1, 2, 3]}}"#,
        ];

        let invalid_docs = vec![
            r#"{"key": }"#,
            r#"[1, 2,"#,
            r#"invalid"#,
        ];

        for doc in valid_docs {
            assert!(JsonParser::parse(Rule::json, doc).is_ok(), "Failed to parse valid document: {}", doc);
        }

        for doc in invalid_docs {
            assert!(JsonParser::parse(Rule::json, doc).is_err(), "Should fail parsing invalid document: {}", doc);
        }
    }
}