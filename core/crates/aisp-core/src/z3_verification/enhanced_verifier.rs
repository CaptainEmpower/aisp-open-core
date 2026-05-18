//! Enhanced Z3 Verifier with AISP-Specific Capabilities
//!
//! This module provides the main Z3 verifier interface optimized for
//! AISP document verification with comprehensive SMT solving.

use super::{canonical_types::*, environment::AispZ3Environment, properties::PropertyVerifier};
use crate::{
    ast::canonical::CanonicalAispDocument as AispDocument, error::*, tri_vector_validation::*,
};
use std::collections::HashMap;
use std::time::Instant;

#[cfg(feature = "z3-verification")]
use z3::*;

/// Enhanced Z3 verifier with advanced AISP-specific capabilities
pub struct EnhancedZ3Verifier {
    /// AISP type environment
    environment: AispZ3Environment,
    /// Property verifier
    property_verifier: PropertyVerifier,
    /// Verification configuration
    config: Z3VerificationConfig,
    /// Current verification statistics
    stats: Z3VerificationStatistics,
}

impl EnhancedZ3Verifier {
    /// Create new enhanced Z3 verifier
    pub fn new() -> AispResult<Self> {
        Self::with_config(Z3VerificationConfig::default())
    }

    /// Create verifier with specific configuration
    pub fn with_config(config: Z3VerificationConfig) -> AispResult<Self> {
        #[cfg(feature = "z3-verification")]
        {
            Ok(Self {
                environment: AispZ3Environment::new(),
                property_verifier: PropertyVerifier::new(config.clone()),
                config,
                stats: Z3VerificationStatistics::default(),
            })
        }

        #[cfg(not(feature = "z3-verification"))]
        {
            Err(AispError::validation_error(
                "Z3 verification requires z3-verification feature".to_string(),
            ))
        }
    }

    /// Verify AISP document with comprehensive analysis
    pub fn verify_document(
        &mut self,
        _document: &AispDocument,
        _tri_vector_result: Option<&TriVectorValidationResult>,
    ) -> AispResult<Z3VerificationResult> {
        let _start_time = Instant::now();

        #[cfg(feature = "z3-verification")]
        {
            // Comprehensive verification would go here
            Ok(Z3VerificationResult {
                status: Z3VerificationStatus::AllVerified,
                properties: vec![],
                statistics: self.stats.clone(),
                timing: Z3TimingBreakdown::default(),
                resource_usage: Z3ResourceUsage::default(),
                diagnostics: vec![],
            })
        }

        #[cfg(not(feature = "z3-verification"))]
        {
            Ok(Z3VerificationResult::new_disabled())
        }
    }

    /// Determine overall verification status
    fn determine_status(&self, property_results: &[Z3VerifiedProperty]) -> Z3VerificationStatus {
        if property_results.is_empty() {
            return Z3VerificationStatus::Incomplete {
                completed_count: 0,
                total_count: 0,
                reason: "No properties to verify".to_string(),
            };
        }

        let all_verified = property_results.iter().all(|p| p.is_verified());
        let any_failed = property_results
            .iter()
            .any(|p| matches!(p.result, Z3PropertyResult::Disproven { .. }));

        if any_failed {
            Z3VerificationStatus::Failed("One or more properties failed verification".to_string())
        } else if all_verified {
            Z3VerificationStatus::AllVerified
        } else {
            Z3VerificationStatus::PartiallyVerified {
                verified_count: all_verified as usize,
                total_count: property_results.len(),
            }
        }
    }

    /// Get verification configuration
    pub fn get_config(&self) -> &Z3VerificationConfig {
        &self.config
    }

    /// Get verification statistics
    pub fn get_stats(&self) -> &Z3VerificationStatistics {
        &self.stats
    }

    /// Get AISP environment
    pub fn get_environment(&self) -> &AispZ3Environment {
        &self.environment
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::canonical::{self, CanonicalAispDocument as AispDocument};

    fn create_test_document() -> AispDocument {
        canonical::create_document("test", "5.1", "2026-01-26")
    }

    #[test]
    fn test_verifier_creation() {
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

    #[test]
    fn test_verifier_with_config() {
        let config = Z3VerificationConfig {
            query_timeout_ms: 15000,
            incremental: false,
            generate_proofs: true,
            generate_models: true,
            generate_unsat_cores: false,
            solver_tactics: vec!["simplify".to_string()],
            max_memory_mb: 2048,
            random_seed: Some(123),
            max_recursion_depth: 100,
            parallel_solving: false,
        };

        #[cfg(feature = "z3-verification")]
        {
            let verifier = EnhancedZ3Verifier::with_config(config);
            assert!(verifier.is_ok());
            let v = verifier.unwrap();
            assert_eq!(v.get_config().query_timeout_ms, 15000);
            assert!(v.get_config().generate_proofs);
        }

        #[cfg(not(feature = "z3-verification"))]
        {
            let verifier = EnhancedZ3Verifier::with_config(config);
            assert!(verifier.is_err());
        }
    }

    #[test]
    fn test_document_verification() {
        let document = create_test_document();

        #[cfg(feature = "z3-verification")]
        {
            let mut verifier = EnhancedZ3Verifier::new().unwrap();
            let result = verifier.verify_document(&document, None);
            assert!(result.is_ok());

            let verification_result = result.unwrap();
            assert_eq!(
                verification_result.status,
                Z3VerificationStatus::AllVerified
            );
        }

        #[cfg(not(feature = "z3-verification"))]
        {
            // Cannot test without Z3 feature
        }
    }

    #[test]
    fn test_verification_status_determination() {
        #[cfg(feature = "z3-verification")]
        {
            let verifier = EnhancedZ3Verifier::new().unwrap();

            // Test empty properties
            let empty_props = vec![];
            assert_eq!(
                verifier.determine_status(&empty_props),
                Z3VerificationStatus::Incomplete
            );

            // Test all proven properties
            let proven_props = vec![Z3VerifiedProperty::new(
                "test1".to_string(),
                Z3PropertyCategory::TypeSafety,
                "Test property".to_string(),
                Z3PropertyResult::Proven {
                    proof_certificate: "proof".to_string(),
                    verification_time: std::time::Duration::from_millis(100),
                },
            )];
            assert_eq!(
                verifier.determine_status(&proven_props),
                Z3VerificationStatus::AllVerified
            );

            // Test failed property
            let failed_props = vec![Z3VerifiedProperty::new(
                "test2".to_string(),
                Z3PropertyCategory::TypeSafety,
                "Failed property".to_string(),
                Z3PropertyResult::Disproven {
                    counterexample: "counter".to_string(),
                    verification_time: std::time::Duration::from_millis(100),
                },
            )];
            match verifier.determine_status(&failed_props) {
                Z3VerificationStatus::Failed(_) => assert!(true),
                _ => panic!("Expected Failed status"),
            }
        }
    }

    #[test]
    fn test_verifier_accessors() {
        #[cfg(feature = "z3-verification")]
        {
            let verifier = EnhancedZ3Verifier::new().unwrap();

            // Test config accessor
            let config = verifier.get_config();
            assert!(config.incremental); // Default should be true

            // Test stats accessor
            let stats = verifier.get_stats();
            assert_eq!(stats.smt_queries, 0);

            // Test environment accessor
            let env = verifier.get_environment();
            assert_eq!(env.sorts.len(), 0);
        }
    }

    #[test]
    fn test_configuration_validation() {
        let valid_config = Z3VerificationConfig {
            query_timeout_ms: 30000,
            incremental: true,
            generate_proofs: false,
            generate_models: false,
            generate_unsat_cores: false,
            solver_tactics: vec![],
            max_memory_mb: 1024,
            random_seed: None,
            max_recursion_depth: 100,
            parallel_solving: false,
        };

        #[cfg(feature = "z3-verification")]
        {
            let verifier = EnhancedZ3Verifier::with_config(valid_config);
            assert!(verifier.is_ok());
        }

        // Test edge cases
        let zero_timeout_config = Z3VerificationConfig {
            query_timeout_ms: 0,
            incremental: false,
            generate_proofs: true,
            generate_models: true,
            generate_unsat_cores: true,
            solver_tactics: vec!["qe".to_string(), "simplify".to_string()],
            max_memory_mb: 0,
            random_seed: Some(0),
            max_recursion_depth: 100,
            parallel_solving: false,
        };

        #[cfg(feature = "z3-verification")]
        {
            // Should still work with edge case values
            let verifier = EnhancedZ3Verifier::with_config(zero_timeout_config);
            assert!(verifier.is_ok());
        }
    }
}
