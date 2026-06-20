//! Lambda Expression Content Parser
//!
//! Focused parser for lambda expressions following SRP.
//! Handles parsing of λx.body syntax and function definitions.

use crate::ast::canonical::{FunctionDefinition, LambdaExpression, LogicalExpression};
use crate::error::{AispError, AispResult};

/// SRP-focused parser for lambda expression content
pub struct LambdaContentParser;

impl LambdaContentParser {
    /// Parse a function definition from "name≜λparams.body" format
    pub fn parse_function_definition(func_text: &str) -> Option<(String, LambdaExpression)> {
        if let Some(pos) = func_text.find('≜') {
            let name = func_text[..pos].trim().to_string();
            let body_text = func_text[pos + '≜'.len_utf8()..].trim();

            let lambda = Self::parse_lambda_expression(body_text);
            Some((name, lambda))
        } else {
            None
        }
    }

    /// Parse lambda expression from text
    pub fn parse_lambda_expression(text: &str) -> LambdaExpression {
        let text = text.trim();

        // Handle Unicode lambda: λx.x+1
        if text.starts_with('λ') {
            Self::parse_unicode_lambda(text)
        }
        // Handle ASCII lambda: \x.x+1
        else if text.starts_with('\\') {
            Self::parse_ascii_lambda(text)
        }
        // Handle function keyword: function(x) { x+1 }
        else if text.starts_with("function") {
            Self::parse_function_syntax(text)
        }
        // Default: treat as simple expression
        else {
            LambdaExpression {
                parameters: vec!["x".to_string()],
                body: LogicalExpression::Variable(text.to_string()),
                span: None,
            }
        }
    }

    /// Parse Unicode lambda expression (λx.body)
    fn parse_unicode_lambda(text: &str) -> LambdaExpression {
        // Find the dot separator
        if let Some(dot_pos) = text.find('.') {
            // Skip the λ character properly using char boundaries
            let param_part = if text.starts_with('λ') {
                &text[text.char_indices().nth(1).map(|(i, _)| i).unwrap_or(1)..dot_pos]
            } else {
                &text[1..dot_pos]
            }
            .trim();
            let body_part = text[dot_pos + 1..].trim();

            let parameters = Self::parse_parameters(param_part);
            let body = Self::parse_lambda_body(body_part);

            LambdaExpression {
                parameters,
                body,
                span: None,
            }
        } else {
            // Malformed lambda, create fallback
            LambdaExpression {
                parameters: vec!["x".to_string()],
                body: LogicalExpression::Variable(text.to_string()),
                span: None,
            }
        }
    }

    /// Parse ASCII lambda expression (\x.body)
    fn parse_ascii_lambda(text: &str) -> LambdaExpression {
        if let Some(dot_pos) = text.find('.') {
            let param_part = &text[1..dot_pos].trim(); // Skip the \
            let body_part = text[dot_pos + 1..].trim();

            let parameters = Self::parse_parameters(param_part);
            let body = Self::parse_lambda_body(body_part);

            LambdaExpression {
                parameters,
                body,
                span: None,
            }
        } else {
            // Malformed lambda
            LambdaExpression {
                parameters: vec!["x".to_string()],
                body: LogicalExpression::Variable(text.to_string()),
                span: None,
            }
        }
    }

    /// Parse function syntax: function(x, y) { body }
    fn parse_function_syntax(text: &str) -> LambdaExpression {
        // Extract parameters from function(x, y) part
        if let Some(paren_start) = text.find('(') {
            if let Some(paren_end) = text.find(')') {
                let param_text = &text[paren_start + 1..paren_end];
                let parameters = Self::parse_parameter_list(param_text);

                // Extract body from { body } part
                if let Some(brace_start) = text.find('{') {
                    if let Some(brace_end) = text.rfind('}') {
                        let body_text = &text[brace_start + 1..brace_end].trim();
                        let body = Self::parse_lambda_body(body_text);

                        return LambdaExpression {
                            parameters,
                            body,
                            span: None,
                        };
                    }
                }

                // Fallback if braces not found
                return LambdaExpression {
                    parameters,
                    body: LogicalExpression::Variable(text.to_string()),
                    span: None,
                };
            }
        }

        // Complete fallback
        LambdaExpression {
            parameters: vec!["x".to_string()],
            body: LogicalExpression::Variable(text.to_string()),
            span: None,
        }
    }

    /// Parse lambda parameters (handles both single and multiple parameters)
    fn parse_parameters(param_text: &str) -> Vec<String> {
        if param_text.contains(',') {
            Self::parse_parameter_list(param_text)
        } else {
            vec![param_text.to_string()]
        }
    }

    /// Parse comma-separated parameter list
    fn parse_parameter_list(param_text: &str) -> Vec<String> {
        param_text
            .split(',')
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .collect()
    }

    /// Parse lambda body into logical expression
    fn parse_lambda_body(body_text: &str) -> LogicalExpression {
        let body_text = body_text.trim();

        // Check for common logical operators
        if body_text.contains('+')
            || body_text.contains('-')
            || body_text.contains('*')
            || body_text.contains('/')
        {
            // Mathematical expression - parse as application
            Self::parse_mathematical_expression(body_text)
        } else if body_text.contains('=') || body_text.contains('<') || body_text.contains('>') {
            // Comparison expression
            Self::parse_comparison_expression(body_text)
        } else if body_text.contains('∧') || body_text.contains('∨') || body_text.contains('¬')
        {
            // Logical expression
            LogicalExpression::Raw(body_text.to_string())
        } else {
            // Simple variable or expression
            LogicalExpression::Variable(body_text.to_string())
        }
    }

    /// Parse mathematical expression (simplified)
    fn parse_mathematical_expression(expr: &str) -> LogicalExpression {
        // For now, treat as a function application
        // This could be enhanced with proper expression parsing
        LogicalExpression::Application {
            function: "math".to_string(),
            arguments: vec![LogicalExpression::Raw(expr.to_string())],
        }
    }

    /// Parse comparison expression (simplified)
    fn parse_comparison_expression(expr: &str) -> LogicalExpression {
        // For now, treat as raw logical expression
        LogicalExpression::Raw(expr.to_string())
    }

    /// Validate lambda expression syntax
    pub fn validate_lambda(lambda: &LambdaExpression) -> AispResult<()> {
        if lambda.parameters.is_empty() {
            return Err(AispError::validation_error(
                "Lambda expression must have at least one parameter",
            ));
        }

        for param in &lambda.parameters {
            if param.is_empty() {
                return Err(AispError::validation_error(
                    "Lambda parameter cannot be empty",
                ));
            }

            if !Self::is_valid_identifier(param) {
                return Err(AispError::validation_error(format!(
                    "Invalid lambda parameter: '{}'. Must be valid identifier",
                    param
                )));
            }
        }

        Ok(())
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_unicode_lambda() {
        let lambda = LambdaContentParser::parse_lambda_expression("λx.x + 1");
        assert_eq!(lambda.parameters, vec!["x".to_string()]);
    }

    #[test]
    fn test_parse_ascii_lambda() {
        let lambda = LambdaContentParser::parse_lambda_expression("\\x.x + 1");
        assert_eq!(lambda.parameters, vec!["x".to_string()]);
    }

    #[test]
    fn test_parse_function_syntax() {
        let lambda = LambdaContentParser::parse_lambda_expression("function(x, y) { x + y }");
        assert_eq!(lambda.parameters, vec!["x".to_string(), "y".to_string()]);
    }

    #[test]
    fn test_parse_function_definition() {
        let result = LambdaContentParser::parse_function_definition("successor≜λn.n + 1");
        assert!(result.is_some());

        let (name, lambda) = result.unwrap();
        assert_eq!(name, "successor");
        assert_eq!(lambda.parameters, vec!["n".to_string()]);
    }

    #[test]
    fn test_validate_lambda() {
        let valid_lambda = LambdaExpression {
            parameters: vec!["x".to_string()],
            body: LogicalExpression::Variable("x".to_string()),
            span: None,
        };
        assert!(LambdaContentParser::validate_lambda(&valid_lambda).is_ok());

        let invalid_lambda = LambdaExpression {
            parameters: vec![],
            body: LogicalExpression::Variable("x".to_string()),
            span: None,
        };
        assert!(LambdaContentParser::validate_lambda(&invalid_lambda).is_err());
    }

    #[test]
    fn test_parameter_parsing() {
        let params = LambdaContentParser::parse_parameter_list("x, y, z");
        assert_eq!(
            params,
            vec!["x".to_string(), "y".to_string(), "z".to_string()]
        );

        let single_param = LambdaContentParser::parse_parameters("x");
        assert_eq!(single_param, vec!["x".to_string()]);
    }

    #[test]
    fn test_identifier_validation() {
        assert!(LambdaContentParser::is_valid_identifier("validName"));
        assert!(LambdaContentParser::is_valid_identifier("_underscore"));
        assert!(LambdaContentParser::is_valid_identifier("name123"));
        assert!(!LambdaContentParser::is_valid_identifier("123invalid"));
        assert!(!LambdaContentParser::is_valid_identifier("name-with-dash"));
        assert!(!LambdaContentParser::is_valid_identifier(""));
    }
}
