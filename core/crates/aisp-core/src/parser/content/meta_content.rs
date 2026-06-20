//! Meta Content Parser
//!
//! Focused parser for meta block content following SRP.
//! Handles parsing of key-value pairs, logical constraints, and metadata.

use crate::ast::canonical::{ConstantValue, LogicalExpression, MetaValue};
use crate::error::{AispError, AispResult};
use std::collections::HashMap;

/// SRP-focused parser for meta block content
pub struct MetaContentParser;

impl MetaContentParser {
    /// Parse a single meta entry from "key≜value" format
    pub fn parse_entry(entry_text: &str) -> Option<(String, MetaValue)> {
        if let Some(pos) = entry_text.find('≜') {
            let key = entry_text[..pos].trim().to_string();
            let value_text = entry_text[pos + '≜'.len_utf8()..].trim();

            let value = Self::parse_meta_value(value_text);
            Some((key, value))
        } else {
            None
        }
    }

    /// Parse meta value with type inference
    pub fn parse_meta_value(value_text: &str) -> MetaValue {
        // Remove quotes if present
        let value_text = value_text.trim_matches('"');

        // Try parsing as number first
        if let Ok(num) = value_text.parse::<f64>() {
            return MetaValue::Number(num);
        }

        // Check for boolean values
        match value_text.to_lowercase().as_str() {
            "true" => return MetaValue::Boolean(true),
            "false" => return MetaValue::Boolean(false),
            _ => {}
        }

        // Check for logical expressions (contains logical operators)
        if Self::contains_logical_operators(value_text) {
            return MetaValue::Constraint(Self::parse_simple_logical_expression(value_text));
        }

        // Check for list format [item1, item2, ...]
        if value_text.starts_with('[') && value_text.ends_with(']') {
            let inner = &value_text[1..value_text.len() - 1];
            let items: Vec<MetaValue> = inner
                .split(',')
                .map(|item| Self::parse_meta_value(item.trim()))
                .collect();
            return MetaValue::List(items);
        }

        // Check for map format {key1: value1, key2: value2}
        if value_text.starts_with('{') && value_text.ends_with('}') {
            let mut map = HashMap::new();
            let inner = &value_text[1..value_text.len() - 1];

            for pair in inner.split(',') {
                if let Some(colon_pos) = pair.find(':') {
                    let map_key = pair[..colon_pos].trim().trim_matches('"').to_string();
                    let map_value = Self::parse_meta_value(&pair[colon_pos + 1..]);
                    map.insert(map_key, map_value);
                }
            }
            return MetaValue::Map(map);
        }

        // Default to string
        MetaValue::String(value_text.to_string())
    }

    /// Check if text contains logical operators
    fn contains_logical_operators(text: &str) -> bool {
        text.contains('∀')
            || text.contains('∃')
            || text.contains('⇒')
            || text.contains('∧')
            || text.contains('∨')
            || text.contains('¬')
            || text.contains("=>")
            || text.contains("AND")
            || text.contains("OR")
    }

    /// Parse simple logical expression (basic implementation)
    fn parse_simple_logical_expression(text: &str) -> LogicalExpression {
        // For now, treat complex logical expressions as raw text
        // This can be enhanced with proper logical expression parsing
        LogicalExpression::Raw(text.to_string())
    }

    /// Validate meta key format
    pub fn validate_key(key: &str) -> AispResult<()> {
        if key.is_empty() {
            return Err(AispError::validation_error("Meta key cannot be empty"));
        }

        if !key
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(AispError::validation_error(format!(
                "Invalid meta key format: '{}'. Only alphanumeric, underscore, and dash allowed",
                key
            )));
        }

        Ok(())
    }

    /// Extract all key-value pairs from multi-line meta content
    pub fn extract_entries(content: &str) -> Vec<(String, MetaValue)> {
        let mut entries = Vec::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") || line.starts_with(";;") {
                continue;
            }

            if let Some((key, value)) = Self::parse_entry(line) {
                entries.push((key, value));
            }
        }

        entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string_entry() {
        let result = MetaContentParser::parse_entry("Vision≜\"Test document\"");
        assert_eq!(
            result,
            Some((
                "Vision".to_string(),
                MetaValue::String("Test document".to_string())
            ))
        );
    }

    #[test]
    fn test_parse_number_entry() {
        let result = MetaContentParser::parse_entry("Version≜5.1");
        assert_eq!(
            result,
            Some(("Version".to_string(), MetaValue::Number(5.1)))
        );
    }

    #[test]
    fn test_parse_boolean_entry() {
        let result = MetaContentParser::parse_entry("IsPublic≜true");
        assert_eq!(
            result,
            Some(("IsPublic".to_string(), MetaValue::Boolean(true)))
        );
    }

    #[test]
    fn test_parse_list_entry() {
        let result = MetaContentParser::parse_entry("Tags≜[\"ai\", \"safety\", \"protocol\"]");
        assert!(matches!(result, Some((_, MetaValue::List(_)))));
    }

    #[test]
    fn test_validate_key() {
        assert!(MetaContentParser::validate_key("ValidKey123").is_ok());
        assert!(MetaContentParser::validate_key("key_with_underscores").is_ok());
        assert!(MetaContentParser::validate_key("key-with-dashes").is_ok());
        assert!(MetaContentParser::validate_key("").is_err());
        assert!(MetaContentParser::validate_key("key with spaces").is_err());
        assert!(MetaContentParser::validate_key("key@symbol").is_err());
    }

    #[test]
    fn test_extract_entries() {
        let content = r#"
        Vision≜"AI Safety Protocol"
        Version≜5.1
        IsPublic≜true
        // This is a comment
        Tags≜["ai", "safety"]
        "#;

        let entries = MetaContentParser::extract_entries(content);
        assert_eq!(entries.len(), 4);
        assert_eq!(entries[0].0, "Vision");
        assert_eq!(entries[1].0, "Version");
        assert_eq!(entries[2].0, "IsPublic");
        assert_eq!(entries[3].0, "Tags");
    }
}
