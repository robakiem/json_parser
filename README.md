### **My json parser**

The JSON parser is a tool designed for parsing and processing JSON files. It efficiently handles input data, ensuring reliable and accurate analysis of JSON structures.

### Overview

The parser uses Pest to analyze email addresses based on an extended grammar. It supports various email formats, including those with comments and special characters, and can validate and extract components from email addresses.

### Grammar rules

- Whitespace: The parser allows for space, tab, carriage return, and newline characters as whitespace, which can be ignored during parsing.

- JSON Structure: The entire document must begin and end with either an object or an array.
        
- Object: An object starts with { and ends with }, containing key-value pairs, which may be separated by commas.

- Pair: A key-value pair consists of a string key followed by a colon : and a value.

- Array: An array starts with [ and ends with ], containing values that may be separated by commas.

- Value: A value can be a string, number, object, array, boolean, or null.

- String: A string is enclosed in double quotes, containing characters that may be escaped or represent Unicode sequences.

- Character: String characters include normal characters, escaped characters (like quotes or backslashes), and Unicode escape sequences.

- Number: A number can have an optional negative sign, an integer part, an optional fractional part, and an optional exponent.

- Boolean: The boolean values true and false are recognized.

- Null: The null value is supported.

This grammar defines the structure for parsing a valid JSON document, supporting typical data types such as objects, arrays, strings, numbers, booleans, and null.

### Running the parser

```cargo run <path to json>```
