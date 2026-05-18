//! Formal Verification Types
//!
//! Core type definitions for formal verification of AISP documents.

use crate::{
    invariant_types::DiscoveredInvariant, proof_types::ProofTree, property_types::PropertyFormula,
    satisfiability_checker::ConstraintModel,
};
use std::collections::{HashMap, HashSet};
use std::time::Duration;

/// Complete formal verification result for an AISP document
#[derive(Debug, Clone, PartialEq)]
pub struct VerificationResult {
    /// Overall verification status
    pub status: VerificationStatus,
    /// Discovered invariants that were verified
    pub verified_invariants: Vec<VerifiedInvariant>,
    /// Generated formal proofs
    pub proofs: Vec<FormalProof>,
    /// Satisfiability model if constraints are satisfiable
    pub model: Option<ConstraintModel>,
    /// Verification statistics
    pub statistics: VerificationStatistics,
    /// Any warnings or issues encountered
    pub warnings: Vec<String>,
}

/// Status of the verification process
#[derive(Debug, Clone, PartialEq)]
pub enum VerificationStatus {
    /// All properties verified successfully
    Verified,
    /// Some properties failed verification
    PartiallyVerified {
        verified_count: usize,
        total_count: usize,
        failures: Vec<VerificationFailure>,
    },
    /// Verification failed completely
    Failed(Vec<VerificationFailure>),
    /// Verification incomplete due to timeouts or resource limits
    Incomplete(String),
    /// Verification could not be performed due to errors
    Error(String),
}

/// An invariant that has been formally verified
#[derive(Debug, Clone, PartialEq)]
pub struct VerifiedInvariant {
    /// The original discovered invariant
    pub invariant: DiscoveredInvariant,
    /// Formal proof that the invariant holds
    pub proof: FormalProof,
    /// Verification confidence score (0.0 to 1.0)
    pub verification_confidence: f64,
    /// Method used for verification
    pub verification_method: VerificationMethod,
    /// Time taken for verification
    pub verification_time: Duration,
}

/// Formal proof with complete derivation
#[derive(Debug, Clone, PartialEq)]
pub struct FormalProof {
    /// Unique identifier for this proof
    pub id: String,
    /// Statement being proved
    pub statement: PropertyFormula,
    /// Proof steps in logical sequence
    pub proof_steps: Vec<ProofStep>,
    /// Proof validation result
    pub validation: ProofValidation,
    /// Time taken to generate the proof
    pub generation_time: Duration,
    /// Proof complexity metrics
    pub complexity: ProofComplexity,
    /// Proof method used
    pub method: VerificationMethod,
}

/// Individual step in a formal proof
#[derive(Debug, Clone, PartialEq)]
pub struct ProofStep {
    /// Step number in sequence
    pub step_number: usize,
    /// Rule applied in this step
    pub rule_name: String,
    /// Premises for this step
    pub premises: Vec<String>,
    /// Conclusion drawn in this step
    pub conclusion: String,
    /// Detailed justification
    pub justification: String,
    /// Dependencies on previous steps
    pub dependencies: Vec<usize>,
}

/// Proof validation result
#[derive(Debug, Clone, PartialEq)]
pub enum ProofValidation {
    /// Proof is mathematically valid
    Valid,
    /// Proof contains errors
    Invalid(String),
    /// Proof validity cannot be determined
    Unknown,
}

/// Different methods used for verification
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VerificationMethod {
    /// Direct constructive proof
    DirectProof,
    /// Proof by contradiction (reductio ad absurdum)
    ProofByContradiction,
    /// Mathematical induction
    InductiveProof,
    /// SMT solver-based verification
    SmtSolverVerification,
    /// Model-based verification
    ModelBasedVerification,
    /// Automated theorem proving
    AutomatedProof,
    /// Hybrid approach combining multiple methods
    HybridVerification(Vec<VerificationMethod>),
}

/// Metrics about proof complexity
#[derive(Debug, Clone, PartialEq)]
pub struct ProofComplexity {
    /// Number of proof steps
    pub steps: usize,
    /// Logical depth of proof
    pub logical_depth: usize,
    /// Number of axioms used
    pub axioms_used: usize,
    /// Number of lemmas required
    pub lemmas_required: usize,
    /// Estimated proof size
    pub size_estimate: usize,
}

/// Verification failure information
#[derive(Debug, Clone, PartialEq)]
pub struct VerificationFailure {
    /// Property that failed verification
    pub property: String,
    /// Reason for failure
    pub reason: FailureReason,
    /// Detailed error message
    pub error_message: String,
    /// Suggested fixes
    pub suggestions: Vec<String>,
    /// Location of failure in source
    pub location: Option<String>,
}

/// Reasons why verification might fail
#[derive(Debug, Clone, PartialEq)]
pub enum FailureReason {
    /// Property is provably false
    Counterexample(String),
    /// Timeout during verification
    Timeout,
    /// Resource exhaustion
    ResourceExhaustion,
    /// Unsupported property type
    UnsupportedProperty,
    /// Invalid or malformed property
    InvalidProperty,
    /// Solver error
    SolverError(String),
    /// Proof generation failed
    ProofGenerationFailed,
}

/// Statistics about the verification process
#[derive(Debug, Clone, PartialEq)]
pub struct VerificationStatistics {
    /// Total verification time
    pub total_time: Duration,
    /// Number of properties checked
    pub properties_checked: usize,
    /// Number of successful verifications
    pub successful_verifications: usize,
    /// Number of failed verifications
    pub failed_verifications: usize,
    /// Time per method breakdown
    pub time_per_method: HashMap<VerificationMethod, Duration>,
    /// Resource usage metrics
    pub resource_usage: ResourceUsageMetrics,
    /// Performance metrics
    pub performance: PerformanceMetrics,
}

/// Resource usage during verification
#[derive(Debug, Clone, PartialEq)]
pub struct ResourceUsageMetrics {
    /// Peak memory usage in bytes
    pub peak_memory: usize,
    /// Average memory usage in bytes
    pub average_memory: usize,
    /// CPU time consumed
    pub cpu_time: Duration,
    /// Number of solver calls
    pub solver_calls: usize,
    /// Cache hit rate
    pub cache_hit_rate: f64,
}

/// Performance metrics for verification
#[derive(Debug, Clone, PartialEq)]
pub struct PerformanceMetrics {
    /// Properties verified per second
    pub properties_per_second: f64,
    /// Average proof generation time
    pub avg_proof_time: Duration,
    /// Success rate percentage
    pub success_rate: f64,
    /// Throughput in proofs per minute
    pub throughput: f64,
}

/// Configuration for formal verification
#[derive(Debug, Clone)]
pub struct VerificationConfig {
    /// Maximum time allowed per property
    pub timeout_per_property: Duration,
    /// Maximum total verification time
    pub total_timeout: Duration,
    /// Maximum memory usage limit
    pub memory_limit: usize,
    /// Enable proof generation
    pub enable_proof_generation: bool,
    /// Enable model generation for satisfiable properties
    pub enable_model_generation: bool,
    /// Verification methods to attempt
    pub methods: Vec<VerificationMethod>,
    /// Enable parallel verification
    pub parallel_verification: bool,
    /// Number of worker threads
    pub worker_threads: usize,
    /// Cache configuration
    pub cache_config: CacheConfig,
}

/// Cache configuration for verification
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Enable result caching
    pub enable_caching: bool,
    /// Maximum cache size in entries
    pub max_cache_size: usize,
    /// Cache entry time-to-live
    pub cache_ttl: Duration,
    /// Enable persistent cache
    pub persistent_cache: bool,
}

/// Verification context for tracking state
#[derive(Debug)]
pub struct VerificationContext {
    /// Current verification configuration
    pub config: VerificationConfig,
    /// Active verification tasks
    pub active_tasks: HashMap<String, VerificationTask>,
    /// Verification cache
    pub cache: VerificationCache,
    /// Statistics collector
    pub statistics: StatisticsCollector,
}

/// Individual verification task
#[derive(Debug, Clone)]
pub struct VerificationTask {
    /// Task identifier
    pub id: String,
    /// Property being verified
    pub property: PropertyFormula,
    /// Verification method being used
    pub method: VerificationMethod,
    /// Task start time
    pub start_time: std::time::Instant,
    /// Current status
    pub status: TaskStatus,
    /// Progress indicator
    pub progress: f64,
}

/// Status of individual verification task
#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed(VerificationResult),
    Failed(VerificationFailure),
    Cancelled,
}

/// Verification result cache
#[derive(Debug)]
pub struct VerificationCache {
    /// Cached verification results
    pub results: HashMap<String, CachedResult>,
    /// Cache statistics
    pub statistics: CacheStatistics,
    /// Cache configuration
    pub config: CacheConfig,
}

/// Cached verification result
#[derive(Debug, Clone)]
pub struct CachedResult {
    /// Cached verification result
    pub result: VerificationResult,
    /// Cache timestamp
    pub timestamp: std::time::Instant,
    /// Hit count
    pub hits: usize,
    /// Cache key hash
    pub key_hash: u64,
}

/// Cache performance statistics
#[derive(Debug, Clone)]
pub struct CacheStatistics {
    /// Total cache hits
    pub hits: usize,
    /// Total cache misses
    pub misses: usize,
    /// Cache size
    pub size: usize,
    /// Hit ratio
    pub hit_ratio: f64,
    /// Average lookup time
    pub avg_lookup_time: Duration,
}

/// Statistics collection and reporting
#[derive(Debug)]
pub struct StatisticsCollector {
    /// Verification events
    pub events: Vec<VerificationEvent>,
    /// Method performance tracking
    pub method_performance: HashMap<VerificationMethod, MethodPerformance>,
    /// Resource usage tracking
    pub resource_tracking: ResourceTracker,
}

/// Verification event for statistics
#[derive(Debug, Clone)]
pub struct VerificationEvent {
    /// Event timestamp
    pub timestamp: std::time::Instant,
    /// Event type
    pub event_type: EventType,
    /// Associated task ID
    pub task_id: String,
    /// Event details
    pub details: String,
    /// Duration (for completed events)
    pub duration: Option<Duration>,
}

/// Types of verification events
#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    TaskStarted,
    TaskCompleted,
    TaskFailed,
    ProofGenerated,
    ModelFound,
    CacheHit,
    CacheMiss,
    ResourceLimit,
    Timeout,
}

/// Performance tracking per method
#[derive(Debug, Clone)]
pub struct MethodPerformance {
    /// Total attempts
    pub attempts: usize,
    /// Successful completions
    pub successes: usize,
    /// Total time spent
    pub total_time: Duration,
    /// Average time per attempt
    pub avg_time: Duration,
    /// Success rate
    pub success_rate: f64,
}

/// Resource usage tracking
#[derive(Debug, Clone)]
pub struct ResourceTracker {
    /// Memory usage samples
    pub memory_samples: Vec<MemorySample>,
    /// CPU usage samples
    pub cpu_samples: Vec<CpuSample>,
    /// Peak resource usage
    pub peak_usage: PeakUsage,
}

/// Memory usage sample
#[derive(Debug, Clone)]
pub struct MemorySample {
    /// Sample timestamp
    pub timestamp: std::time::Instant,
    /// Memory usage in bytes
    pub usage: usize,
    /// Associated task
    pub task_id: Option<String>,
}

/// CPU usage sample
#[derive(Debug, Clone)]
pub struct CpuSample {
    /// Sample timestamp
    pub timestamp: std::time::Instant,
    /// CPU usage percentage
    pub usage_percent: f64,
    /// Associated task
    pub task_id: Option<String>,
}

/// Peak resource usage
#[derive(Debug, Clone)]
pub struct PeakUsage {
    /// Peak memory usage
    pub peak_memory: usize,
    /// Peak CPU usage
    pub peak_cpu: f64,
    /// Time of peak memory usage
    pub peak_memory_time: std::time::Instant,
    /// Time of peak CPU usage
    pub peak_cpu_time: std::time::Instant,
}

impl Default for VerificationConfig {
    fn default() -> Self {
        Self {
            timeout_per_property: Duration::from_secs(30),
            total_timeout: Duration::from_secs(300),
            memory_limit: 1024 * 1024 * 1024, // 1GB
            enable_proof_generation: true,
            enable_model_generation: true,
            methods: vec![
                VerificationMethod::SmtSolverVerification,
                VerificationMethod::AutomatedProof,
                VerificationMethod::DirectProof,
            ],
            parallel_verification: true,
            worker_threads: num_cpus::get(),
            cache_config: CacheConfig::default(),
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enable_caching: true,
            max_cache_size: 1000,
            cache_ttl: Duration::from_secs(3600),
            persistent_cache: false,
        }
    }
}

impl Default for VerificationStatistics {
    fn default() -> Self {
        Self {
            total_time: Duration::from_secs(0),
            properties_checked: 0,
            successful_verifications: 0,
            failed_verifications: 0,
            time_per_method: HashMap::new(),
            resource_usage: ResourceUsageMetrics::default(),
            performance: PerformanceMetrics::default(),
        }
    }
}

impl Default for ResourceUsageMetrics {
    fn default() -> Self {
        Self {
            peak_memory: 0,
            average_memory: 0,
            cpu_time: Duration::from_secs(0),
            solver_calls: 0,
            cache_hit_rate: 0.0,
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            properties_per_second: 0.0,
            avg_proof_time: Duration::from_secs(0),
            success_rate: 0.0,
            throughput: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verification_status_variants() {
        assert!(matches!(
            VerificationStatus::Verified,
            VerificationStatus::Verified
        ));

        let partial = VerificationStatus::PartiallyVerified {
            verified_count: 5,
            total_count: 10,
            failures: vec![],
        };
        assert!(matches!(
            partial,
            VerificationStatus::PartiallyVerified { .. }
        ));
    }

    #[test]
    fn test_verification_method_equality() {
        assert_eq!(
            VerificationMethod::DirectProof,
            VerificationMethod::DirectProof
        );
        assert_ne!(
            VerificationMethod::DirectProof,
            VerificationMethod::InductiveProof
        );

        let hybrid1 = VerificationMethod::HybridVerification(vec![
            VerificationMethod::DirectProof,
            VerificationMethod::SmtSolverVerification,
        ]);
        let hybrid2 = VerificationMethod::HybridVerification(vec![
            VerificationMethod::DirectProof,
            VerificationMethod::SmtSolverVerification,
        ]);
        assert_eq!(hybrid1, hybrid2);
    }

    #[test]
    fn test_proof_validation() {
        assert_eq!(ProofValidation::Valid, ProofValidation::Valid);
        assert_ne!(ProofValidation::Valid, ProofValidation::Unknown);

        let invalid = ProofValidation::Invalid("Logic error".to_string());
        assert!(matches!(invalid, ProofValidation::Invalid(_)));
    }

    #[test]
    fn test_default_configuration() {
        let config = VerificationConfig::default();
        assert_eq!(config.timeout_per_property, Duration::from_secs(30));
        assert!(config.enable_proof_generation);
        assert!(config.parallel_verification);
        assert!(!config.methods.is_empty());
    }

    #[test]
    fn test_task_status_transitions() {
        let pending = TaskStatus::Pending;
        let running = TaskStatus::Running;
        let cancelled = TaskStatus::Cancelled;

        assert_eq!(pending, TaskStatus::Pending);
        assert_ne!(pending, running);
        assert_ne!(running, cancelled);
    }

    #[test]
    fn test_failure_reason_types() {
        let timeout = FailureReason::Timeout;
        let counterex = FailureReason::Counterexample("x = 0".to_string());
        let solver_err = FailureReason::SolverError("Z3 timeout".to_string());

        assert_eq!(timeout, FailureReason::Timeout);
        assert!(matches!(counterex, FailureReason::Counterexample(_)));
        assert!(matches!(solver_err, FailureReason::SolverError(_)));
    }

    #[test]
    fn test_event_type_equality() {
        assert_eq!(EventType::TaskStarted, EventType::TaskStarted);
        assert_ne!(EventType::TaskStarted, EventType::TaskCompleted);
        assert_eq!(EventType::CacheHit, EventType::CacheHit);
    }

    #[test]
    fn test_cache_statistics_calculation() {
        let stats = CacheStatistics {
            hits: 80,
            misses: 20,
            size: 100,
            hit_ratio: 0.8,
            avg_lookup_time: Duration::from_micros(50),
        };

        assert_eq!(stats.hits + stats.misses, 100);
        assert_eq!(stats.hit_ratio, 0.8);
    }
}
