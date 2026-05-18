//! Expression Parser
//!
//! Core expression parsing logic for mathematical notation.

use super::category_parser::CategoryTheoryParser;
use super::types::*;
use super::unicode_parser::UnicodeParser;
use crate::error::{AispError, AispResult};
use std::iter::Peekable;
use std::str::Chars;

/// Main expression parser
pub struct ExpressionParser {
    /// Unicode symbol parser
    unicode_parser: UnicodeParser,
    /// Category theory parser
    category_parser: CategoryTheoryParser,
    /// Parsing configuration
    config: MathParsingConfig,
}

impl ExpressionParser {
    /// Create new expression parser
    pub fn new() -> Self {
        Self {
            unicode_parser: UnicodeParser::new(),
            category_parser: CategoryTheoryParser::new(),
            config: MathParsingConfig::default(),
        }
    }

    /// Create expression parser with custom configuration
    pub fn with_config(config: MathParsingConfig) -> Self {
        Self {
            unicode_parser: UnicodeParser::with_config(config.create_unicode_config()),
            category_parser: CategoryTheoryParser::new(),
            config,
        }
    }

    /// Parse mathematical expression from string
    pub fn parse_mathematical_expression(&self, input: &str) -> AispResult<EnhancedMathExpression> {
        let mut chars = input.chars().peekable();
        let mut context = ParsingContext::new();
        self.parse_expression(&mut chars, &mut context)
    }

    /// Parse expression with context
    pub fn parse_expression(
        &self,
        chars: &mut Peekable<Chars>,
        context: &mut ParsingContext,
    ) -> AispResult<EnhancedMathExpression> {
        if context.depth > self.config.max_parsing_depth {
            context.add_error(MathNotationError::DepthLimitExceeded {
                depth: context.depth,
            });
            return Err(AispError::validation_error(
                "Maximum parsing depth exceeded".to_string(),
            ));
        }

        context.depth += 1;
        self.skip_whitespace(chars, context);

        if chars.peek().is_none() {
            context.depth -= 1;
            return Err(AispError::validation_error(
                "Unexpected end of mathematical expression".to_string(),
            ));
        }

        // Try parsing different types of mathematical expressions
        let result = if let Some(&ch) = chars.peek() {
            match ch {
                // Quantifiers: ∀, ∃, ∃!, λ
                '∀' | '∃' | 'λ' => self.parse_quantified_expression(chars, context),
                // Category theory symbols
                '𝔽' | '𝔾' | '⟨' | '⇒' | '⊣' | '∘' if self.config.enable_category_theory => {
                    self.category_parser
                        .parse_category_theory_construct(chars, context)
                }
                // Greek letters and mathematical symbols
                'α'..='ω' | 'Α'..='Ω' => self.unicode_parser.parse_greek_letter(chars, context),
                // Mathematical operators and logic symbols
                '≜' | '≔' | '≡' | '⇒' | '↔' | '⊢' | '⊨' | '⊕' | '⊖' | '⊗' | '∈' | '∉' | '⊆'
                | '⊊' | '∪' | '∩' | '∅' | '℘' | '∧' | '∨' | '¬' | '→' | '↦' | '≤' | '≥' | '≠'
                | '◊' | '⊘' => self
                    .unicode_parser
                    .parse_mathematical_operator(chars, context),
                // Number sets and mathematical constants
                'ℕ' | 'ℤ' | 'ℚ' | 'ℝ' | 'ℂ' | '𝔸' | '𝔹' | '𝕊' | '𝕃' => self
                    .unicode_parser
                    .parse_mathematical_constant(chars, context),
                // Subscripts and superscripts
                '₀' | '₁' | '₂' | '₃' | '₄' | '₅' | '₆' | '₇' | '₈' | '₉' | '⁰' | '¹' | '²'
                | '³' | '⁴' | '⁵' | '⁶' | '⁷' | '⁸' | '⁹' | '⁺' | '⁻' => {
                    self.unicode_parser.parse_script_symbol(chars, context)
                }
                // Parentheses and brackets
                '(' | '[' | '{' => self.parse_bracketed_expression(chars, context),
                // Unicode mathematical symbols
                _ if self.unicode_parser.is_mathematical_symbol(ch) => {
                    self.unicode_parser.parse_unicode_symbol(chars, context)
                }
                // Regular identifiers and numbers
                _ => self.parse_basic_expression(chars, context),
            }
        } else {
            Err(AispError::validation_error(
                "Empty mathematical expression".to_string(),
            ))
        };

        context.depth -= 1;
        result
    }

    /// Parse quantified expressions (∀, ∃, λ)
    fn parse_quantified_expression(
        &self,
        chars: &mut Peekable<Chars>,
        context: &mut ParsingContext,
    ) -> AispResult<EnhancedMathExpression> {
        let quantifier_char = chars.next().unwrap();
        context.position += 1;

        let quantifier = match quantifier_char {
            '∀' => Quantifier::Forall,
            '∃' => {
                // Check for unique existence ∃!
                if chars.peek() == Some(&'!') {
                    chars.next();
                    context.position += 1;
                    Quantifier::ExistsUnique
                } else {
                    Quantifier::Exists
                }
            }
            'λ' => {
                if !self.config.enable_lambda_calculus {
                    context.add_error(MathNotationError::InvalidExpression {
                        expression: "lambda expression".to_string(),
                        reason: "Lambda calculus parsing disabled".to_string(),
                    });
                    return Ok(EnhancedMathExpression::BasicSymbol(
                        quantifier_char.to_string(),
                    ));
                }
                Quantifier::Lambda
            }
            _ => {
                context.add_error(MathNotationError::InvalidExpression {
                    expression: format!("quantifier: {}", quantifier_char),
                    reason: "Invalid quantifier symbol".to_string(),
                });
                return Err(AispError::validation_error(format!(
                    "Invalid quantifier: {}",
                    quantifier_char
                )));
            }
        };

        self.skip_whitespace(chars, context);

        // Enter quantifier scope
        context.enter_scope(ScopeType::Quantifier);

        // Parse variable
        let variable = self.parse_identifier(chars, context)?;

        // Parse domain (if present)
        let domain = if chars.peek() == Some(&'∈') || chars.peek() == Some(&':') {
            chars.next(); // consume ∈ or :
            context.position += 1;
            self.skip_whitespace(chars, context);
            self.parse_identifier(chars, context)?
        } else {
            String::new()
        };

        // Bind variable in current scope
        let binding = VariableBinding::new(
            variable.clone(),
            Some(domain.clone()),
            format!("{:?}", quantifier),
            context.position,
        );
        context.bind_variable(variable.clone(), binding);

        // Parse body
        self.skip_whitespace(chars, context);
        if chars.peek() == Some(&':') || chars.peek() == Some(&'.') {
            chars.next(); // consume separator
            context.position += 1;
            self.skip_whitespace(chars, context);
        }

        let body = Box::new(self.parse_expression(chars, context)?);

        // Exit quantifier scope
        context.exit_scope();

        Ok(EnhancedMathExpression::Quantified {
            quantifier,
            variable,
            domain,
            body,
        })
    }

    /// Parse bracketed expression: (expr), [expr], {expr}
    fn parse_bracketed_expression(
        &self,
        chars: &mut Peekable<Chars>,
        context: &mut ParsingContext,
    ) -> AispResult<EnhancedMathExpression> {
        let open_bracket = chars.next().unwrap();
        context.position += 1;

        let close_bracket = match open_bracket {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            _ => {
                context.add_error(MathNotationError::InvalidExpression {
                    expression: "bracketed expression".to_string(),
                    reason: format!("Invalid opening bracket: {}", open_bracket),
                });
                return Ok(EnhancedMathExpression::BasicSymbol(
                    open_bracket.to_string(),
                ));
            }
        };

        self.skip_whitespace(chars, context);

        // Handle empty brackets
        if chars.peek() == Some(&close_bracket) {
            chars.next();
            context.position += 1;
            return Ok(EnhancedMathExpression::ComplexStructure {
                structure_type: "empty_brackets".to_string(),
                components: Vec::new(),
            });
        }

        // Parse inner expression
        let inner_expr = self.parse_expression(chars, context)?;

        self.skip_whitespace(chars, context);

        // Expect closing bracket
        if chars.peek() == Some(&close_bracket) {
            chars.next();
            context.position += 1;
            Ok(inner_expr)
        } else {
            context.add_error(MathNotationError::InvalidExpression {
                expression: "bracketed expression".to_string(),
                reason: format!("Expected closing bracket '{}'", close_bracket),
            });
            Ok(inner_expr)
        }
    }

    /// Parse basic expression (identifiers, numbers)
    fn parse_basic_expression(
        &self,
        chars: &mut Peekable<Chars>,
        context: &mut ParsingContext,
    ) -> AispResult<EnhancedMathExpression> {
        let mut expr = String::new();

        // Parse identifier or number
        while let Some(&ch) = chars.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                expr.push(chars.next().unwrap());
                context.position += 1;
            } else {
                break;
            }
        }

        if expr.is_empty() {
            // Single character that doesn't fit other patterns
            if let Some(ch) = chars.next() {
                context.position += 1;
                Ok(EnhancedMathExpression::BasicSymbol(ch.to_string()))
            } else {
                context.add_error(MathNotationError::InvalidExpression {
                    expression: "basic expression".to_string(),
                    reason: "Empty expression".to_string(),
                });
                Err(AispError::validation_error("Empty expression".to_string()))
            }
        } else {
            // Check if it's a bound variable
            if let Some(_binding) = context.lookup_variable(&expr) {
                Ok(EnhancedMathExpression::BasicSymbol(expr))
            } else {
                Ok(EnhancedMathExpression::BasicSymbol(expr))
            }
        }
    }

    /// Parse identifier
    fn parse_identifier(
        &self,
        chars: &mut Peekable<Chars>,
        context: &mut ParsingContext,
    ) -> AispResult<String> {
        let mut identifier = String::new();

        while let Some(&ch) = chars.peek() {
            if ch.is_alphanumeric() || ch == '_' || self.is_mathematical_letter(ch) {
                identifier.push(chars.next().unwrap());
                context.position += 1;
            } else {
                break;
            }
        }

        if identifier.is_empty() {
            context.add_error(MathNotationError::InvalidExpression {
                expression: "identifier".to_string(),
                reason: "Empty identifier".to_string(),
            });
            Ok("_".to_string())
        } else {
            Ok(identifier)
        }
    }

    /// Check if character is a mathematical letter
    fn is_mathematical_letter(&self, ch: char) -> bool {
        matches!(ch as u32,
            0x1D400..=0x1D7FF | // Mathematical Alphanumeric Symbols
            0x2102..=0x2138     // Letterlike Symbols
        )
    }

    /// Skip whitespace characters
    fn skip_whitespace(&self, chars: &mut Peekable<Chars>, context: &mut ParsingContext) {
        while let Some(&ch) = chars.peek() {
            if ch.is_whitespace() {
                chars.next();
                context.position += 1;
            } else {
                break;
            }
        }
    }

    /// Get parsing configuration
    pub fn config(&self) -> &MathParsingConfig {
        &self.config
    }

    /// Update parsing configuration
    pub fn set_config(&mut self, config: MathParsingConfig) {
        self.config = config.clone();
        self.unicode_parser = UnicodeParser::with_config(config.create_unicode_config());
    }
}

impl MathParsingConfig {
    /// Create Unicode parsing configuration from math parsing config
    pub fn create_unicode_config(&self) -> super::unicode_parser::UnicodeParsingConfig {
        super::unicode_parser::UnicodeParsingConfig {
            enable_advanced_symbols: self.enable_advanced_unicode,
            enable_letterlike: self.enable_advanced_unicode,
            enable_arrows: self.enable_category_theory,
            enable_geometric: self.enable_advanced_unicode,
            strict_validation: self.strict_mode,
        }
    }
}

impl Default for ExpressionParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression_parser_creation() {
        let parser = ExpressionParser::new();
        assert!(parser.config.enable_category_theory);
        assert!(parser.config.enable_advanced_unicode);
        assert!(parser.config.enable_lambda_calculus);
    }

    #[test]
    fn test_basic_expression_parsing() {
        let parser = ExpressionParser::new();
        let result = parser.parse_mathematical_expression("x");
        assert!(result.is_ok());

        let expr = result.unwrap();
        assert!(matches!(expr, EnhancedMathExpression::BasicSymbol(_)));
    }

    #[test]
    fn test_quantified_expression_parsing() {
        let parser = ExpressionParser::new();
        let result = parser.parse_mathematical_expression("∀x: P(x)");
        assert!(result.is_ok());

        let expr = result.unwrap();
        assert!(matches!(expr, EnhancedMathExpression::Quantified { .. }));
    }

    #[test]
    fn test_lambda_expression_parsing() {
        let parser = ExpressionParser::new();
        let result = parser.parse_mathematical_expression("λx.x");
        assert!(result.is_ok());

        let expr = result.unwrap();
        assert!(matches!(expr, EnhancedMathExpression::Quantified { .. }));
    }

    #[test]
    fn test_bracketed_expression_parsing() {
        let parser = ExpressionParser::new();
        let result = parser.parse_mathematical_expression("(x + y)");
        assert!(result.is_ok());
    }

    #[test]
    fn test_mathematical_constant_parsing() {
        let parser = ExpressionParser::new();
        let result = parser.parse_mathematical_expression("ℝ");
        assert!(result.is_ok());

        let expr = result.unwrap();
        assert!(matches!(expr, EnhancedMathExpression::Constant { .. }));
    }

    #[test]
    fn test_unique_existence_parsing() {
        let parser = ExpressionParser::new();
        let result = parser.parse_mathematical_expression("∃!x: P(x)");
        assert!(result.is_ok());

        let expr = result.unwrap();
        if let EnhancedMathExpression::Quantified { quantifier, .. } = expr {
            assert_eq!(quantifier, Quantifier::ExistsUnique);
        } else {
            panic!("Expected quantified expression");
        }
    }

    #[test]
    fn test_depth_limit() {
        let mut config = MathParsingConfig::default();
        config.max_parsing_depth = 1;
        let parser = ExpressionParser::with_config(config);

        // This should fail due to depth limit
        let result = parser.parse_mathematical_expression("∀x: ∀y: P(x, y)");
        assert!(result.is_err());
    }

    #[test]
    fn test_lambda_calculus_disabled() {
        let mut config = MathParsingConfig::default();
        config.enable_lambda_calculus = false;
        let parser = ExpressionParser::with_config(config);

        let result = parser.parse_mathematical_expression("λx.x");
        assert!(result.is_ok());

        // Should parse as basic symbol when lambda calculus is disabled
        let expr = result.unwrap();
        assert!(matches!(expr, EnhancedMathExpression::BasicSymbol(_)));
    }

    #[test]
    fn test_empty_brackets() {
        let parser = ExpressionParser::new();
        let result = parser.parse_mathematical_expression("()");
        assert!(result.is_ok());

        let expr = result.unwrap();
        assert!(matches!(
            expr,
            EnhancedMathExpression::ComplexStructure { .. }
        ));
    }

    #[test]
    fn test_mathematical_letter_detection() {
        let parser = ExpressionParser::new();

        assert!(parser.is_mathematical_letter('𝔸')); // Mathematical bold A
        assert!(parser.is_mathematical_letter('ℂ')); // Complex numbers
        assert!(!parser.is_mathematical_letter('a')); // Regular letter
    }

    #[test]
    fn test_config_update() {
        let mut parser = ExpressionParser::new();
        let mut new_config = MathParsingConfig::default();
        new_config.enable_category_theory = false;

        parser.set_config(new_config);
        assert!(!parser.config().enable_category_theory);
    }

    #[test]
    fn test_identifier_parsing() {
        let parser = ExpressionParser::new();
        let mut context = ParsingContext::new();
        let mut chars = "variable123".chars().peekable();

        let result = parser.parse_identifier(&mut chars, &mut context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "variable123");
    }

    #[test]
    fn test_whitespace_handling() {
        let parser = ExpressionParser::new();
        let result = parser.parse_mathematical_expression("  x  ");
        assert!(result.is_ok());

        let expr = result.unwrap();
        assert!(matches!(expr, EnhancedMathExpression::BasicSymbol(_)));
    }
}
