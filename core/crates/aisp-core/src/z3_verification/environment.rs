//! Z3 environment setup and AISP-specific sort/function declarations
//!
//! This module handles the setup of Z3 contexts, sorts, and functions
//! specifically tailored for AISP document verification.

use crate::{
    ast::canonical::{
        BasicType, CanonicalAispBlock as AispBlock, CanonicalAispDocument as AispDocument,
        FunctionDefinition, TypeDefinition, TypeExpression,
    },
    error::*,
};
use std::collections::HashMap;

#[cfg(feature = "z3-verification")]
use z3::*;

/// AISP type environment for Z3
pub struct AispZ3Environment {
    /// Declared sorts by name
    pub sorts: HashMap<String, AispSort>,
    /// Function declarations by name
    pub functions: HashMap<String, AispFunction>,
    /// Constant declarations by name
    pub constants: HashMap<String, AispConstant>,
}

/// AISP sort representation
#[derive(Debug, Clone)]
pub struct AispSort {
    /// Sort name
    pub name: String,
    /// Sort arity (for parameterized sorts)
    pub arity: usize,
    /// Sort description
    pub description: String,
}

/// AISP function representation
#[derive(Debug, Clone)]
pub struct AispFunction {
    /// Function name
    pub name: String,
    /// Domain types
    pub domain: Vec<String>,
    /// Codomain type
    pub codomain: String,
    /// Function interpretation
    pub interpretation: Option<String>,
}

/// AISP constant representation
#[derive(Debug, Clone)]
pub struct AispConstant {
    /// Constant name
    pub name: String,
    /// Constant type
    pub sort: String,
    /// Constant value (if known)
    pub value: Option<String>,
}

impl AispZ3Environment {
    /// Create new AISP Z3 environment
    pub fn new() -> Self {
        Self {
            sorts: HashMap::new(),
            functions: HashMap::new(),
            constants: HashMap::new(),
        }
    }

    /// Setup Z3 environment with AISP-specific sorts and functions
    #[cfg(feature = "z3-verification")]
    pub fn setup_context(&mut self, context: &Context) -> AispResult<()> {
        self.declare_basic_sorts(context)?;
        self.declare_vector_sorts(context)?;
        self.declare_aisp_sorts(context)?;
        Ok(())
    }

    /// Setup environment from AISP document
    pub fn setup_from_document(&mut self, document: &AispDocument) -> AispResult<()> {
        // Process type definitions
        for block in &document.blocks {
            if let AispBlock::Types(types_block) = block {
                for (name, type_def) in &types_block.definitions {
                    self.declare_type_sort(name, &type_def.type_expr)?;
                }
            }
        }

        // Process function definitions
        for block in &document.blocks {
            if let AispBlock::Functions(funcs_block) = block {
                for func_def in &funcs_block.functions {
                    self.declare_function(&func_def.name, func_def)?;
                }
            }
        }

        Ok(())
    }

    /// Declare basic Z3 sorts
    #[cfg(feature = "z3-verification")]
    fn declare_basic_sorts(&mut self, _context: &Context) -> AispResult<()> {
        // Real numbers
        self.sorts.insert(
            "Real".to_string(),
            AispSort {
                name: "Real".to_string(),
                arity: 0,
                description: "Real numbers".to_string(),
            },
        );

        // Integers
        self.sorts.insert(
            "Int".to_string(),
            AispSort {
                name: "Int".to_string(),
                arity: 0,
                description: "Integers".to_string(),
            },
        );

        // Booleans
        self.sorts.insert(
            "Bool".to_string(),
            AispSort {
                name: "Bool".to_string(),
                arity: 0,
                description: "Boolean values".to_string(),
            },
        );

        Ok(())
    }

    /// Declare vector space sorts for tri-vector validation
    #[cfg(feature = "z3-verification")]
    fn declare_vector_sorts(&mut self, _context: &Context) -> AispResult<()> {
        // Generic vector space
        self.sorts.insert(
            "Vector".to_string(),
            AispSort {
                name: "Vector".to_string(),
                arity: 0,
                description: "Vector space element".to_string(),
            },
        );

        // Specific vector spaces for AISP tri-vector
        let vector_spaces = [
            ("V_H", "Semantic vector space (768 dimensions)"),
            ("V_L", "Structural vector space (512 dimensions)"),
            ("V_S", "Safety vector space (256 dimensions)"),
        ];

        for (name, desc) in &vector_spaces {
            self.sorts.insert(
                name.to_string(),
                AispSort {
                    name: name.to_string(),
                    arity: 0,
                    description: desc.to_string(),
                },
            );
        }

        // Vector space functions
        self.declare_vector_functions()?;

        Ok(())
    }

    /// Declare AISP-specific sorts
    #[cfg(feature = "z3-verification")]
    fn declare_aisp_sorts(&mut self, _context: &Context) -> AispResult<()> {
        // AISP Signal type
        self.sorts.insert(
            "Signal".to_string(),
            AispSort {
                name: "Signal".to_string(),
                arity: 0,
                description: "AISP signal (tri-vector decomposition)".to_string(),
            },
        );

        // AISP Pocket type
        self.sorts.insert(
            "Pocket".to_string(),
            AispSort {
                name: "Pocket".to_string(),
                arity: 0,
                description: "AISP pocket structure".to_string(),
            },
        );

        // AISP Quality tier type
        self.sorts.insert(
            "QualityTier".to_string(),
            AispSort {
                name: "QualityTier".to_string(),
                arity: 0,
                description: "AISP quality tier (◊⁺⁺, ◊⁺, ◊, ◊⁻, ⊘)".to_string(),
            },
        );

        Ok(())
    }

    /// Declare vector space operations
    fn declare_vector_functions(&mut self) -> AispResult<()> {
        let vector_functions = [
            (
                "dot_product",
                vec!["Vector".to_string(), "Vector".to_string()],
                "Real".to_string(),
                "Vector dot product operation",
            ),
            (
                "in_space",
                vec!["Vector".to_string(), "String".to_string()],
                "Bool".to_string(),
                "Check if vector belongs to named space",
            ),
            (
                "project_H",
                vec!["Signal".to_string()],
                "V_H".to_string(),
                "Project signal to semantic space",
            ),
            (
                "project_L",
                vec!["Signal".to_string()],
                "V_L".to_string(),
                "Project signal to structural space",
            ),
            (
                "project_S",
                vec!["Signal".to_string()],
                "V_S".to_string(),
                "Project signal to safety space",
            ),
        ];

        for (name, domain, codomain, desc) in &vector_functions {
            self.functions.insert(
                name.to_string(),
                AispFunction {
                    name: name.to_string(),
                    domain: domain.clone(),
                    codomain: codomain.clone(),
                    interpretation: Some(desc.to_string()),
                },
            );
        }

        Ok(())
    }

    /// Declare type sort from AISP type expression
    fn declare_type_sort(&mut self, name: &str, type_expr: &TypeExpression) -> AispResult<()> {
        let (sort_name, description) = match type_expr {
            TypeExpression::Basic(BasicType::Natural) => {
                ("Int".to_string(), "Natural numbers".to_string())
            }
            TypeExpression::Basic(BasicType::Real) => {
                ("Real".to_string(), "Real numbers".to_string())
            }
            TypeExpression::Basic(BasicType::Boolean) => {
                ("Bool".to_string(), "Boolean values".to_string())
            }
            TypeExpression::Basic(BasicType::String) => {
                ("String".to_string(), "String values".to_string())
            }
            _ => (name.to_string(), format!("User-defined type: {}", name)),
        };

        self.sorts.insert(
            name.to_string(),
            AispSort {
                name: sort_name,
                arity: 0,
                description,
            },
        );

        Ok(())
    }

    /// Declare function from AISP function definition
    fn declare_function(&mut self, name: &str, _func_def: &FunctionDefinition) -> AispResult<()> {
        // For now, create uninterpreted functions
        // TODO: Parse function signatures properly
        self.functions.insert(
            name.to_string(),
            AispFunction {
                name: name.to_string(),
                domain: vec!["Any".to_string()],
                codomain: "Any".to_string(),
                interpretation: None,
            },
        );

        Ok(())
    }

    /// Get sort by name
    pub fn get_sort(&self, name: &str) -> Option<&AispSort> {
        self.sorts.get(name)
    }

    /// Get function by name
    pub fn get_function(&self, name: &str) -> Option<&AispFunction> {
        self.functions.get(name)
    }

    /// Get constant by name
    pub fn get_constant(&self, name: &str) -> Option<&AispConstant> {
        self.constants.get(name)
    }

    /// Check if sort is defined
    pub fn has_sort(&self, name: &str) -> bool {
        self.sorts.contains_key(name)
    }

    /// Check if function is defined
    pub fn has_function(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    /// Get all vector space sorts
    pub fn get_vector_spaces(&self) -> Vec<&AispSort> {
        self.sorts
            .values()
            .filter(|sort| sort.name.starts_with("V_") || sort.name == "Vector")
            .collect()
    }

    /// Get tri-vector projection functions
    pub fn get_tri_vector_projections(&self) -> Vec<&AispFunction> {
        self.functions
            .values()
            .filter(|func| func.name.starts_with("project_"))
            .collect()
    }
}

impl Default for AispZ3Environment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_environment() -> AispZ3Environment {
        let mut env = AispZ3Environment::new();

        // Simulate basic setup without Z3 context
        env.sorts.insert(
            "Real".to_string(),
            AispSort {
                name: "Real".to_string(),
                arity: 0,
                description: "Real numbers".to_string(),
            },
        );

        env.sorts.insert(
            "V_H".to_string(),
            AispSort {
                name: "V_H".to_string(),
                arity: 0,
                description: "Semantic vector space".to_string(),
            },
        );

        env.functions.insert(
            "dot_product".to_string(),
            AispFunction {
                name: "dot_product".to_string(),
                domain: vec!["Vector".to_string(), "Vector".to_string()],
                codomain: "Real".to_string(),
                interpretation: Some("Vector dot product".to_string()),
            },
        );

        env
    }

    #[test]
    fn test_environment_creation() {
        let env = AispZ3Environment::new();
        assert!(env.sorts.is_empty());
        assert!(env.functions.is_empty());
        assert!(env.constants.is_empty());
    }

    #[test]
    fn test_sort_management() {
        let env = create_test_environment();

        assert!(env.has_sort("Real"));
        assert!(env.has_sort("V_H"));
        assert!(!env.has_sort("NonExistent"));

        let real_sort = env.get_sort("Real").unwrap();
        assert_eq!(real_sort.name, "Real");
        assert_eq!(real_sort.arity, 0);
    }

    #[test]
    fn test_function_management() {
        let env = create_test_environment();

        assert!(env.has_function("dot_product"));
        assert!(!env.has_function("nonexistent_func"));

        let dot_product = env.get_function("dot_product").unwrap();
        assert_eq!(dot_product.name, "dot_product");
        assert_eq!(dot_product.domain.len(), 2);
        assert_eq!(dot_product.codomain, "Real");
    }

    #[test]
    fn test_vector_space_filtering() {
        let env = create_test_environment();
        let vector_spaces = env.get_vector_spaces();

        assert!(!vector_spaces.is_empty());
        let vh_found = vector_spaces.iter().any(|sort| sort.name == "V_H");
        assert!(vh_found);
    }

    #[test]
    fn test_type_expression_mapping() {
        let mut env = AispZ3Environment::new();

        let result = env.declare_type_sort("TestType", &TypeExpression::Basic(BasicType::Real));
        assert!(result.is_ok());

        assert!(env.has_sort("TestType"));
        let sort = env.get_sort("TestType").unwrap();
        assert_eq!(sort.name, "Real");
    }

    #[test]
    fn test_document_processing() {
        let mut env = AispZ3Environment::new();

        // Create a minimal test document
        let test_doc = AispDocument {
            header: crate::ast::canonical::DocumentHeader {
                version: "5.1".to_string(),
                name: "test".to_string(),
                date: "2026".to_string(),
                metadata: None,
            },
            metadata: crate::ast::canonical::DocumentMetadata {
                domain: Some("test".to_string()),
                protocol: None,
            },
            blocks: vec![],
            span: Some(crate::ast::canonical::Span {
                start: 0,
                end: 0,
                line: 1,
                column: 1,
            }),
        };

        let result = env.setup_from_document(&test_doc);
        assert!(result.is_ok());
    }
}
