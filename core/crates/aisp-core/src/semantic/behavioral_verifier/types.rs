//! Behavioral Verification Types and Data Structures
//!
//! Defines core types for behavioral verification including security policies,
//! resource limits, and verification results following SRP architecture.

use std::collections::HashSet;
use std::fmt;
use std::time::{Duration, Instant};

/// Behavioral verification result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BehavioralVerificationResult {
    pub overall_score: f64,
    pub execution_safety_score: f64,
    pub behavioral_consistency_score: f64,
    pub property_compliance_score: f64,
    pub authenticity_score: f64,
    pub execution_results: Vec<ExecutionResult>,
    pub security_assessment: BehavioralSecurityAssessment,
    pub violations: Vec<BehavioralViolation>,
    pub recommendations: Vec<BehavioralRecommendation>,
}

/// Security policy for sandbox execution
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SandboxSecurityPolicy {
    pub allow_file_access: bool,
    pub allow_network_access: bool,
    pub allow_system_calls: bool,
    pub allowed_operations: HashSet<SandboxOperation>,
    pub security_level: SandboxSecurityLevel,
    pub isolation_mode: IsolationMode,
}

/// Resource limits for safe execution
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResourceLimits {
    pub max_execution_time_ms: u64,
    pub max_memory_mb: usize,
    pub max_cpu_usage_percent: f64,
    pub max_iterations: usize,
    pub max_recursion_depth: usize,
    pub max_output_size_bytes: usize,
}

/// Security violation record
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SecurityViolation {
    pub violation_type: SecurityViolationType,
    pub severity: ViolationSeverity,
    pub description: String,
    #[serde(skip)]
    pub timestamp: Instant,
    pub context: String,
}

/// Execution result from behavioral verification
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ExecutionResult {
    pub function_name: String,
    pub input_parameters: Vec<String>,
    pub output: ExecutionOutput,
    #[serde(skip)]
    pub execution_time: Duration,
    pub memory_usage: usize,
    pub security_violations: Vec<SecurityViolation>,
    pub behavior_classification: BehaviorClassification,
}

/// Behavioral security assessment
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BehavioralSecurityAssessment {
    pub threat_level: ThreatLevel,
    pub attack_surface_size: f64,
    pub vulnerability_count: usize,
    pub security_score: f64,
    pub compliance_level: ComplianceLevel,
}

// Enumerations

#[derive(Debug, Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SandboxOperation {
    MathematicalComputation,
    LogicalEvaluation,
    TypeChecking,
    StringManipulation,
    CollectionOperations,
    ConditionalExecution,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SandboxSecurityLevel {
    Strict,     // Maximum security, minimal permissions
    Balanced,   // Balanced security and functionality
    Permissive, // More permissions for complex operations
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum IsolationMode {
    ProcessIsolation, // Separate process execution
    ThreadIsolation,  // Thread-based isolation
    MemoryIsolation,  // Memory space isolation
    VirtualMachine,   // VM-based isolation
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SecurityViolationType {
    UnauthorizedFileAccess,
    NetworkAccessAttempt,
    SystemCallViolation,
    ResourceLimitExceeded,
    SuspiciousBehavior,
    InjectionAttempt,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ViolationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ExecutionOutput {
    Success(String),
    Error(String),
    Timeout,
    MemoryExhausted,
    SecurityViolation(String),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum BehaviorClassification {
    Safe,
    Suspicious,
    Malicious,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, serde::Serialize, serde::Deserialize)]
pub enum ThreatLevel {
    Minimal,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ComplianceLevel {
    NonCompliant,
    PartiallyCompliant,
    FullyCompliant,
    ExceedsCompliance,
}

/// Behavioral violation record
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BehavioralViolation {
    pub violation_type: String,
    pub description: String,
    pub severity: ViolationSeverity,
}

/// Behavioral recommendation for improvements
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BehavioralRecommendation {
    pub priority: String,
    pub action: String,
    pub impact: String,
}

// Coverage tracking
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoverageTracker {
    pub line_coverage: f64,
    pub branch_coverage: f64,
}

// Test statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestStatistics {
    pub total_tests: usize,
    pub passed_tests: usize,
}

impl fmt::Display for BehavioralVerificationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BehavioralVerificationResult {{ overall_score: {:.3}, safety: {:.3}, consistency: {:.3}, compliance: {:.3}, authenticity: {:.3} }}", 
               self.overall_score, self.execution_safety_score, self.behavioral_consistency_score, 
               self.property_compliance_score, self.authenticity_score)
    }
}

// Default implementations for testing and initialization
impl Default for SandboxSecurityPolicy {
    fn default() -> Self {
        let mut allowed_ops = HashSet::new();
        allowed_ops.insert(SandboxOperation::MathematicalComputation);
        allowed_ops.insert(SandboxOperation::LogicalEvaluation);
        allowed_ops.insert(SandboxOperation::TypeChecking);

        Self {
            allow_file_access: false,
            allow_network_access: false,
            allow_system_calls: false,
            allowed_operations: allowed_ops,
            security_level: SandboxSecurityLevel::Strict,
            isolation_mode: IsolationMode::MemoryIsolation,
        }
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_execution_time_ms: 5000,        // 5 seconds
            max_memory_mb: 128,                 // 128 MB
            max_cpu_usage_percent: 80.0,        // 80% CPU
            max_iterations: 10000,              // 10k iterations
            max_recursion_depth: 100,           // 100 levels
            max_output_size_bytes: 1024 * 1024, // 1 MB
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox_policy_creation() {
        let policy = SandboxSecurityPolicy::default();
        assert!(!policy.allow_file_access);
        assert!(!policy.allow_network_access);
        assert_eq!(policy.security_level, SandboxSecurityLevel::Strict);
        assert!(policy
            .allowed_operations
            .contains(&SandboxOperation::MathematicalComputation));
    }

    #[test]
    fn test_resource_limits_creation() {
        let limits = ResourceLimits::default();
        assert_eq!(limits.max_execution_time_ms, 5000);
        assert_eq!(limits.max_memory_mb, 128);
        assert!(limits.max_cpu_usage_percent > 0.0);
    }

    #[test]
    fn test_threat_level_ordering() {
        assert!(ThreatLevel::Minimal < ThreatLevel::Critical);
        // Note: This would need PartialOrd implementation
    }

    #[test]
    fn test_verification_result_display() {
        let result = BehavioralVerificationResult {
            overall_score: 0.95,
            execution_safety_score: 0.98,
            behavioral_consistency_score: 0.92,
            property_compliance_score: 0.96,
            authenticity_score: 0.94,
            execution_results: vec![],
            security_assessment: BehavioralSecurityAssessment {
                threat_level: ThreatLevel::Low,
                attack_surface_size: 0.1,
                vulnerability_count: 0,
                security_score: 0.95,
                compliance_level: ComplianceLevel::FullyCompliant,
            },
            violations: vec![],
            recommendations: vec![],
        };

        let display = format!("{}", result);
        assert!(display.contains("overall_score: 0.950"));
        assert!(display.contains("safety: 0.980"));
    }
}
// Production-ready Default implementations
impl Default for BehavioralVerificationResult {
    fn default() -> Self {
        Self {
            overall_score: 0.0,
            execution_safety_score: 0.0,
            behavioral_consistency_score: 0.0,
            property_compliance_score: 0.0,
            authenticity_score: 0.0,
            execution_results: Vec::new(),
            security_assessment: BehavioralSecurityAssessment::default(),
            violations: Vec::new(),
            recommendations: Vec::new(),
        }
    }
}

impl Default for BehavioralSecurityAssessment {
    fn default() -> Self {
        Self {
            threat_level: ThreatLevel::Minimal,
            attack_surface_size: 0.0,
            vulnerability_count: 0,
            security_score: 0.0,
            compliance_level: ComplianceLevel::NonCompliant,
        }
    }
}

impl Default for SecurityViolation {
    fn default() -> Self {
        Self {
            violation_type: SecurityViolationType::SuspiciousBehavior,
            severity: ViolationSeverity::Low,
            description: String::new(),
            timestamp: Instant::now(),
            context: String::new(),
        }
    }
}

impl Default for ExecutionResult {
    fn default() -> Self {
        Self {
            function_name: String::new(),
            input_parameters: Vec::new(),
            output: ExecutionOutput::Success(String::new()),
            execution_time: Duration::from_secs(0),
            memory_usage: 0,
            security_violations: Vec::new(),
            behavior_classification: BehaviorClassification::Safe,
        }
    }
}
