//! Type Content Parser
//!
//! Focused parser for type definition content following SRP.
//! Handles parsing of type expressions and definitions.

use crate::ast::canonical::{BasicType, TypeExpression};
use crate::error::{AispError, AispResult};

/// SRP-focused parser for type definition content
pub struct TypeContentParser;

impl TypeContentParser {
    /// Parse type definition from "TypeName≜TypeExpression" format
    pub fn parse_type_definition(def_text: &str) -> Option<(String, TypeExpression)> {
        if let Some(pos) = def_text.find('≜') {
            let name = def_text[..pos].trim().to_string();
            let type_expr_text = def_text[pos + '≜'.len_utf8()..].trim();

            let type_expr = Self::parse_type_expression(type_expr_text);
            Some((name, type_expr))
        } else {
            None
        }
    }

    /// Parse type expression from text
    pub fn parse_type_expression(type_text: &str) -> TypeExpression {
        let type_text = type_text.trim();

        // Handle brace types: {a,b,c} is an enumeration, {element_type} a set
        if type_text.starts_with('{') && type_text.ends_with('}') {
            let inner = &type_text[1..type_text.len() - 1].trim();
            if inner.contains(',') {
                let values: Vec<String> = inner.split(',').map(|v| v.trim().to_string()).collect();
                return TypeExpression::Enumeration(values);
            }
            let element_type = Self::parse_type_expression(inner);
            return TypeExpression::Set(Box::new(element_type));
        }

        // Handle union types: Type1 | Type2 | Type3
        if type_text.contains('|') {
            let types: Vec<TypeExpression> = type_text
                .split('|')
                .map(|t| Self::parse_type_expression(t.trim()))
                .collect();
            return TypeExpression::Union(types);
        }

        // Handle product types: Type1 * Type2 * Type3
        if type_text.contains('*') {
            let types: Vec<TypeExpression> = type_text
                .split('*')
                .map(|t| Self::parse_type_expression(t.trim()))
                .collect();
            return TypeExpression::Product(types);
        }

        // Handle function types: (param1, param2) -> return_type
        if type_text.contains("->") {
            return Self::parse_function_type(type_text);
        }

        // Handle tuple types: (Type1, Type2, Type3)
        if type_text.starts_with('(') && type_text.ends_with(')') && type_text.contains(',') {
            let inner = &type_text[1..type_text.len() - 1];
            let types: Vec<TypeExpression> = inner
                .split(',')
                .map(|t| Self::parse_type_expression(t.trim()))
                .collect();
            return TypeExpression::Tuple(types);
        }

        // Handle enumeration types: enum{value1, value2, value3}
        if type_text.starts_with("enum{") && type_text.ends_with('}') {
            let inner = &type_text[5..type_text.len() - 1];
            let values: Vec<String> = inner.split(',').map(|v| v.trim().to_string()).collect();
            return TypeExpression::Enumeration(values);
        }

        // Parse as basic type
        TypeExpression::Basic(Self::parse_basic_type(type_text))
    }

    /// Parse function type from "(params) -> return" format
    fn parse_function_type(type_text: &str) -> TypeExpression {
        if let Some(arrow_pos) = type_text.find("->") {
            let params_text = type_text[..arrow_pos].trim();
            let return_text = type_text[arrow_pos + 2..].trim();

            // Parse parameters
            let params = if params_text.starts_with('(') && params_text.ends_with(')') {
                let inner = &params_text[1..params_text.len() - 1];
                if inner.is_empty() {
                    Vec::new()
                } else {
                    inner
                        .split(',')
                        .map(|p| Self::parse_type_expression(p.trim()))
                        .collect()
                }
            } else {
                vec![Self::parse_type_expression(params_text)]
            };

            let return_type = Box::new(Self::parse_type_expression(return_text));

            TypeExpression::Function {
                params,
                return_type,
            }
        } else {
            // Fallback if arrow not found
            TypeExpression::Basic(BasicType::Custom(type_text.to_string()))
        }
    }

    /// Parse basic type from text
    fn parse_basic_type(type_text: &str) -> BasicType {
        match type_text {
            // Unicode mathematical symbols
            "ℕ" => BasicType::Natural,
            "ℤ" => BasicType::Integer,
            "ℝ" => BasicType::Real,
            "ℂ" => BasicType::Real, // Complex numbers mapped to Real for now
            "ℚ" => BasicType::Real, // Rationals mapped to Real for now
            "𝔹" => BasicType::Boolean,
            "𝕊" => BasicType::String,

            // ASCII equivalents
            "Natural" | "Nat" | "N" => BasicType::Natural,
            "Integer" | "Int" | "Z" => BasicType::Integer,
            "Real" | "R" | "Float" | "Double" => BasicType::Real,
            "Boolean" | "Bool" | "B" => BasicType::Boolean,
            "String" | "Str" | "S" => BasicType::String,
            "Symbol" | "Sym" => BasicType::Symbol,
            "Unit" => BasicType::Custom("Unit".to_string()),

            // Vector space types
            type_text if type_text.starts_with("Vector") => Self::parse_vector_type(type_text),
            "RealVector" | "ℝ^n" => BasicType::RealVector,
            "DirectSum" | "⊕" => BasicType::DirectSum,

            // Mathematical structure types
            type_text
                if type_text.starts_with("Group")
                    || type_text.starts_with("Ring")
                    || type_text.starts_with("Field") =>
            {
                BasicType::MathematicalStructure(type_text.to_string())
            }

            // Custom types
            _ => BasicType::Custom(type_text.to_string()),
        }
    }

    /// Parse vector space type with dimension
    fn parse_vector_type(type_text: &str) -> BasicType {
        // Handle Vector[n] format
        if type_text.starts_with("Vector[") && type_text.ends_with(']') {
            let dim_text = &type_text[7..type_text.len() - 1];
            if let Ok(dimension) = dim_text.parse::<usize>() {
                return BasicType::VectorSpace(dimension);
            }
        }

        // Handle Vector(n) format
        if type_text.starts_with("Vector(") && type_text.ends_with(')') {
            let dim_text = &type_text[7..type_text.len() - 1];
            if let Ok(dimension) = dim_text.parse::<usize>() {
                return BasicType::VectorSpace(dimension);
            }
        }

        // Default to custom type if parsing fails
        BasicType::Custom(type_text.to_string())
    }

    /// Validate type name
    pub fn validate_type_name(name: &str) -> AispResult<()> {
        if name.is_empty() {
            return Err(AispError::validation_error("Type name cannot be empty"));
        }

        // Type names should start with uppercase letter
        let first_char = name.chars().next().unwrap();
        if !first_char.is_uppercase() {
            return Err(AispError::validation_error(&format!(
                "Type name '{}' should start with uppercase letter",
                name
            )));
        }

        // Check for valid characters
        if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(AispError::validation_error(&format!(
                "Invalid type name '{}'. Only alphanumeric and underscore allowed",
                name
            )));
        }

        Ok(())
    }

    /// Check if type expression is well-formed
    pub fn validate_type_expression(expr: &TypeExpression) -> AispResult<()> {
        match expr {
            TypeExpression::Set(element_type) => Self::validate_type_expression(element_type),
            TypeExpression::Union(types) => {
                if types.is_empty() {
                    return Err(AispError::validation_error(
                        "Union type must have at least one type",
                    ));
                }
                for type_expr in types {
                    Self::validate_type_expression(type_expr)?;
                }
                Ok(())
            }
            TypeExpression::Product(types) => {
                if types.is_empty() {
                    return Err(AispError::validation_error(
                        "Product type must have at least one type",
                    ));
                }
                for type_expr in types {
                    Self::validate_type_expression(type_expr)?;
                }
                Ok(())
            }
            TypeExpression::Function {
                params,
                return_type,
            } => {
                for param in params {
                    Self::validate_type_expression(param)?;
                }
                Self::validate_type_expression(return_type)
            }
            TypeExpression::Tuple(types) => {
                if types.len() < 2 {
                    return Err(AispError::validation_error(
                        "Tuple type must have at least two types",
                    ));
                }
                for type_expr in types {
                    Self::validate_type_expression(type_expr)?;
                }
                Ok(())
            }
            TypeExpression::Enumeration(values) => {
                if values.is_empty() {
                    return Err(AispError::validation_error(
                        "Enumeration type must have at least one value",
                    ));
                }
                Ok(())
            }
            TypeExpression::Basic(_) => Ok(()), // Basic types are always valid
        }
    }

    /// Extract all type names from expression (for dependency analysis)
    pub fn extract_type_dependencies(expr: &TypeExpression) -> Vec<String> {
        let mut dependencies = Vec::new();
        Self::collect_type_dependencies(expr, &mut dependencies);
        dependencies.sort();
        dependencies.dedup();
        dependencies
    }

    /// Recursively collect type dependencies
    fn collect_type_dependencies(expr: &TypeExpression, deps: &mut Vec<String>) {
        match expr {
            TypeExpression::Basic(BasicType::Custom(name)) => {
                deps.push(name.clone());
            }
            TypeExpression::Set(inner) => {
                Self::collect_type_dependencies(inner, deps);
            }
            TypeExpression::Union(types)
            | TypeExpression::Product(types)
            | TypeExpression::Tuple(types) => {
                for type_expr in types {
                    Self::collect_type_dependencies(type_expr, deps);
                }
            }
            TypeExpression::Function {
                params,
                return_type,
            } => {
                for param in params {
                    Self::collect_type_dependencies(param, deps);
                }
                Self::collect_type_dependencies(return_type, deps);
            }
            _ => {} // Built-in types don't create dependencies
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_type_definition() {
        let result = TypeContentParser::parse_type_definition("Natural≜ℕ");
        assert!(result.is_some());

        let (name, type_expr) = result.unwrap();
        assert_eq!(name, "Natural");
        assert!(matches!(
            type_expr,
            TypeExpression::Basic(BasicType::Natural)
        ));
    }

    #[test]
    fn test_parse_set_type() {
        let type_expr = TypeContentParser::parse_type_expression("{ℕ}");
        assert!(matches!(type_expr, TypeExpression::Set(_)));
    }

    #[test]
    fn test_parse_union_type() {
        let type_expr = TypeContentParser::parse_type_expression("ℕ | ℤ | ℝ");
        assert!(matches!(type_expr, TypeExpression::Union(_)));
    }

    #[test]
    fn test_parse_function_type() {
        let type_expr = TypeContentParser::parse_type_expression("(ℕ, ℕ) -> ℝ");
        assert!(matches!(type_expr, TypeExpression::Function { .. }));
    }

    #[test]
    fn test_parse_vector_type() {
        let basic_type = TypeContentParser::parse_basic_type("Vector[3]");
        assert_eq!(basic_type, BasicType::VectorSpace(3));

        let basic_type2 = TypeContentParser::parse_basic_type("Vector(5)");
        assert_eq!(basic_type2, BasicType::VectorSpace(5));
    }

    #[test]
    fn test_parse_enumeration_type() {
        let type_expr = TypeContentParser::parse_type_expression("enum{Red, Green, Blue}");
        if let TypeExpression::Enumeration(values) = type_expr {
            assert_eq!(values, vec!["Red", "Green", "Blue"]);
        } else {
            panic!("Expected enumeration type");
        }
    }

    #[test]
    fn test_validate_type_name() {
        assert!(TypeContentParser::validate_type_name("ValidType").is_ok());
        assert!(TypeContentParser::validate_type_name("Type_With_Underscores").is_ok());
        assert!(TypeContentParser::validate_type_name("Type123").is_ok());

        assert!(TypeContentParser::validate_type_name("").is_err());
        assert!(TypeContentParser::validate_type_name("lowercase").is_err());
        assert!(TypeContentParser::validate_type_name("Type-With-Dashes").is_err());
    }

    #[test]
    fn test_validate_type_expression() {
        let valid_union = TypeExpression::Union(vec![
            TypeExpression::Basic(BasicType::Natural),
            TypeExpression::Basic(BasicType::Integer),
        ]);
        assert!(TypeContentParser::validate_type_expression(&valid_union).is_ok());

        let invalid_union = TypeExpression::Union(vec![]);
        assert!(TypeContentParser::validate_type_expression(&invalid_union).is_err());

        let invalid_tuple = TypeExpression::Tuple(vec![TypeExpression::Basic(BasicType::Natural)]);
        assert!(TypeContentParser::validate_type_expression(&invalid_tuple).is_err());
    }

    #[test]
    fn test_extract_dependencies() {
        let complex_type = TypeExpression::Function {
            params: vec![TypeExpression::Basic(BasicType::Custom(
                "CustomA".to_string(),
            ))],
            return_type: Box::new(TypeExpression::Set(Box::new(TypeExpression::Basic(
                BasicType::Custom("CustomB".to_string()),
            )))),
        };

        let deps = TypeContentParser::extract_type_dependencies(&complex_type);
        assert_eq!(deps, vec!["CustomA", "CustomB"]);
    }

    #[test]
    fn test_unicode_basic_types() {
        assert!(matches!(
            TypeContentParser::parse_basic_type("ℕ"),
            BasicType::Natural
        ));
        assert!(matches!(
            TypeContentParser::parse_basic_type("ℤ"),
            BasicType::Integer
        ));
        assert!(matches!(
            TypeContentParser::parse_basic_type("ℝ"),
            BasicType::Real
        ));
        assert!(matches!(
            TypeContentParser::parse_basic_type("𝔹"),
            BasicType::Boolean
        ));
        assert!(matches!(
            TypeContentParser::parse_basic_type("𝕊"),
            BasicType::String
        ));
    }
}
