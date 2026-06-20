//! Canonical Z3 Verification Types
//!
//! Production-ready canonical type definitions for Z3-based verification.
//! This module consolidates all type definitions to eliminate conflicts
//! and provide a single source of truth for Z3 verification types.

use crate::error::*;
use std::collections::HashMap;
use std::time::{Duration, Instant, SystemTime};

/// Production-ready Z3 verification configuration
///
/// **Production Requirements:**
/// - Timeout bounds enforced for resource management
/// - Memory limits to prevent resource exhaustion
/// - Reproducible results via seeded randomization
/// - Comprehensive error recovery mechanisms
#[derive(Debug, Clone, PartialEq)]
pub struct Z3VerificationConfig {
    /// Query timeout in milliseconds [1000, 600_000]
    pub query_timeout_ms: u64,
    /// Enable incremental solving for performance
    pub incremental: bool,
    /// Generate proofs for audit trails
    pub generate_proofs: bool,
    /// Generate models for debugging
    pub generate_models: bool,
    /// Generate unsat cores for minimal conflicts
    pub generate_unsat_cores: bool,
    /// Z3 solver tactics in execution order
    pub solver_tactics: Vec<String>,
    /// Maximum memory usage in MB [256, 32768]
    pub max_memory_mb: usize,
    /// Random seed for reproducible results
    pub random_seed: Option<u64>,
    /// Maximum recursion depth for complex formulas
    pub max_recursion_depth: u32,
    /// Enable parallel solving when available
    pub parallel_solving: bool,
}

impl Default for Z3VerificationConfig {
    fn default() -> Self {
        Self {
            query_timeout_ms: 30000,
            incremental: true,
            generate_proofs: true,
            generate_models: true,
            generate_unsat_cores: true,
            solver_tactics: vec![
                "simplify".to_string(),
                "solve-eqs".to_string(),
                "elim-uncnstr".to_string(),
                "smt".to_string(),
            ],
            max_memory_mb: 4096,   // 4GB default
            random_seed: Some(42), // Reproducible by default
            max_recursion_depth: 1000,
            parallel_solving: false, // Conservative default
        }
    }
}

impl Z3VerificationConfig {
    /// Create production configuration with validation
    pub fn new() -> Self {
        Self::default()
    }

    /// Create configuration with custom timeout
    pub fn with_timeout(mut self, timeout_ms: u64) -> AispResult<Self> {
        if !(1000..=600_000).contains(&timeout_ms) {
            return Err(AispError::validation_error(
                "Timeout must be between 1 second and 10 minutes",
            ));
        }
        self.query_timeout_ms = timeout_ms;
        Ok(self)
    }

    /// Create configuration with custom memory limit
    pub fn with_memory_limit(mut self, memory_mb: usize) -> AispResult<Self> {
        if !(256..=32768).contains(&memory_mb) {
            return Err(AispError::validation_error(
                "Memory limit must be between 256MB and 32GB",
            ));
        }
        self.max_memory_mb = memory_mb;
        Ok(self)
    }

    /// Validate configuration for production use
    pub fn validate(&self) -> AispResult<()> {
        if self.query_timeout_ms < 1000 || self.query_timeout_ms > 600_000 {
            return Err(AispError::validation_error("Invalid timeout range"));
        }

        if self.max_memory_mb < 256 || self.max_memory_mb > 32768 {
            return Err(AispError::validation_error("Invalid memory limit range"));
        }

        if self.max_recursion_depth == 0 || self.max_recursion_depth > 10000 {
            return Err(AispError::validation_error("Invalid recursion depth"));
        }

        // Validate tactics
        let valid_tactics = [
            "simplify",
            "solve-eqs",
            "elim-uncnstr",
            "smt",
            "bit-blast",
            "qe",
            "nlsat",
            "sat",
        ];
        for tactic in &self.solver_tactics {
            if !valid_tactics.contains(&tactic.as_str()) {
                return Err(AispError::validation_error(format!(
                    "Invalid Z3 tactic: {}",
                    tactic
                )));
            }
        }

        Ok(())
    }
}

/// Canonical property verification result
#[derive(Debug, Clone, PartialEq)]
pub enum Z3PropertyResult {
    /// Property has been formally proven
    Proven {
        proof_certificate: String,
        verification_time: Duration,
    },
    /// Property has been formally disproven
    Disproven {
        counterexample: String,
        verification_time: Duration,
    },
    /// Result is unknown (timeout, resource limits, etc.)
    Unknown {
        reason: String,
        partial_progress: f64, // 0.0 to 1.0
    },
    /// Verification error occurred
    Error {
        error_message: String,
        error_code: i32,
    },
    /// Property type not supported by current configuration
    Unsupported {
        property_type: String,
        suggested_alternative: Option<String>,
    },
}

impl Z3PropertyResult {
    /// Check if the result is definitive (proven or disproven)
    pub fn is_definitive(&self) -> bool {
        matches!(
            self,
            Z3PropertyResult::Proven { .. } | Z3PropertyResult::Disproven { .. }
        )
    }

    /// Get the verification time if available
    pub fn verification_time(&self) -> Option<Duration> {
        match self {
            Z3PropertyResult::Proven {
                verification_time, ..
            }
            | Z3PropertyResult::Disproven {
                verification_time, ..
            } => Some(*verification_time),
            _ => None,
        }
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        match self {
            Z3PropertyResult::Proven { .. } => "Property formally proven".to_string(),
            Z3PropertyResult::Disproven { .. } => "Property formally disproven".to_string(),
            Z3PropertyResult::Unknown { reason, .. } => format!("Unknown: {}", reason),
            Z3PropertyResult::Error { error_message, .. } => format!("Error: {}", error_message),
            Z3PropertyResult::Unsupported { property_type, .. } => {
                format!("Unsupported property type: {}", property_type)
            }
        }
    }
}

/// Categories of properties for Z3 verification
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Z3PropertyCategory {
    /// Tri-vector orthogonality constraints
    TriVectorOrthogonality,
    /// Temporal logic properties (LTL, CTL)
    TemporalLogic,
    /// Type safety and well-formedness
    TypeSafety,
    /// Mathematical consistency and soundness
    MathematicalConsistency,
    /// Protocol state machine correctness
    ProtocolCorrectness,
    /// Security properties and invariants
    Security,
    /// Performance bounds and guarantees
    Performance,
    /// Resource utilization constraints
    ResourceBounds,
    /// Custom user-defined properties
    Custom(String),
}

/// Verified property with comprehensive metadata
#[derive(Debug, Clone)]
pub struct Z3VerifiedProperty {
    /// Unique identifier
    pub id: String,
    /// Property category
    pub category: Z3PropertyCategory,
    /// Human-readable description
    pub description: String,
    /// SMT-LIB formula
    pub smt_formula: String,
    /// Verification result
    pub result: Z3PropertyResult,
    /// Verification timestamp
    pub timestamp: SystemTime,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl Z3VerifiedProperty {
    /// Create new verified property
    pub fn new(
        id: String,
        category: Z3PropertyCategory,
        description: String,
        result: Z3PropertyResult,
    ) -> Self {
        Self {
            id,
            category,
            description,
            smt_formula: String::new(),
            result,
            timestamp: SystemTime::now(),
            metadata: HashMap::new(),
        }
    }

    /// Add SMT formula
    pub fn with_formula(mut self, formula: String) -> Self {
        self.smt_formula = formula;
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Check if verification was successful
    pub fn is_verified(&self) -> bool {
        matches!(self.result, Z3PropertyResult::Proven { .. })
    }
}

/// Comprehensive Z3 verification result
#[derive(Debug, Clone)]
pub struct Z3VerificationResult {
    /// Overall verification status
    pub status: Z3VerificationStatus,
    /// Individual property results
    pub properties: Vec<Z3VerifiedProperty>,
    /// Verification statistics
    pub statistics: Z3VerificationStatistics,
    /// Timing breakdown
    pub timing: Z3TimingBreakdown,
    /// Resource usage
    pub resource_usage: Z3ResourceUsage,
    /// Any warnings or notes
    pub diagnostics: Vec<Z3Diagnostic>,
}

/// Overall verification status
#[derive(Debug, Clone, PartialEq)]
pub enum Z3VerificationStatus {
    /// All properties verified successfully
    AllVerified,
    /// Some properties verified, others failed/unknown
    PartiallyVerified {
        verified_count: usize,
        total_count: usize,
    },
    /// No properties could be verified
    Failed(String),
    /// Verification incomplete due to resource limits
    Incomplete {
        completed_count: usize,
        total_count: usize,
        reason: String,
    },
    /// Z3 verification disabled or unavailable
    Disabled,
}

/// Verification statistics
#[derive(Debug, Clone, Default)]
pub struct Z3VerificationStatistics {
    /// Total properties checked
    pub total_properties: usize,
    /// Successfully proven properties
    pub proven_properties: usize,
    /// Disproven properties
    pub disproven_properties: usize,
    /// Unknown results
    pub unknown_results: usize,
    /// Verification errors
    pub error_count: usize,
    /// Total verification time
    pub total_time: Duration,
    /// SMT queries issued
    pub smt_queries: usize,
    /// Cache hit ratio
    pub cache_hit_ratio: f64,
}

/// Detailed timing breakdown
#[derive(Debug, Clone, Default)]
pub struct Z3TimingBreakdown {
    /// Formula preparation time
    pub preparation_time: Duration,
    /// Actual Z3 solving time
    pub solving_time: Duration,
    /// Result processing time
    pub processing_time: Duration,
    /// Cache lookup time
    pub cache_time: Duration,
    /// Overhead time
    pub overhead_time: Duration,
}

/// Resource usage tracking
#[derive(Debug, Clone, Default)]
pub struct Z3ResourceUsage {
    /// Peak memory usage in bytes
    pub peak_memory_bytes: usize,
    /// Average memory usage in bytes
    pub avg_memory_bytes: usize,
    /// CPU time consumed
    pub cpu_time: Duration,
    /// Number of solver instances created
    pub solver_instances: usize,
    /// Z3 internal statistics
    pub z3_stats: HashMap<String, u64>,
}

/// Diagnostic message
#[derive(Debug, Clone)]
pub struct Z3Diagnostic {
    /// Severity level
    pub level: Z3DiagnosticLevel,
    /// Diagnostic message
    pub message: String,
    /// Optional context
    pub context: Option<String>,
    /// Timestamp
    pub timestamp: Instant,
}

/// Diagnostic severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum Z3DiagnosticLevel {
    Info,
    Warning,
    Error,
    Performance,
    Security,
}

/// Z3 counterexample representation
#[derive(Debug, Clone)]
pub struct Z3CounterexampleModel {
    /// Model assignments
    pub assignments: HashMap<String, String>,
    /// Function interpretations
    pub functions: HashMap<String, String>,
    /// Array values
    pub arrays: HashMap<String, String>,
    /// Model evaluation context
    pub context: String,
}

/// Z3 unsat core for minimal conflicts
#[derive(Debug, Clone)]
pub struct Z3UnsatCore {
    /// Core assertion names
    pub core_assertions: Vec<String>,
    /// Minimal conflict explanation
    pub explanation: String,
    /// Core size (smaller is better)
    pub core_size: usize,
}

/// Z3 formal proof representation
#[derive(Debug, Clone)]
pub struct Z3FormalProof {
    /// Proof tree in Lean/Coq format
    pub proof_tree: String,
    /// Proof steps
    pub steps: Vec<Z3ProofStep>,
    /// Proof validation status
    pub validated: bool,
    /// Proof size metrics
    pub size_metrics: Z3ProofMetrics,
}

/// Individual proof step
#[derive(Debug, Clone)]
pub struct Z3ProofStep {
    /// Step type
    pub step_type: String,
    /// Applied rule
    pub rule: String,
    /// Premise references
    pub premises: Vec<usize>,
    /// Conclusion
    pub conclusion: String,
}

/// Proof size and complexity metrics
#[derive(Debug, Clone, Default)]
pub struct Z3ProofMetrics {
    /// Total proof steps
    pub total_steps: usize,
    /// Maximum proof depth
    pub max_depth: usize,
    /// Number of lemmas used
    pub lemma_count: usize,
    /// Proof generation time
    pub generation_time: Duration,
}

impl Z3VerificationResult {
    /// Create disabled verification result for non-Z3 builds
    pub fn new_disabled() -> Self {
        Self {
            status: Z3VerificationStatus::Disabled,
            properties: Vec::new(),
            statistics: Z3VerificationStatistics::default(),
            timing: Z3TimingBreakdown {
                preparation_time: Duration::from_secs(0),
                solving_time: Duration::from_secs(0),
                processing_time: Duration::from_secs(0),
                cache_time: Duration::from_secs(0),
                overhead_time: Duration::from_secs(0),
            },
            resource_usage: Z3ResourceUsage::default(),
            diagnostics: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = Z3VerificationConfig::new();
        assert!(config.validate().is_ok());
        assert_eq!(config.query_timeout_ms, 30000);
        assert!(config.incremental);
    }

    #[test]
    fn test_config_validation() {
        let mut config = Z3VerificationConfig::new();

        // Test timeout validation
        config.query_timeout_ms = 500; // Too low
        assert!(config.validate().is_err());

        config.query_timeout_ms = 700_000; // Too high
        assert!(config.validate().is_err());

        config.query_timeout_ms = 30000; // Valid
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_property_result() {
        let result = Z3PropertyResult::Proven {
            proof_certificate: "test_proof".to_string(),
            verification_time: Duration::from_millis(1000),
        };

        assert!(result.is_definitive());
        assert_eq!(
            result.verification_time(),
            Some(Duration::from_millis(1000))
        );
        assert_eq!(result.description(), "Property formally proven");
    }

    #[test]
    fn test_verified_property() {
        let property = Z3VerifiedProperty::new(
            "test_prop".to_string(),
            Z3PropertyCategory::TypeSafety,
            "Test property".to_string(),
            Z3PropertyResult::Proven {
                proof_certificate: "proof".to_string(),
                verification_time: Duration::from_millis(500),
            },
        );

        assert_eq!(property.id, "test_prop");
        assert!(property.is_verified());
        assert_eq!(property.category, Z3PropertyCategory::TypeSafety);
    }

    #[test]
    fn test_verification_status() {
        let status = Z3VerificationStatus::PartiallyVerified {
            verified_count: 3,
            total_count: 5,
        };

        match status {
            Z3VerificationStatus::PartiallyVerified {
                verified_count,
                total_count,
            } => {
                assert_eq!(verified_count, 3);
                assert_eq!(total_count, 5);
            }
            _ => panic!("Expected PartiallyVerified status"),
        }
    }

    #[test]
    fn test_config_builder() {
        let config = Z3VerificationConfig::new()
            .with_timeout(15000)
            .unwrap()
            .with_memory_limit(2048)
            .unwrap();

        assert_eq!(config.query_timeout_ms, 15000);
        assert_eq!(config.max_memory_mb, 2048);
        assert!(config.validate().is_ok());
    }
}
