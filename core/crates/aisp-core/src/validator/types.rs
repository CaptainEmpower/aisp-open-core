//! Validation Configuration and Result Types
//!
//! Defines core types for AISP document validation configuration,
//! results, and quality assessment following SRP architecture.

use crate::anti_drift::AntiDriftValidationResult;
use crate::ast::canonical::CanonicalAispDocument as AispDocument;
use crate::error::*;
use crate::ghost_intent_validation::GhostIntentValidationResult;
use crate::hebbian_learning::HebbianValidationResult;
use crate::rossnet_scoring::RossNetValidationResult;
use crate::semantic::{DeepVerificationResult, QualityTier};
use crate::tri_vector_validation::TriVectorValidationResult;
use crate::z3_verification::canonical_types::Z3VerificationResult;
use crate::{AISP_VERSION, MAX_DOCUMENT_SIZE};
use std::time::Duration;

/// Validation configuration options
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    /// Maximum document size in bytes
    pub max_document_size: usize,
    /// Strict mode enables additional validations
    pub strict_mode: bool,
    /// Include detailed timing information
    pub include_timing: bool,
    /// Include AST in result (for debugging)
    pub include_ast: bool,
    /// Include symbol statistics
    pub include_symbol_stats: bool,
    /// Enable formal verification with Z3
    pub enable_formal_verification: bool,
    /// Z3 verification timeout
    pub z3_timeout: Duration,
    /// Enable tri-vector signal validation
    pub enable_trivector_validation: bool,
    /// Enable enhanced Z3 verification
    pub enable_enhanced_z3: bool,
    /// Enable ghost intent search validation
    pub enable_ghost_intent_validation: bool,
    /// Enable RossNet scoring validation
    pub enable_rossnet_scoring: bool,
    /// Enable Hebbian learning constraint validation
    pub enable_hebbian_learning: bool,
    /// Enable anti-drift protocol verification
    pub enable_anti_drift: bool,
    /// Strict formal verification mode - failures cause validation to fail instead of warnings
    pub strict_formal_verification: bool,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        use crate::z3_verification::Z3VerificationFacade;

        Self {
            max_document_size: MAX_DOCUMENT_SIZE,
            strict_mode: false,
            include_timing: false,
            include_ast: false,
            include_symbol_stats: false,
            enable_formal_verification: false,
            z3_timeout: Duration::from_secs(30),
            enable_trivector_validation: true,
            enable_enhanced_z3: Z3VerificationFacade::is_available(),
            enable_ghost_intent_validation: true,
            enable_rossnet_scoring: true,
            enable_hebbian_learning: true,
            enable_anti_drift: true,
            strict_formal_verification: true, // Default to strict mode for sound verification
        }
    }
}

/// Complete validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Document is valid
    pub valid: bool,
    /// Quality tier
    pub tier: QualityTier,
    /// Tier symbol
    pub tier_symbol: String,
    /// Tier name
    pub tier_name: String,
    /// Tier numeric value (0-4)
    pub tier_value: u8,
    /// Semantic density (δ)
    pub delta: f64,
    /// Pure symbol density
    pub pure_density: f64,
    /// Calculated ambiguity level
    pub ambiguity: f64,
    /// Validation mode
    pub mode: String,
    /// Document size in bytes
    pub document_size: usize,
    /// Parsing timing
    pub parse_time: Option<Duration>,
    /// Semantic analysis timing
    pub semantic_time: Option<Duration>,
    /// Total validation timing
    pub total_time: Option<Duration>,
    /// Parsed AST (if requested)
    pub ast: Option<AispDocument>,
    /// Semantic analysis details
    pub semantic_analysis: Option<DeepVerificationResult>,
    /// Formal verification results
    pub formal_verification: Option<DeepVerificationResult>,
    /// Tri-vector validation results
    pub trivector_validation: Option<TriVectorValidationResult>,
    /// Enhanced Z3 verification results
    pub enhanced_z3_verification: Option<Z3VerificationResult>,
    /// Ghost intent search validation results
    pub ghost_intent_validation: Option<GhostIntentValidationResult>,
    /// RossNet scoring validation results
    pub rossnet_validation: Option<RossNetValidationResult>,
    /// Hebbian learning constraint validation results
    pub hebbian_validation: Option<HebbianValidationResult>,
    /// Anti-drift protocol verification results
    pub anti_drift_validation: Option<AntiDriftValidationResult>,
    /// All warnings collected
    pub warnings: Vec<AispWarning>,
    /// Error details (if validation failed)
    pub error: Option<AispError>,
}

impl ValidationResult {
    /// Create a failed validation result
    pub fn failed(error: AispError, document_size: usize) -> Self {
        Self {
            valid: false,
            tier: QualityTier::Reject,
            tier_symbol: "⊘".to_string(),
            tier_name: "Reject".to_string(),
            tier_value: 0,
            delta: 0.0,
            pure_density: 0.0,
            ambiguity: 1.0,
            mode: "rust-pure".to_string(),
            document_size,
            parse_time: None,
            semantic_time: None,
            total_time: None,
            ast: None,
            semantic_analysis: None,
            formal_verification: None,
            trivector_validation: None,
            enhanced_z3_verification: None,
            ghost_intent_validation: None,
            rossnet_validation: None,
            hebbian_validation: None,
            anti_drift_validation: None,
            warnings: Vec::new(),
            error: Some(error),
        }
    }

    /// Create a successful validation result
    pub fn success(
        analysis: DeepVerificationResult,
        document_size: usize,
        parse_time: Duration,
        semantic_time: Duration,
        ast: Option<AispDocument>,
        formal_verification: Option<DeepVerificationResult>,
        trivector_validation: Option<TriVectorValidationResult>,
        enhanced_z3_verification: Option<Z3VerificationResult>,
        ghost_intent_validation: Option<GhostIntentValidationResult>,
        rossnet_validation: Option<RossNetValidationResult>,
        hebbian_validation: Option<HebbianValidationResult>,
        anti_drift_validation: Option<AntiDriftValidationResult>,
    ) -> Self {
        Self {
            valid: analysis.valid(),
            tier: analysis.tier(),
            tier_symbol: analysis.tier().symbol().to_string(),
            tier_name: analysis.tier().name().to_string(),
            tier_value: analysis.tier().value(),
            delta: analysis.delta(),
            pure_density: analysis.pure_density(),
            ambiguity: analysis.ambiguity(),
            mode: "rust-pure".to_string(),
            document_size,
            parse_time: Some(parse_time),
            semantic_time: Some(semantic_time),
            total_time: Some(parse_time + semantic_time),
            ast,
            semantic_analysis: Some(analysis.clone()),
            formal_verification,
            trivector_validation,
            enhanced_z3_verification,
            ghost_intent_validation,
            rossnet_validation,
            hebbian_validation,
            anti_drift_validation,
            warnings: analysis
                .warnings()
                .into_iter()
                .map(|w| AispWarning::warning(w))
                .collect(),
            error: None,
        }
    }

    /// Check if document has acceptable quality
    pub fn is_acceptable(&self) -> bool {
        self.valid && self.tier != QualityTier::Reject && self.ambiguity < 0.02
    }

    /// Get detailed error message
    pub fn error_message(&self) -> Option<String> {
        self.error.as_ref().map(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_config_default() {
        let config = ValidationConfig::default();
        assert_eq!(config.max_document_size, MAX_DOCUMENT_SIZE);
        assert!(!config.strict_mode);
        assert!(config.strict_formal_verification);
        assert!(config.enable_trivector_validation);
    }

    #[test]
    fn test_validation_result_failed() {
        let error = AispError::validation_error("Test error");
        let result = ValidationResult::failed(error, 1000);

        assert!(!result.valid);
        assert_eq!(result.tier, QualityTier::Reject);
        assert_eq!(result.document_size, 1000);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_validation_result_acceptable() {
        let error = AispError::validation_error("Test error");
        let mut result = ValidationResult::failed(error, 1000);

        // Should not be acceptable when failed
        assert!(!result.is_acceptable());

        // Make it acceptable
        result.valid = true;
        result.tier = QualityTier::Gold;
        result.ambiguity = 0.01;
        assert!(result.is_acceptable());

        // High ambiguity should make it unacceptable
        result.ambiguity = 0.05;
        assert!(!result.is_acceptable());
    }

    #[test]
    fn test_validation_config_clone() {
        let config1 = ValidationConfig::default();
        let config2 = config1.clone();
        assert_eq!(config1.max_document_size, config2.max_document_size);
        assert_eq!(config1.strict_mode, config2.strict_mode);
    }
}
