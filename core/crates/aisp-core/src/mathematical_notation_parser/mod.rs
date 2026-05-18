//! Enhanced Mathematical Notation Parser for AISP 5.1
//!
//! This module provides sophisticated parsing capabilities for complex Unicode
//! mathematical symbols and expressions found in the AISP 5.1 reference
//! specification, including category theory notation, advanced mathematical
//! operators, and complex mathematical formulas.
//!
//! ## Module Organization
//!
//! The parser is organized into focused modules:
//! - `types`: Core type definitions and configuration
//! - `unicode_parser`: Unicode mathematical symbol parsing
//! - `category_parser`: Category theory construct parsing
//! - `expression_parser`: Main expression parsing logic
//!
//! ## Usage
//!
//! ```rust
//! use aisp_core::mathematical_notation_parser::MathematicalNotationParser;
//!
//! let parser = MathematicalNotationParser::new();
//! let result = parser.parse_mathematical_expression("∀x ∈ ℝ: P(x)");
//! ```

pub mod category_parser;
pub mod expression_parser;
pub mod types;
pub mod unicode_parser;

pub use category_parser::CategoryTheoryParser;
pub use expression_parser::ExpressionParser;
pub use types::*;
pub use unicode_parser::UnicodeParser;

use crate::error::AispResult;

/// Main mathematical notation parser facade
pub struct MathematicalNotationParser {
    expression_parser: ExpressionParser,
}

impl MathematicalNotationParser {
    /// Create new mathematical notation parser
    pub fn new() -> Self {
        Self {
            expression_parser: ExpressionParser::new(),
        }
    }

    /// Create parser with custom configuration
    pub fn with_config(config: MathParsingConfig) -> Self {
        Self {
            expression_parser: ExpressionParser::with_config(config),
        }
    }

    /// Parse mathematical expression from string
    pub fn parse_mathematical_expression(&self, input: &str) -> AispResult<EnhancedMathExpression> {
        self.expression_parser.parse_mathematical_expression(input)
    }

    /// Get parsing configuration
    pub fn config(&self) -> &MathParsingConfig {
        self.expression_parser.config()
    }

    /// Update parsing configuration
    pub fn set_config(&mut self, config: MathParsingConfig) {
        self.expression_parser.set_config(config);
    }

    /// Parse multiple expressions
    pub fn parse_expressions(&self, inputs: &[&str]) -> Vec<AispResult<EnhancedMathExpression>> {
        inputs
            .iter()
            .map(|input| self.parse_mathematical_expression(input))
            .collect()
    }

    /// Validate mathematical expression syntax
    pub fn validate_expression(&self, input: &str) -> AispResult<()> {
        self.parse_mathematical_expression(input).map(|_| ())
    }
}

impl Default for MathematicalNotationParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_creation() {
        let parser = MathematicalNotationParser::new();
        assert!(parser.config().enable_category_theory);
    }

    #[test]
    fn test_parser_with_config() {
        let config = MathParsingConfig::strict();
        let parser = MathematicalNotationParser::with_config(config);
        assert!(parser.config().strict_mode);
    }

    #[test]
    fn test_basic_expression_parsing() {
        let parser = MathematicalNotationParser::new();
        let result = parser.parse_mathematical_expression("x + y");
        assert!(result.is_ok());
    }

    #[test]
    fn test_mathematical_constants() {
        let parser = MathematicalNotationParser::new();

        let expressions = ["ℕ", "ℤ", "ℚ", "ℝ", "ℂ"];
        for expr in &expressions {
            let result = parser.parse_mathematical_expression(expr);
            assert!(result.is_ok(), "Failed to parse: {}", expr);
        }
    }

    #[test]
    fn test_quantified_expressions() {
        let parser = MathematicalNotationParser::new();

        let expressions = ["∀x: P(x)", "∃y: Q(y)", "∃!z: R(z)", "λx.x"];

        for expr in &expressions {
            let result = parser.parse_mathematical_expression(expr);
            assert!(result.is_ok(), "Failed to parse: {}", expr);
        }
    }

    #[test]
    fn test_category_theory_expressions() {
        let parser = MathematicalNotationParser::new();

        let expressions = ["∘", "⇒", "⊣", "⟨T,η,μ⟩"];

        for expr in &expressions {
            let result = parser.parse_mathematical_expression(expr);
            assert!(result.is_ok(), "Failed to parse: {}", expr);
        }
    }

    #[test]
    fn test_unicode_mathematical_operators() {
        let parser = MathematicalNotationParser::new();

        let expressions = [
            "∧", "∨", "¬", "∈", "∉", "⊆", "∪", "∩", "≤", "≥", "≠", "≡", "→", "↦", "⇒", "↔",
        ];

        for expr in &expressions {
            let result = parser.parse_mathematical_expression(expr);
            assert!(result.is_ok(), "Failed to parse: {}", expr);
        }
    }

    #[test]
    fn test_greek_letters() {
        let parser = MathematicalNotationParser::new();

        let expressions = ["α", "β", "γ", "π", "Ω"];
        for expr in &expressions {
            let result = parser.parse_mathematical_expression(expr);
            assert!(result.is_ok(), "Failed to parse: {}", expr);
        }
    }

    #[test]
    fn test_script_notation() {
        let parser = MathematicalNotationParser::new();

        let expressions = ["₁", "₂", "²", "³", "⁺", "⁻"];
        for expr in &expressions {
            let result = parser.parse_mathematical_expression(expr);
            assert!(result.is_ok(), "Failed to parse: {}", expr);
        }
    }

    #[test]
    fn test_parse_multiple_expressions() {
        let parser = MathematicalNotationParser::new();

        let expressions = vec!["x", "∀y: P(y)", "ℝ", "∘"];
        let results = parser.parse_expressions(&expressions);

        assert_eq!(results.len(), 4);
        for result in results {
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_expression_validation() {
        let parser = MathematicalNotationParser::new();

        // Valid expressions
        assert!(parser.validate_expression("∀x: P(x)").is_ok());
        assert!(parser.validate_expression("ℝ").is_ok());

        // Test with empty string (should fail)
        assert!(parser.validate_expression("").is_err());
    }

    #[test]
    fn test_config_update() {
        let mut parser = MathematicalNotationParser::new();

        let mut new_config = MathParsingConfig::default();
        new_config.enable_lambda_calculus = false;

        parser.set_config(new_config);
        assert!(!parser.config().enable_lambda_calculus);
    }

    #[test]
    fn test_complex_expressions() {
        let parser = MathematicalNotationParser::new();

        let complex_expressions = [
            "∀x ∈ ℝ: ∃y ∈ ℝ: x + y = 0",
            "λf. λx. f(x)",
            "⟨Objects, Morphisms, ∘, id⟩",
        ];

        for expr in &complex_expressions {
            let result = parser.parse_mathematical_expression(expr);
            assert!(
                result.is_ok(),
                "Failed to parse complex expression: {}",
                expr
            );
        }
    }

    #[test]
    fn test_bracketed_expressions() {
        let parser = MathematicalNotationParser::new();

        let expressions = ["(x)", "[y]", "{z}", "()"];
        for expr in &expressions {
            let result = parser.parse_mathematical_expression(expr);
            assert!(
                result.is_ok(),
                "Failed to parse bracketed expression: {}",
                expr
            );
        }
    }
}
