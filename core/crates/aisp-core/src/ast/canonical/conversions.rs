//! Canonical Conversion Operations
//!
//! Conversion traits and utility functions for canonical AST operations
//! following SRP architecture. Since the parser now uses canonical AST types
//! directly, this module focuses on utility operations and type conversions.

use super::document::*;
use super::types::*;
use std::collections::HashMap;

/// Conversion trait for future extensibility
pub trait IntoCanonical<T> {
    fn into_canonical(self) -> T;
}

/// Implementation for CanonicalAispDocument (identity conversion)
impl IntoCanonical<CanonicalAispDocument> for CanonicalAispDocument {
    fn into_canonical(self) -> CanonicalAispDocument {
        self
    }
}

// Note: The robust parser now uses canonical AST types directly,
// so no conversion from parser types is needed.

/// Convert canonical types to legacy types for backward compatibility
pub trait FromCanonical<T> {
    fn from_canonical(canonical: T) -> Self;
}

/// Utility function to extract specific metadata from canonical document
pub fn extract_domain_metadata(doc: &CanonicalAispDocument) -> Option<String> {
    doc.metadata.domain.clone()
}

/// Utility function to extract protocol metadata from canonical document
pub fn extract_protocol_metadata(doc: &CanonicalAispDocument) -> Option<String> {
    doc.metadata.protocol.clone()
}

/// Utility function to extract meta entries from all meta blocks
pub fn extract_all_meta_entries(doc: &CanonicalAispDocument) -> HashMap<String, MetaValue> {
    let mut all_entries = HashMap::new();

    for meta_block in doc.get_meta_blocks() {
        for (key, entry) in &meta_block.entries {
            all_entries.insert(key.clone(), entry.value.clone());
        }
    }

    all_entries
}

/// Utility function to extract all type definitions
pub fn extract_all_type_definitions(
    doc: &CanonicalAispDocument,
) -> HashMap<String, TypeExpression> {
    let mut all_definitions = HashMap::new();

    for types_block in doc.get_types_blocks() {
        for (name, type_def) in &types_block.definitions {
            all_definitions.insert(name.clone(), type_def.type_expr.clone());
        }
    }

    all_definitions
}

/// Utility function to extract all logical rules
pub fn extract_all_logical_rules(doc: &CanonicalAispDocument) -> Vec<LogicalRule> {
    let mut all_rules = Vec::new();

    for rules_block in doc.get_rules_blocks() {
        all_rules.extend(rules_block.rules.clone());
    }

    all_rules
}

/// Utility function to extract all function definitions
pub fn extract_all_function_definitions(doc: &CanonicalAispDocument) -> Vec<FunctionDefinition> {
    let mut all_functions = Vec::new();

    for functions_block in doc.get_functions_blocks() {
        all_functions.extend(functions_block.functions.clone());
    }

    all_functions
}

/// Utility function to extract all evidence metrics
pub fn extract_all_evidence_metrics(doc: &CanonicalAispDocument) -> HashMap<String, f64> {
    let mut all_metrics = HashMap::new();

    for evidence_block in doc.get_evidence_blocks() {
        all_metrics.extend(evidence_block.metrics.clone());
    }

    all_metrics
}

// ConversionBuilder removed since parser uses canonical AST types directly

/// Conversion errors
#[derive(Debug, Clone)]
pub enum ConversionError {
    ValidationFailed(Vec<String>),
    ParseError(String),
    UnsupportedType(String),
}

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConversionError::ValidationFailed(errors) => {
                write!(f, "Validation failed: {}", errors.join(", "))
            }
            ConversionError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ConversionError::UnsupportedType(typ) => write!(f, "Unsupported type: {}", typ),
        }
    }
}

impl std::error::Error for ConversionError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::canonical::{CanonicalAispBlock, MetaBlock};

    #[test]
    fn test_meta_extraction_utilities() {
        let mut doc = CanonicalAispDocument::default();

        let mut entries = HashMap::new();
        entries.insert(
            "test_key".to_string(),
            MetaEntry {
                key: "test_key".to_string(),
                value: MetaValue::String("test_value".to_string()),
                span: None,
            },
        );

        doc.add_block(CanonicalAispBlock::Meta(MetaBlock {
            entries,
            raw_entries: Vec::new(),
            span: None,
        }));

        let extracted = extract_all_meta_entries(&doc);
        assert_eq!(extracted.len(), 1);
        assert_eq!(
            extracted.get("test_key"),
            Some(&MetaValue::String("test_value".to_string()))
        );
    }

    #[test]
    fn test_domain_protocol_extraction() {
        let mut doc = CanonicalAispDocument::default();
        doc.set_domain("mathematics".to_string());
        doc.set_protocol("aisp".to_string());

        assert_eq!(
            extract_domain_metadata(&doc),
            Some("mathematics".to_string())
        );
        assert_eq!(extract_protocol_metadata(&doc), Some("aisp".to_string()));
    }

    #[test]
    fn test_conversion_error_display() {
        let error =
            ConversionError::ValidationFailed(vec!["Error 1".to_string(), "Error 2".to_string()]);
        let display = format!("{}", error);
        assert!(display.contains("Validation failed"));
        assert!(display.contains("Error 1"));
        assert!(display.contains("Error 2"));
    }

    #[test]
    fn test_conversion_trait_exists() {
        // Test that the conversion trait is available for future use
        // This is a placeholder test for the trait infrastructure
    }
}
