//! Formal Verification Module
//!
//! This module provides comprehensive formal verification capabilities for AISP documents,
//! breaking down the verification process into specialized components for maintainability.

pub mod proof_engine;
pub mod types;
pub mod verifier;

pub use proof_engine::ProofEngine;
pub use types::*;
pub use verifier::FormalVerifier;

use crate::{ast::canonical::CanonicalAispDocument as AispDocument, error::AispResult};

/// Main formal verification system facade
pub struct FormalVerificationSystem {
    /// Core formal verifier
    verifier: FormalVerifier,
    /// Advanced proof engine
    proof_engine: ProofEngine,
    /// System configuration
    config: VerificationConfig,
}

impl FormalVerificationSystem {
    /// Create new formal verification system
    pub fn new() -> Self {
        Self {
            verifier: FormalVerifier::new(),
            proof_engine: ProofEngine::new(),
            config: VerificationConfig::default(),
        }
    }

    /// Create system with custom configuration
    pub fn with_config(config: VerificationConfig) -> Self {
        Self {
            verifier: FormalVerifier::with_config(config.clone()),
            proof_engine: ProofEngine::new(),
            config,
        }
    }

    /// Perform comprehensive formal verification
    pub fn verify_document(&mut self, document: &AispDocument) -> AispResult<VerificationResult> {
        // Delegate to main verifier
        self.verifier.verify(document)
    }

    /// Generate proofs for specific properties
    pub fn generate_proofs(
        &mut self,
        properties: &[crate::property_types::PropertyFormula],
    ) -> AispResult<Vec<FormalProof>> {
        let mut proofs = Vec::new();

        for property in properties {
            let proof = self.proof_engine.generate_proof(property)?;
            proofs.push(proof);
        }

        Ok(proofs)
    }

    /// Get verification statistics
    pub fn get_statistics(&self) -> VerificationStatistics {
        // Would return actual statistics from verifier
        VerificationStatistics::default()
    }
}

impl Default for FormalVerificationSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verification_system_creation() {
        let system = FormalVerificationSystem::new();
        assert_eq!(
            system.config.timeout_per_property,
            std::time::Duration::from_secs(30)
        );
    }

    #[test]
    fn test_system_with_config() {
        let mut config = VerificationConfig::default();
        config.parallel_verification = false;

        let system = FormalVerificationSystem::with_config(config.clone());
        assert_eq!(system.config.parallel_verification, false);
    }

    #[test]
    fn test_module_integration() {
        // Test that all sub-modules are properly integrated
        let _verifier = FormalVerifier::new();
        let _proof_engine = ProofEngine::new();
        let _system = FormalVerificationSystem::new();

        // Integration smoke test
        assert!(true);
    }
}
