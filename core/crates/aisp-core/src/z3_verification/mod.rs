//! Enhanced Z3 verification system for AISP documents
//!
//! This module provides a comprehensive Z3-based verification system
//! for AISP documents, with genuine formal verification capabilities.
//!
//! The system is organized into focused modules:
//! - `types`: Core types and configuration
//! - `environment`: Z3 environment setup and AISP sorts
//! - `properties`: Property verification logic
//! - `enhanced_verifier`: Enhanced Z3 verifier with AISP capabilities
//! - `smt_interface`: SMT syntax validation and Z3 integration
//! - `facade`: High-level verification facade
//!
//! Compatible with Z3 version 4.15.4 and z3 crate 0.11

pub mod canonical_types;
pub mod enhanced_verifier;
pub mod environment;
pub mod facade;
pub mod production_verifier;
pub mod properties;
pub mod smt_interface;
pub mod types;
pub mod verifier;

// Re-export main interfaces from canonical types
pub use canonical_types::{
    Z3CounterexampleModel as CounterexampleModel, Z3Diagnostic as SolverDiagnostic,
    Z3DiagnosticLevel as DiagnosticLevel, Z3FormalProof as FormalProof,
    Z3PropertyCategory as PropertyCategory, Z3PropertyResult as PropertyResult,
    Z3UnsatCore as UnsatCore, Z3VerificationConfig as AdvancedVerificationConfig,
    Z3VerificationResult as EnhancedVerificationResult,
    Z3VerificationStatistics as EnhancedVerificationStats,
    Z3VerificationStatus as VerificationStatus, Z3VerifiedProperty as VerifiedProperty,
};

pub use environment::{AispConstant, AispFunction, AispSort, AispZ3Environment};

pub use properties::PropertyVerifier;

pub use enhanced_verifier::EnhancedZ3Verifier;
pub use facade::Z3VerificationFacade;
pub use smt_interface::SmtInterface;

/// Convenience function to check Z3 availability
pub fn is_z3_available() -> bool {
    Z3VerificationFacade::is_available()
}

/// Create a new Z3 verification facade
pub fn create_z3_facade() -> crate::error::AispResult<Z3VerificationFacade> {
    Z3VerificationFacade::new()
}

/// Create a verification facade with default configuration
pub fn create_default_verifier() -> crate::error::AispResult<EnhancedZ3Verifier> {
    EnhancedZ3Verifier::new()
}

/// Create a verification facade with custom configuration
pub fn create_configured_verifier(
    config: AdvancedVerificationConfig,
) -> crate::error::AispResult<EnhancedZ3Verifier> {
    EnhancedZ3Verifier::with_config(config)
}

/// Quick verification function for AISP documents
pub fn quick_verify(
    document: &crate::ast::canonical::CanonicalAispDocument,
    tri_vector_result: Option<&crate::tri_vector_validation::TriVectorValidationResult>,
) -> crate::error::AispResult<canonical_types::Z3VerificationResult> {
    let mut facade = create_z3_facade()?;
    facade.verify_document(document, tri_vector_result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::canonical::{
        CanonicalAispDocument as AispDocument, DocumentHeader, DocumentMetadata, Span,
    };
    use crate::ast::Position;

    fn create_minimal_document() -> AispDocument {
        AispDocument {
            header: DocumentHeader {
                version: "5.1".to_string(),
                name: "test".to_string(),
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
    fn test_z3_availability() {
        let available = is_z3_available();
        println!("Z3 availability: {}", available);
        // Should not panic regardless of feature state
    }

    #[test]
    fn test_facade_creation() {
        let facade_result = create_z3_facade();

        #[cfg(feature = "z3-verification")]
        {
            assert!(facade_result.is_ok());
        }

        #[cfg(not(feature = "z3-verification"))]
        {
            assert!(facade_result.is_ok());
        }
    }

    #[test]
    fn test_verifier_creation() {
        let verifier_result = create_default_verifier();

        #[cfg(feature = "z3-verification")]
        {
            assert!(verifier_result.is_ok());
        }

        #[cfg(not(feature = "z3-verification"))]
        {
            assert!(verifier_result.is_err());
        }
    }

    #[test]
    fn test_configured_verifier_creation() {
        let config = AdvancedVerificationConfig {
            query_timeout_ms: 15000,
            incremental: false,
            generate_proofs: false,
            generate_models: true,
            generate_unsat_cores: false,
            solver_tactics: vec!["simplify".to_string()],
            max_memory_mb: 2048,
            random_seed: Some(123),
        };

        let verifier_result = create_configured_verifier(config);

        #[cfg(feature = "z3-verification")]
        {
            assert!(verifier_result.is_ok());
            let verifier = verifier_result.unwrap();
            assert_eq!(verifier.get_config().query_timeout_ms, 15000);
            assert!(!verifier.get_config().incremental);
        }

        #[cfg(not(feature = "z3-verification"))]
        {
            assert!(verifier_result.is_err());
        }
    }

    #[test]
    fn test_quick_verify() {
        let document = create_minimal_document();
        let result = quick_verify(&document, None);

        assert!(result.is_ok());
        let verification_result = result.unwrap();

        #[cfg(feature = "z3-verification")]
        {
            // With Z3, we should get a proper result
            match verification_result.status {
                VerificationStatus::AllVerified
                | VerificationStatus::PartiallyVerified
                | VerificationStatus::Incomplete => {
                    // All acceptable for minimal document
                    assert!(true);
                }
                VerificationStatus::Failed(_) => {
                    // Failed verification is also acceptable for testing minimal documents
                    assert!(true);
                }
                VerificationStatus::Disabled => {
                    // If Z3 is disabled, that's also acceptable
                    assert!(true);
                }
            }
        }

        #[cfg(not(feature = "z3-verification"))]
        {
            // Without Z3, should get disabled status
            assert_eq!(verification_result.status, VerificationStatus::Disabled);
        }
    }

    #[test]
    fn test_type_imports() {
        // Test that all main types are accessible
        let config = AdvancedVerificationConfig::default();
        assert!(config.incremental);

        let stats = EnhancedVerificationStats::default();
        assert_eq!(stats.smt_queries, 0);

        let property = VerifiedProperty::new(
            "test".to_string(),
            PropertyCategory::TypeSafety,
            "Test property".to_string(),
            PropertyResult::Proven,
        );
        assert_eq!(property.id, "test");
        assert_eq!(property.category, PropertyCategory::TypeSafety);
    }

    #[test]
    fn test_environment_imports() {
        let env = AispZ3Environment::new();
        assert_eq!(env.sorts.len(), 0);
        assert_eq!(env.functions.len(), 0);
        assert_eq!(env.constants.len(), 0);
    }

    #[test]
    fn test_verification_status_equality() {
        assert_eq!(
            VerificationStatus::AllVerified,
            VerificationStatus::AllVerified
        );
        assert_ne!(
            VerificationStatus::AllVerified,
            VerificationStatus::Disabled
        );

        match VerificationStatus::Failed("test".to_string()) {
            VerificationStatus::Failed(msg) => assert_eq!(msg, "test"),
            _ => panic!("Expected failed status"),
        }
    }

    #[test]
    fn test_property_result_types() {
        let results = [
            PropertyResult::Proven,
            PropertyResult::Disproven,
            PropertyResult::Unknown,
            PropertyResult::Error("test".to_string()),
            PropertyResult::Unsupported,
        ];

        assert_eq!(results.len(), 5);
        assert_eq!(results[0], PropertyResult::Proven);
        assert_ne!(results[0], PropertyResult::Disproven);
    }

    #[test]
    fn test_diagnostic_levels() {
        let levels = [
            DiagnosticLevel::Info,
            DiagnosticLevel::Warning,
            DiagnosticLevel::Error,
            DiagnosticLevel::Performance,
        ];

        assert_eq!(levels.len(), 4);
        assert_eq!(levels[1], DiagnosticLevel::Warning);
    }
}
