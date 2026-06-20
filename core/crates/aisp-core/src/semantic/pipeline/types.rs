//! Verification Pipeline Types
//!
//! Shared types and enums for the verification pipeline system
//! Follows SRP by containing only type definitions

use std::collections::HashMap;
use std::time::Duration;

/// Verification execution strategies for pipeline orchestration
///
/// **Contract Invariants:**
/// - Each strategy defines deterministic execution ordering
/// - Adaptive strategies maintain performance > Sequential baseline
/// - Priority-based ensures critical verifications execute first
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub enum ExecutionStrategy {
    Sequential,
    Parallel,
    #[default]
    AdaptiveParallel,
    PriorityBased,
}

/// Failure handling strategies for verification pipeline
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub enum FailureHandlingStrategy {
    FailFast,
    #[default]
    ContinueOnError,
    RetryWithBackoff,
    GracefulDegradation,
}

/// Verification stage enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VerificationStage {
    ParseValidation,
    SemanticAnalysis,
    BehavioralVerification,
    CrossValidation,
    AdversarialTesting,
    ComplianceAudit,
    SecurityEnforcement,
    PerformanceValidation,
}

/// Security violation types for enterprise compliance
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SecurityViolationType {
    UnauthorizedAccess,
    DataLeakage,
    IntegrityBreach,
    AvailabilityImpact,
    ComplianceViolation,
    PolicyViolation,
}

/// Adversarial test results for pipeline integration
#[derive(Debug, Clone)]
pub struct AdversarialTestResults {
    pub attack_resistance_score: f64,
    pub total_attacks: usize,
    pub successful_attacks: usize,
    pub success_rate: f64,
    pub vulnerabilities_found: Vec<String>,
    pub recommendations: Vec<String>,
}

/// Comprehensive verification result with enterprise-grade metrics
///
/// **Contract Invariants:**
/// - All scores ∈ [0.0, 1.0] representing percentage compliance
/// - `overall_success == true` ⟺ all critical stages passed
/// - `execution_time` accurately reflects wall-clock verification duration
/// - `stage_results.len() >= 1` (at least one verification stage executed)
#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub overall_success: bool,
    pub semantic_score: f64,
    pub behavioral_score: f64,
    pub security_score: f64,
    pub compliance_score: f64,
    pub performance_score: f64,
    pub execution_time: Duration,
    pub stage_results: HashMap<VerificationStage, StageResult>,
}

/// Individual stage verification result
#[derive(Debug, Clone)]
pub struct StageResult {
    pub success: bool,
    pub score: f64,
    pub execution_time: Duration,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

// Supporting types
#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    pub name: String,
    pub rules: Vec<String>,
}
#[derive(Debug, Clone)]
pub struct EnforcementRule {
    pub condition: String,
    pub action: String,
}
#[derive(Debug, Clone)]
pub struct ViolationHandler {
    pub handler_type: String,
}
#[derive(Debug, Clone)]
pub struct AuditLogger {
    pub log_level: String,
}
#[derive(Debug, Clone)]
pub struct IncidentResponder {
    pub response_type: String,
}
#[derive(Debug, Clone)]
pub struct ComplianceFramework {
    pub name: String,
    pub version: String,
}
#[derive(Debug, Clone)]
pub struct AuditCheckpoint {
    pub name: String,
    pub status: bool,
}
#[derive(Debug, Clone)]
pub struct CertificationRequirement {
    pub requirement: String,
    pub met: bool,
}
#[derive(Debug, Clone)]
pub struct ResourceManager {
    pub resource_pools: HashMap<String, usize>,
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_strategy_default() {
        assert_eq!(
            ExecutionStrategy::default(),
            ExecutionStrategy::AdaptiveParallel
        );
    }

    #[test]
    fn test_verification_stage_equality() {
        assert_eq!(
            VerificationStage::SemanticAnalysis,
            VerificationStage::SemanticAnalysis
        );
        assert_ne!(
            VerificationStage::ParseValidation,
            VerificationStage::SemanticAnalysis
        );
    }

    #[test]
    fn test_security_violation_types() {
        let violations = [SecurityViolationType::UnauthorizedAccess,
            SecurityViolationType::DataLeakage,
            SecurityViolationType::IntegrityBreach];
        assert_eq!(violations.len(), 3);
    }

    #[test]
    fn test_stage_result_creation() {
        let result = StageResult {
            success: true,
            score: 0.95,
            execution_time: Duration::from_millis(100),
            warnings: vec!["Minor issue".to_string()],
            errors: vec![],
        };
        assert!(result.success);
        assert_eq!(result.score, 0.95);
        assert_eq!(result.warnings.len(), 1);
        assert_eq!(result.errors.len(), 0);
    }
}
