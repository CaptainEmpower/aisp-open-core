//! Logic Content Parser
//!
//! Focused parser for logical expressions and rules following SRP.
//! Handles parsing of logical formulas, quantifiers, and rule definitions.

use crate::ast::canonical::{
    BinaryOperator, ConstantValue, LogicalExpression, LogicalRule, Quantifier, QuantifierKind,
    TemporalOperator, UnaryOperator,
};
use crate::error::{AispError, AispResult};

/// SRP-focused parser for logical expression content
pub struct LogicContentParser;

impl LogicContentParser {
    /// Parse logical rule from text
    pub fn parse_logical_rule(rule_text: &str) -> LogicalRule {
        let rule_text = rule_text.trim();

        // Check for quantifiers at the beginning
        let (quantifier, expression_text) = Self::extract_quantifier(rule_text);

        // Parse the main logical expression
        let expression = Self::parse_logical_expression(expression_text);

        LogicalRule {
            quantifier,
            expression,
            raw_text: rule_text.to_string(),
            span: None,
        }
    }

    /// Extract quantifier from the beginning of rule text
    fn extract_quantifier(text: &str) -> (Option<Quantifier>, &str) {
        let text = text.trim();

        // Universal quantifier: ∀x ∈ Domain: expression
        if text.starts_with('∀') {
            return Self::parse_universal_quantifier(text);
        }

        // Existential quantifier: ∃x ∈ Domain: expression
        if text.starts_with('∃') {
            return Self::parse_existential_quantifier(text);
        }

        // ASCII quantifiers
        if text.starts_with("forall") {
            return Self::parse_ascii_universal(text);
        }

        if text.starts_with("exists") {
            return Self::parse_ascii_existential(text);
        }

        // No quantifier found
        (None, text)
    }

    /// Parse universal quantifier (∀x ∈ Domain: ...)
    fn parse_universal_quantifier(text: &str) -> (Option<Quantifier>, &str) {
        // Find the colon that separates quantifier from expression
        if let Some(colon_pos) = text.find(':') {
            // Skip the ∀ character properly using char boundaries
            let quantifier_part = if text.starts_with('∀') {
                &text[text.char_indices().nth(1).map(|(i, _)| i).unwrap_or(1)..colon_pos]
            } else {
                &text[1..colon_pos]
            }
            .trim();
            let expression_part = &text[colon_pos + 1..].trim();

            let (variable, domain) = Self::parse_quantifier_binding(quantifier_part);

            let quantifier = Quantifier {
                kind: QuantifierKind::Universal,
                variable,
                domain,
                span: None,
            };

            (Some(quantifier), expression_part)
        } else {
            // Malformed quantifier, treat as raw expression
            (None, text)
        }
    }

    /// Parse existential quantifier (∃x ∈ Domain: ...)
    fn parse_existential_quantifier(text: &str) -> (Option<Quantifier>, &str) {
        if let Some(colon_pos) = text.find(':') {
            // Skip the ∃ character properly using char boundaries
            let quantifier_part = if text.starts_with('∃') {
                &text[text.char_indices().nth(1).map(|(i, _)| i).unwrap_or(1)..colon_pos]
            } else {
                &text[1..colon_pos]
            }
            .trim();
            let expression_part = &text[colon_pos + 1..].trim();

            let (variable, domain) = Self::parse_quantifier_binding(quantifier_part);

            let quantifier = Quantifier {
                kind: QuantifierKind::Existential,
                variable,
                domain,
                span: None,
            };

            (Some(quantifier), expression_part)
        } else {
            (None, text)
        }
    }

    /// Parse ASCII universal quantifier (forall x in Domain: ...)
    fn parse_ascii_universal(text: &str) -> (Option<Quantifier>, &str) {
        if let Some(colon_pos) = text.find(':') {
            let quantifier_part = &text[6..colon_pos].trim(); // Skip "forall"
            let expression_part = &text[colon_pos + 1..].trim();

            let (variable, domain) = Self::parse_ascii_quantifier_binding(quantifier_part);

            let quantifier = Quantifier {
                kind: QuantifierKind::Universal,
                variable,
                domain,
                span: None,
            };

            (Some(quantifier), expression_part)
        } else {
            (None, text)
        }
    }

    /// Parse ASCII existential quantifier (exists x in Domain: ...)
    fn parse_ascii_existential(text: &str) -> (Option<Quantifier>, &str) {
        if let Some(colon_pos) = text.find(':') {
            let quantifier_part = &text[6..colon_pos].trim(); // Skip "exists"
            let expression_part = &text[colon_pos + 1..].trim();

            let (variable, domain) = Self::parse_ascii_quantifier_binding(quantifier_part);

            let quantifier = Quantifier {
                kind: QuantifierKind::Existential,
                variable,
                domain,
                span: None,
            };

            (Some(quantifier), expression_part)
        } else {
            (None, text)
        }
    }

    /// Parse quantifier binding (x ∈ Domain)
    fn parse_quantifier_binding(binding_text: &str) -> (String, Option<String>) {
        if let Some(in_pos) = binding_text.find('∈') {
            let variable = binding_text[..in_pos].trim().to_string();
            let domain = binding_text[in_pos + '∈'.len_utf8()..].trim().to_string();
            (variable, Some(domain))
        } else {
            // No domain specified, just variable
            (binding_text.trim().to_string(), None)
        }
    }

    /// Parse ASCII quantifier binding (x in Domain)
    fn parse_ascii_quantifier_binding(binding_text: &str) -> (String, Option<String>) {
        if let Some(in_pos) = binding_text.find(" in ") {
            let variable = binding_text[..in_pos].trim().to_string();
            let domain = binding_text[in_pos + 4..].trim().to_string(); // Skip " in "
            (variable, Some(domain))
        } else {
            (binding_text.trim().to_string(), None)
        }
    }

    /// Parse logical expression
    pub fn parse_logical_expression(text: &str) -> LogicalExpression {
        let text = text.trim();

        // Check for binary operators (in order of precedence)
        if let Some(expr) = Self::try_parse_binary_expression(text) {
            return expr;
        }

        // Check for unary operators
        if let Some(expr) = Self::try_parse_unary_expression(text) {
            return expr;
        }

        // Check for temporal operators
        if let Some(expr) = Self::try_parse_temporal_expression(text) {
            return expr;
        }

        // Check for function application
        if let Some(expr) = Self::try_parse_application(text) {
            return expr;
        }

        // Check for membership
        if let Some(expr) = Self::try_parse_membership(text) {
            return expr;
        }

        // Check for constants
        if let Some(expr) = Self::try_parse_constant(text) {
            return expr;
        }

        // Check for parenthesized expressions
        if text.starts_with('(') && text.ends_with(')') {
            let inner = &text[1..text.len() - 1];
            return Self::parse_logical_expression(inner);
        }

        // Default to variable or raw expression
        if Self::is_valid_identifier(text) {
            LogicalExpression::Variable(text.to_string())
        } else {
            LogicalExpression::Raw(text.to_string())
        }
    }

    /// Try to parse binary expression
    fn try_parse_binary_expression(text: &str) -> Option<LogicalExpression> {
        // Order by precedence (lowest to highest)
        let operators = [
            ("⇔", BinaryOperator::Biconditional),
            ("↔", BinaryOperator::Biconditional),
            ("⇒", BinaryOperator::Implication),
            ("→", BinaryOperator::Implication),
            ("=>", BinaryOperator::Implication),
            ("∨", BinaryOperator::Or),
            ("OR", BinaryOperator::Or),
            ("||", BinaryOperator::Or),
            ("∧", BinaryOperator::And),
            ("AND", BinaryOperator::And),
            ("&&", BinaryOperator::And),
            ("≡", BinaryOperator::Equivalence),
            ("==", BinaryOperator::Equivalence),
            ("≜", BinaryOperator::Definition),
            (":=", BinaryOperator::Assignment),
            ("≠", BinaryOperator::NotEquals),
            ("!=", BinaryOperator::NotEquals),
            ("≤", BinaryOperator::LessEqual),
            ("<=", BinaryOperator::LessEqual),
            ("≥", BinaryOperator::GreaterEqual),
            (">=", BinaryOperator::GreaterEqual),
            ("<", BinaryOperator::LessThan),
            (">", BinaryOperator::GreaterThan),
            ("=", BinaryOperator::Equals),
            ("∪", BinaryOperator::Union),
            ("∩", BinaryOperator::Intersection),
            ("⊕", BinaryOperator::Xor),
            ("XOR", BinaryOperator::Xor),
        ];

        for (op_str, op) in &operators {
            if let Some(op_pos) = Self::find_main_operator(text, op_str) {
                let left_text = &text[..op_pos].trim();
                let right_text = &text[op_pos + op_str.len()..].trim();

                let left = Box::new(Self::parse_logical_expression(left_text));
                let right = Box::new(Self::parse_logical_expression(right_text));

                return Some(LogicalExpression::Binary {
                    op: op.clone(),
                    left,
                    right,
                });
            }
        }

        None
    }

    /// Try to parse unary expression
    fn try_parse_unary_expression(text: &str) -> Option<LogicalExpression> {
        if text.starts_with('¬') || text.starts_with("NOT") {
            let operand_text = if text.starts_with('¬') {
                // Skip the ¬ character properly using char boundaries
                &text[text.char_indices().nth(1).map(|(i, _)| i).unwrap_or(1)..].trim()
            } else {
                &text[3..].trim() // Skip "NOT"
            };

            let operand = Box::new(Self::parse_logical_expression(operand_text));
            return Some(LogicalExpression::Unary {
                op: UnaryOperator::Not,
                operand,
            });
        }

        if text.starts_with('𝒫') || text.starts_with("PowerSet") {
            let operand_text = if text.starts_with('𝒫') {
                &text[1..].trim()
            } else {
                &text[8..].trim() // Skip "PowerSet"
            };

            let operand = Box::new(Self::parse_logical_expression(operand_text));
            return Some(LogicalExpression::Unary {
                op: UnaryOperator::PowerSet,
                operand,
            });
        }

        None
    }

    /// Try to parse temporal expression
    fn try_parse_temporal_expression(text: &str) -> Option<LogicalExpression> {
        let temporal_ops = [
            ("□", TemporalOperator::Always),
            ("G", TemporalOperator::Always),
            ("◊", TemporalOperator::Eventually),
            ("F", TemporalOperator::Eventually),
            ("X", TemporalOperator::Next),
            ("U", TemporalOperator::Until),
            ("W", TemporalOperator::WeakUntil),
            ("R", TemporalOperator::Release),
        ];

        for (op_str, op) in &temporal_ops {
            if text.starts_with(op_str) {
                let operand_text = &text[op_str.len()..].trim();
                let operand = Box::new(Self::parse_logical_expression(operand_text));

                return Some(LogicalExpression::Temporal {
                    op: op.clone(),
                    operand,
                });
            }
        }

        None
    }

    /// Try to parse function application
    fn try_parse_application(text: &str) -> Option<LogicalExpression> {
        if let Some(paren_pos) = text.find('(') {
            if text.ends_with(')') {
                let function_name = text[..paren_pos].trim();
                let args_text = &text[paren_pos + 1..text.len() - 1];

                if Self::is_valid_identifier(function_name) {
                    let arguments = if args_text.trim().is_empty() {
                        Vec::new()
                    } else {
                        Self::parse_argument_list(args_text)
                    };

                    return Some(LogicalExpression::Application {
                        function: function_name.to_string(),
                        arguments,
                    });
                }
            }
        }

        None
    }

    /// Try to parse set membership
    fn try_parse_membership(text: &str) -> Option<LogicalExpression> {
        if let Some(in_pos) = text.find(" ∈ ") {
            let element_text = &text[..in_pos].trim();
            // Skip " ∈ " properly - it's 5 bytes total (space + 3-byte ∈ + space)
            let set_text = &text[in_pos + 5..].trim();

            let element = Box::new(Self::parse_logical_expression(element_text));
            let set = Box::new(Self::parse_logical_expression(set_text));

            return Some(LogicalExpression::Membership { element, set });
        }

        None
    }

    /// Try to parse constant value
    fn try_parse_constant(text: &str) -> Option<LogicalExpression> {
        // Number
        if let Ok(num) = text.parse::<f64>() {
            return Some(LogicalExpression::Constant(ConstantValue::Number(num)));
        }

        // Boolean
        match text.to_lowercase().as_str() {
            "true" => return Some(LogicalExpression::Constant(ConstantValue::Boolean(true))),
            "false" => return Some(LogicalExpression::Constant(ConstantValue::Boolean(false))),
            _ => {}
        }

        // String (quoted)
        if text.starts_with('"') && text.ends_with('"') && text.len() >= 2 {
            let string_content = text[1..text.len() - 1].to_string();
            return Some(LogicalExpression::Constant(ConstantValue::String(
                string_content,
            )));
        }

        None
    }

    /// Parse argument list for function application
    fn parse_argument_list(args_text: &str) -> Vec<LogicalExpression> {
        if args_text.trim().is_empty() {
            return Vec::new();
        }

        // Simple comma splitting (doesn't handle nested parentheses properly)
        args_text
            .split(',')
            .map(|arg| Self::parse_logical_expression(arg.trim()))
            .collect()
    }

    /// Find main operator position (not inside parentheses)
    fn find_main_operator(text: &str, operator: &str) -> Option<usize> {
        let mut paren_depth = 0;
        let chars: Vec<char> = text.chars().collect();
        let op_chars: Vec<char> = operator.chars().collect();

        for i in 0..chars.len() {
            match chars[i] {
                '(' => paren_depth += 1,
                ')' => paren_depth -= 1,
                _ => {
                    if paren_depth == 0 {
                        // Check if operator starts at position i
                        if i + op_chars.len() <= chars.len() {
                            let slice = &chars[i..i + op_chars.len()];
                            if slice == &op_chars[..] {
                                return Some(i);
                            }
                        }
                    }
                }
            }
        }

        None
    }

    /// Check if string is valid identifier
    fn is_valid_identifier(name: &str) -> bool {
        if name.is_empty() {
            return false;
        }

        let first_char = name.chars().next().unwrap();
        if !first_char.is_alphabetic() && first_char != '_' {
            return false;
        }

        name.chars().all(|c| c.is_alphanumeric() || c == '_')
    }

    /// Validate logical rule
    pub fn validate_logical_rule(rule: &LogicalRule) -> AispResult<()> {
        if rule.raw_text.trim().is_empty() {
            return Err(AispError::validation_error("Logical rule cannot be empty"));
        }

        // Validate quantifier if present
        if let Some(quantifier) = &rule.quantifier {
            Self::validate_quantifier(quantifier)?;
        }

        // Expression validation could be added here

        Ok(())
    }

    /// Validate quantifier
    fn validate_quantifier(quantifier: &Quantifier) -> AispResult<()> {
        if quantifier.variable.is_empty() {
            return Err(AispError::validation_error(
                "Quantifier variable cannot be empty",
            ));
        }

        if !Self::is_valid_identifier(&quantifier.variable) {
            return Err(AispError::validation_error(&format!(
                "Invalid quantifier variable: '{}'",
                quantifier.variable
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_universal_quantifier() {
        let rule = LogicContentParser::parse_logical_rule("∀x ∈ ℕ: x ≥ 0");

        assert!(rule.quantifier.is_some());
        let q = rule.quantifier.unwrap();
        assert!(matches!(q.kind, QuantifierKind::Universal));
        assert_eq!(q.variable, "x");
        assert_eq!(q.domain, Some("ℕ".to_string()));
    }

    #[test]
    fn test_parse_existential_quantifier() {
        let rule = LogicContentParser::parse_logical_rule("∃y ∈ ℝ: y > 0");

        assert!(rule.quantifier.is_some());
        let q = rule.quantifier.unwrap();
        assert!(matches!(q.kind, QuantifierKind::Existential));
        assert_eq!(q.variable, "y");
        assert_eq!(q.domain, Some("ℝ".to_string()));
    }

    #[test]
    fn test_parse_binary_expression() {
        let expr = LogicContentParser::parse_logical_expression("x ∧ y");

        if let LogicalExpression::Binary { op, left, right } = expr {
            assert!(matches!(op, BinaryOperator::And));
            assert!(matches!(*left, LogicalExpression::Variable(_)));
            assert!(matches!(*right, LogicalExpression::Variable(_)));
        } else {
            panic!("Expected binary expression");
        }
    }

    #[test]
    fn test_parse_unary_expression() {
        let expr = LogicContentParser::parse_logical_expression("¬p");

        if let LogicalExpression::Unary { op, operand: _ } = expr {
            assert!(matches!(op, UnaryOperator::Not));
        } else {
            panic!("Expected unary expression");
        }
    }

    #[test]
    fn test_parse_function_application() {
        let expr = LogicContentParser::parse_logical_expression("f(x, y)");

        if let LogicalExpression::Application {
            function,
            arguments,
        } = expr
        {
            assert_eq!(function, "f");
            assert_eq!(arguments.len(), 2);
        } else {
            panic!("Expected function application");
        }
    }

    #[test]
    fn test_parse_membership() {
        let expr = LogicContentParser::parse_logical_expression("x ∈ S");

        if let LogicalExpression::Membership { element: _, set: _ } = expr {
            // Success
        } else {
            panic!("Expected membership expression");
        }
    }

    #[test]
    fn test_parse_constants() {
        let num_expr = LogicContentParser::parse_logical_expression("42");
        assert!(matches!(
            num_expr,
            LogicalExpression::Constant(ConstantValue::Number(_))
        ));

        let bool_expr = LogicContentParser::parse_logical_expression("true");
        assert!(matches!(
            bool_expr,
            LogicalExpression::Constant(ConstantValue::Boolean(true))
        ));

        let str_expr = LogicContentParser::parse_logical_expression("\"hello\"");
        assert!(matches!(
            str_expr,
            LogicalExpression::Constant(ConstantValue::String(_))
        ));
    }

    #[test]
    fn test_ascii_quantifiers() {
        let rule = LogicContentParser::parse_logical_rule("forall x in Natural: x >= 0");
        assert!(rule.quantifier.is_some());

        let q = rule.quantifier.unwrap();
        assert!(matches!(q.kind, QuantifierKind::Universal));
        assert_eq!(q.variable, "x");
        assert_eq!(q.domain, Some("Natural".to_string()));
    }

    #[test]
    fn test_temporal_operators() {
        let expr = LogicContentParser::parse_logical_expression("□p");
        assert!(matches!(expr, LogicalExpression::Temporal { .. }));

        let expr2 = LogicContentParser::parse_logical_expression("◊q");
        assert!(matches!(expr2, LogicalExpression::Temporal { .. }));
    }

    #[test]
    fn test_validate_logical_rule() {
        let valid_rule = LogicalRule {
            quantifier: None,
            expression: LogicalExpression::Variable("x".to_string()),
            raw_text: "x > 0".to_string(),
            span: None,
        };
        assert!(LogicContentParser::validate_logical_rule(&valid_rule).is_ok());

        let invalid_rule = LogicalRule {
            quantifier: None,
            expression: LogicalExpression::Variable("x".to_string()),
            raw_text: "".to_string(),
            span: None,
        };
        assert!(LogicContentParser::validate_logical_rule(&invalid_rule).is_err());
    }
}
