//! AISP Core - High-performance parsing and validation for AI Symbolic Protocol
//!
//! This crate provides the foundational components for parsing and validating
//! AISP 5.1 documents with zero-copy parsing and strong type safety guarantees.

pub mod ast;
pub mod batch_verification; // Batch verification optimization
pub mod parser; // Consolidated SRP-compliant parser
pub mod relational_new;
pub mod temporal_new;
pub mod validator;
// pub mod z3_integration; // Temporarily disabled for compilation
pub mod conflict_types;
pub mod error;
pub mod symbols;

// New modular semantic analysis components
pub mod type_checker;

// New modular relational analysis components
pub mod conflict_detector;
pub mod constraint_solver;

// New modular temporal analysis components
pub mod temporal_logic_solver;
pub mod temporal_model_checker;
pub mod temporal_operator_analyzer;
pub mod temporal_pattern_detector;

// Concurrent behavior verification components
pub mod concurrent_behavior_verifier;

// Advanced formal verification components
pub mod axiom_system;
pub mod formula_converter;
pub mod model_checker;
pub mod proof_search;
pub mod proof_types;
pub mod property_extractor;
pub mod property_factory;
pub mod property_types;
pub mod smt_formula_converter;
pub mod smt_generator;
pub mod smt_types;
pub mod theorem_prover;

// Invariant discovery system components
pub mod invariant_analyzer;
pub mod invariant_discovery;
pub mod invariant_discovery_main;
pub mod invariant_exporters;
pub mod invariant_formulas;
pub mod invariant_types;

// Satisfiability checking components
pub mod satisfiability_checker;

// Formal verification framework
pub mod formal_verification;

// Advanced behavioral verification components
pub mod performance_verification;
pub mod protocol_state_machine;
pub mod resource_utilization;

// Formal methods and remediation components
pub mod completeness_analysis;
pub mod formal_semantics;
pub mod semantic_preservation;
pub mod soundness_proofs;

// Rigorous mathematical foundations
pub mod incompleteness_handler;
pub mod mathematical_evaluator;
pub mod mathematical_semantics;
pub mod mechanized_proofs;
pub mod vector_space_verifier;

// Tri-vector signal validation
pub mod tri_vector_validation;

// Enhanced Z3 SMT solver integration (modular)
pub mod semantic_z3_verifier;
pub mod z3_verification;

// Enhanced Z3 SMT solver integration (legacy re-exports)
pub mod enhanced_z3_verification;

// Ghost intent search validation
pub mod ghost_intent_validation;

// RossNet scoring validation
pub mod rossnet_scoring;

// Complete AISP 5.1 Architecture Implementation
pub mod compositional_proof_chain; // Compositional proof chain connecting all layers
pub mod core_features; // Missing core features F4,F6,F7,F14,F15,F16,F18
pub mod ghost_intent_search; // Layer 2 (𝕃₂): Ghost Intent Search
pub mod performance_guarantees;
pub mod pocket_architecture; // Layer 1 (𝕃₁): Pocket Architecture // Performance guarantee verifications

// Phase 2: Enhanced Verification Capabilities
pub mod advanced_theorem_prover; // Advanced mathematical theorem proving
pub mod category_theory_verifier; // Category theory verification module
pub mod mathematical_notation_parser; // Enhanced Unicode mathematical parsing

// Hebbian learning constraint validation
pub mod hebbian_learning;

// Anti-drift protocol verification
pub mod anti_drift;

// Reference.md specification compliance validator
pub mod reference_validator;

// Reference.md comprehensive integration testing
pub mod reference_integration_test;

// Security hardening components (Pest parser migration)
pub mod grammar;
pub mod testing;

// Deep verification architecture (Phase 2)
pub mod semantic;

// Test fixtures and utilities are now handled inline in each module

pub use ast::*;
pub use parser::*;
pub use relational_new::*;
pub use semantic::*;
pub use temporal_new::*;
pub use validator::*;
// pub use z3_integration::*; // Temporarily disabled
pub use error::*;

/// AISP version supported by this implementation
pub const AISP_VERSION: &str = "5.1";

/// Maximum supported document size (1MB)
pub const MAX_DOCUMENT_SIZE: usize = 1024 * 1024;

/// Quality tier thresholds
pub mod tier_thresholds {
    pub const PLATINUM: f64 = 0.75;
    pub const GOLD: f64 = 0.60;
    pub const SILVER: f64 = 0.40;
    pub const BRONZE: f64 = 0.20;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(AISP_VERSION, "5.1");
    }
}
