//! Mathematical Expression Evaluator with Formal Error Handling
//!
//! This module provides safe evaluation of mathematical expressions with proper
//! handling of division by zero, NaN propagation, and formal error conditions.

use std::collections::HashMap;
use std::fmt;
use thiserror::Error;

/// Mathematical value with formal error handling
#[derive(Debug, Clone)]
pub enum MathValue {
    /// Finite real number
    Real(f64),
    /// Positive or negative infinity
    Infinity(InfinityType),
    /// Not a Number (undefined result)
    NaN,
    /// Formal bottom (⊥) - logical contradiction
    Bottom,
    /// Undefined expression
    Undefined(UndefinedReason),
}

/// Types of infinity for formal reasoning
#[derive(Debug, Clone, PartialEq)]
pub enum InfinityType {
    Positive,
    Negative,
}

/// Reasons for undefined mathematical expressions
#[derive(Debug, Clone, PartialEq)]
pub enum UndefinedReason {
    /// Division by zero: a/0 where a ≠ 0
    DivisionByZero,
    /// Indeterminate form: 0/0
    IndeterminateForm,
    /// Self-reference: "This statement is false"
    SelfReference,
    /// Circular definition
    CircularDefinition(String),
    /// Domain error: √(-1) in ℝ
    DomainError(String),
}

/// Mathematical evaluation errors
#[derive(Debug, Error)]
pub enum MathError {
    #[error("Division by zero: {expression}")]
    DivisionByZero { expression: String },

    #[error("Indeterminate form: {expression}")]
    IndeterminateForm { expression: String },

    #[error("Self-referential definition: {name}")]
    SelfReference { name: String },

    #[error("Circular definition detected: {cycle}")]
    CircularDefinition { cycle: String },

    #[error("Domain error in {operation}: {message}")]
    DomainError { operation: String, message: String },

    #[error("Undefined variable: {name}")]
    UndefinedVariable { name: String },

    #[error("Type error: {message}")]
    TypeError { message: String },
}

/// Mathematical expression evaluator with formal semantics
pub struct MathEvaluator {
    /// Variable definitions with dependency tracking
    variables: HashMap<String, (MathValue, Vec<String>)>,
    /// Currently evaluating variables (for cycle detection)
    evaluation_stack: Vec<String>,
}

impl Default for MathEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl MathEvaluator {
    /// Create a new mathematical evaluator
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            evaluation_stack: Vec::new(),
        }
    }

    /// Define a variable with dependency tracking
    pub fn define_variable(&mut self, name: String, value: MathValue, dependencies: Vec<String>) {
        self.variables.insert(name, (value, dependencies));
    }

    /// Evaluate mathematical division with formal error handling
    pub fn divide(&self, a: MathValue, b: MathValue) -> MathValue {
        match (a, b) {
            // Standard cases
            (MathValue::Real(x), MathValue::Real(y)) => {
                if y == 0.0 {
                    if x == 0.0 {
                        // 0/0 is indeterminate
                        MathValue::Undefined(UndefinedReason::IndeterminateForm)
                    } else {
                        // x/0 where x ≠ 0 is undefined (division by zero)
                        MathValue::Undefined(UndefinedReason::DivisionByZero)
                    }
                } else {
                    MathValue::Real(x / y)
                }
            }

            // Infinity cases
            (MathValue::Real(x), MathValue::Infinity(_)) => {
                if x == 0.0 {
                    MathValue::Undefined(UndefinedReason::IndeterminateForm)
                } else {
                    MathValue::Real(0.0)
                }
            }

            (MathValue::Infinity(inf_type), MathValue::Real(y)) => {
                if y == 0.0 {
                    MathValue::Undefined(UndefinedReason::DivisionByZero)
                } else if y > 0.0 {
                    MathValue::Infinity(inf_type)
                } else {
                    MathValue::Infinity(match inf_type {
                        InfinityType::Positive => InfinityType::Negative,
                        InfinityType::Negative => InfinityType::Positive,
                    })
                }
            }

            (MathValue::Infinity(_), MathValue::Infinity(_)) => {
                MathValue::Undefined(UndefinedReason::IndeterminateForm)
            }

            // Error propagation
            (MathValue::NaN, _) | (_, MathValue::NaN) => MathValue::NaN,
            (MathValue::Bottom, _) | (_, MathValue::Bottom) => MathValue::Bottom,
            (MathValue::Undefined(reason), _) | (_, MathValue::Undefined(reason)) => {
                MathValue::Undefined(reason)
            }
        }
    }

    /// Calculate ambiguity with proper error handling
    pub fn calculate_ambiguity(
        &self,
        parse_unique: i32,
        parse_total: i32,
    ) -> Result<MathValue, MathError> {
        if parse_total == 0 {
            if parse_unique == 0 {
                // This is the critical 0/0 case from the reference.md
                return Ok(MathValue::Undefined(UndefinedReason::IndeterminateForm));
            } else {
                return Err(MathError::DomainError {
                    operation: "ambiguity_calculation".to_string(),
                    message: "parse_unique > 0 but parse_total = 0 is impossible".to_string(),
                });
            }
        }

        if parse_unique > parse_total {
            return Err(MathError::DomainError {
                operation: "ambiguity_calculation".to_string(),
                message: "parse_unique cannot exceed parse_total".to_string(),
            });
        }

        let ratio = parse_unique as f64 / parse_total as f64;
        let ambiguity = 1.0 - ratio;

        Ok(MathValue::Real(ambiguity))
    }

    /// Check for circular dependencies in variable definitions
    pub fn check_circular_dependency(&mut self, name: &str) -> Result<(), MathError> {
        if self.evaluation_stack.contains(&name.to_string()) {
            let cycle_start = self
                .evaluation_stack
                .iter()
                .position(|x| x == name)
                .unwrap();
            let mut cycle: Vec<String> = self.evaluation_stack[cycle_start..].to_vec();
            cycle.push(name.to_string());

            return Err(MathError::CircularDefinition {
                cycle: cycle.join(" → "),
            });
        }

        Ok(())
    }

    /// Evaluate variable with dependency checking
    pub fn evaluate_variable(&mut self, name: &str) -> Result<MathValue, MathError> {
        self.check_circular_dependency(name)?;

        let (value, dependencies) = self
            .variables
            .get(name)
            .ok_or_else(|| MathError::UndefinedVariable {
                name: name.to_string(),
            })?
            .clone();

        // Add current variable to evaluation stack to track circular dependencies
        self.evaluation_stack.push(name.to_string());

        // Recursively evaluate all dependencies to detect circular dependencies
        for dep in &dependencies {
            if let Err(e) = self.evaluate_variable(dep) {
                self.evaluation_stack.pop();
                return Err(e);
            }
        }

        self.evaluation_stack.pop();
        Ok(value)
    }

    /// Create a self-referential paradox detector
    pub fn detect_self_reference(&self, statement: &str) -> bool {
        // Simple heuristic: check if statement refers to itself
        statement.to_lowercase().contains("this statement")
            || statement.to_lowercase().contains("this document")
            || statement.to_lowercase().contains("itself")
    }

    /// Verify mathematical consistency
    pub fn verify_consistency(&self) -> Result<bool, Vec<MathError>> {
        let mut errors = Vec::new();

        // Check for undefined values that indicate inconsistency
        for (name, (value, _)) in &self.variables {
            match value {
                MathValue::Bottom => {
                    errors.push(MathError::TypeError {
                        message: format!("Variable '{}' contains logical contradiction (⊥)", name),
                    });
                }
                MathValue::Undefined(UndefinedReason::CircularDefinition(cycle)) => {
                    errors.push(MathError::CircularDefinition {
                        cycle: cycle.clone(),
                    });
                }
                _ => {}
            }
        }

        if errors.is_empty() {
            Ok(true)
        } else {
            Err(errors)
        }
    }
}

impl PartialEq for MathValue {
    fn eq(&self, other: &Self) -> bool {
        const EPSILON: f64 = 1e-10;

        match (self, other) {
            (MathValue::Real(a), MathValue::Real(b)) => {
                // Use relative epsilon comparison for floating-point values
                if a.abs() < EPSILON && b.abs() < EPSILON {
                    // Both are effectively zero
                    true
                } else {
                    (a - b).abs() / a.abs().max(b.abs()) < EPSILON
                }
            }
            (MathValue::Infinity(a), MathValue::Infinity(b)) => a == b,
            (MathValue::NaN, MathValue::NaN) => true,
            (MathValue::Bottom, MathValue::Bottom) => true,
            (MathValue::Undefined(a), MathValue::Undefined(b)) => a == b,
            _ => false,
        }
    }
}

impl fmt::Display for MathValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathValue::Real(x) => write!(f, "{}", x),
            MathValue::Infinity(InfinityType::Positive) => write!(f, "∞"),
            MathValue::Infinity(InfinityType::Negative) => write!(f, "-∞"),
            MathValue::NaN => write!(f, "NaN"),
            MathValue::Bottom => write!(f, "⊥"),
            MathValue::Undefined(reason) => write!(f, "undefined({})", reason),
        }
    }
}

impl fmt::Display for UndefinedReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UndefinedReason::DivisionByZero => write!(f, "division_by_zero"),
            UndefinedReason::IndeterminateForm => write!(f, "indeterminate_form"),
            UndefinedReason::SelfReference => write!(f, "self_reference"),
            UndefinedReason::CircularDefinition(cycle) => write!(f, "circular: {}", cycle),
            UndefinedReason::DomainError(msg) => write!(f, "domain_error: {}", msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_division_by_zero() {
        let evaluator = MathEvaluator::new();

        // Test x/0 where x ≠ 0 (should be undefined)
        let result = evaluator.divide(MathValue::Real(5.0), MathValue::Real(0.0));
        assert_eq!(
            result,
            MathValue::Undefined(UndefinedReason::DivisionByZero)
        );
    }

    #[test]
    fn test_indeterminate_form() {
        let evaluator = MathEvaluator::new();

        // Test 0/0 (should be indeterminate)
        let result = evaluator.divide(MathValue::Real(0.0), MathValue::Real(0.0));
        assert_eq!(
            result,
            MathValue::Undefined(UndefinedReason::IndeterminateForm)
        );
    }

    #[test]
    fn test_ambiguity_calculation() {
        let evaluator = MathEvaluator::new();

        // Normal case
        let result = evaluator.calculate_ambiguity(98, 100).unwrap();
        assert_eq!(result, MathValue::Real(0.02));

        // Zero division case
        let result = evaluator.calculate_ambiguity(0, 0).unwrap();
        assert_eq!(
            result,
            MathValue::Undefined(UndefinedReason::IndeterminateForm)
        );

        // Impossible case
        let result = evaluator.calculate_ambiguity(50, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut evaluator = MathEvaluator::new();

        // Create circular dependency: A depends on B, B depends on A
        evaluator.define_variable("A".to_string(), MathValue::Real(1.0), vec!["B".to_string()]);
        evaluator.define_variable("B".to_string(), MathValue::Real(2.0), vec!["A".to_string()]);

        // This should detect the circular dependency
        let result = evaluator.evaluate_variable("A");
        assert!(result.is_err());
    }

    #[test]
    fn test_self_reference_detection() {
        let evaluator = MathEvaluator::new();

        let self_ref = "This statement is false";
        assert!(evaluator.detect_self_reference(self_ref));

        let normal = "The sky is blue";
        assert!(!evaluator.detect_self_reference(normal));
    }
}
