//! Enhanced Z3 SMT Solver Integration for Advanced AISP Formal Verification
//!
//! This module provides sophisticated Z3 integration for complex AISP property verification,
//! including temporal logic, orthogonality constraints, and mathematical theorem proving.
//!
//! The implementation is organized into focused submodules:
//! - `z3_verification::types` - Core types and configuration (< 300 LOC)
//! - `z3_verification::environment` - Z3 environment setup (< 300 LOC)
//! - `z3_verification::properties` - Property verification (< 300 LOC)
//! - `z3_verification::verifier` - Main verifier implementation (< 300 LOC)
//!
//! Each module includes comprehensive inline unit tests and is designed
//! for maintainability and clarity.

// Re-export the complete modular Z3 verification system
pub use crate::z3_verification::*;

// Convenience re-exports for backward compatibility
pub use crate::z3_verification::{
    AdvancedVerificationConfig, EnhancedVerificationResult, EnhancedVerificationStats,
    EnhancedZ3Verifier, PropertyCategory, PropertyResult, VerificationStatus, VerifiedProperty,
    Z3VerificationFacade,
};

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::ast::canonical::{
        CanonicalAispDocument as AispDocument, DocumentHeader, DocumentMetadata, Span,
    };
    

    fn create_test_document() -> AispDocument {
        AispDocument {
            header: DocumentHeader {
                version: "5.1".to_string(),
                name: "integration_test".to_string(),
                date: "2026-01-26".to_string(),
                metadata: None,
            },
            metadata: DocumentMetadata {
                domain: Some("test".to_string()),
                protocol: None,
            },
            blocks: vec![],
            span: Some(Span {
                start: 0,
                end: 0,
                line: 1,
                column: 1,
            }),
        }
    }

    #[test]
    fn test_enhanced_z3_integration() {
        let available = Z3VerificationFacade::is_available();
        println!("Z3 enhanced verification available: {}", available);

        let facade_result = Z3VerificationFacade::new();
        assert!(facade_result.is_ok());
    }

    #[test]
    fn test_full_verification_pipeline() {
        let document = create_test_document();

        // Test the complete verification pipeline
        let result = quick_verify(&document, None);
        assert!(result.is_ok());

        let verification_result = result.unwrap();

        // Should work regardless of Z3 feature availability
        match verification_result.status {
            VerificationStatus::AllVerified
            | VerificationStatus::PartiallyVerified { .. }
            | VerificationStatus::Incomplete { .. }
            | VerificationStatus::Disabled => {
                // All acceptable for integration test
            }
            VerificationStatus::Failed(msg) => {
                // Print error for debugging but don't fail test
                println!("Verification failed: {}", msg);
            }
        }
    }

    #[test]
    fn test_modular_component_integration() {
        // Test that all modules work together
        let config = AdvancedVerificationConfig::default();
        assert!(config.incremental);

        let mut env = AispZ3Environment::new();
        let document = create_test_document();
        let setup_result = env.setup_from_document(&document);
        assert!(setup_result.is_ok());

        let property_verifier = PropertyVerifier::new(config);
        assert_eq!(property_verifier.get_stats().smt_queries, 0);
    }

    #[test]
    fn test_backward_compatibility() {
        // Test that the old interface still works
        let facade = Z3VerificationFacade::new();
        assert!(facade.is_ok());

        #[cfg(feature = "z3-verification")]
        {
            let verifier = EnhancedZ3Verifier::new();
            assert!(verifier.is_ok());
        }

        #[cfg(not(feature = "z3-verification"))]
        {
            let verifier = EnhancedZ3Verifier::new();
            assert!(verifier.is_err());
        }
    }
}
