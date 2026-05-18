//! Performance Verification Types
//!
//! Core types and structures for performance constraint verification,
//! following SRP with focused type definitions under 300 LOC.

use std::time::Duration;

/// Comprehensive performance constraint analysis result
#[derive(Debug, Clone)]
pub struct PerformanceConstraintAnalysis {
    /// Performance constraint verification results
    pub constraint_verification: ConstraintVerificationResult,
    /// Timing constraint analysis
    pub timing_analysis: super::timing::TimingConstraintAnalysis,
    /// Throughput analysis
    pub throughput_analysis: super::throughput::ThroughputAnalysis,
    /// Resource bound analysis
    pub resource_bound_analysis: super::resources::ResourceBoundAnalysis,
    /// Quality of Service (QoS) analysis
    pub qos_analysis: super::qos::QoSAnalysis,
    /// Service Level Agreement (SLA) compliance
    pub sla_compliance: super::sla::SLACompliance,
    /// Performance optimization opportunities
    pub optimization_opportunities: Vec<PerformanceOptimization>,
    /// Performance degradation analysis
    pub degradation_analysis: super::degradation::PerformanceDegradationAnalysis,
    /// Analysis warnings
    pub warnings: Vec<String>,
}

/// Overall constraint verification result
#[derive(Debug, Clone)]
pub struct ConstraintVerificationResult {
    /// Overall verification status
    pub status: VerificationStatus,
    /// Total constraints verified
    pub total_constraints: usize,
    /// Successfully verified constraints
    pub verified_constraints: usize,
    /// Failed constraint verifications
    pub failed_constraints: usize,
    /// Constraints with warnings
    pub warning_constraints: usize,
    /// Overall compliance score (0.0 to 1.0)
    pub compliance_score: f64,
    /// Detailed verification results
    pub detailed_results: Vec<ConstraintVerificationDetail>,
}

/// Verification status levels
#[derive(Debug, Clone, PartialEq)]
pub enum VerificationStatus {
    /// All constraints verified successfully
    Passed,
    /// Some constraints failed verification
    PartiallyPassed,
    /// Most constraints failed
    Failed,
    /// Verification could not be completed
    Incomplete,
    /// No constraints to verify
    NoConstraints,
}

/// Detailed verification result for a specific constraint
#[derive(Debug, Clone)]
pub struct ConstraintVerificationDetail {
    /// Constraint identifier
    pub constraint_id: String,
    /// Constraint description
    pub description: String,
    /// Constraint type
    pub constraint_type: PerformanceConstraintType,
    /// Verification result
    pub result: ConstraintResult,
    /// Expected value/range
    pub expected: ConstraintValue,
    /// Actual measured value
    pub actual: ConstraintValue,
    /// Deviation from expected
    pub deviation: f64,
    /// Verification confidence
    pub confidence: f64,
    /// Evidence supporting the result
    pub evidence: Vec<String>,
}

/// Types of performance constraints
#[derive(Debug, Clone, PartialEq)]
pub enum PerformanceConstraintType {
    /// Response time constraints
    ResponseTime,
    /// Throughput constraints
    Throughput,
    /// Latency constraints
    Latency,
    /// Resource utilization constraints
    ResourceUtilization,
    /// Availability constraints
    Availability,
    /// Reliability constraints
    Reliability,
    /// Scalability constraints
    Scalability,
    /// Memory usage constraints
    MemoryUsage,
    /// CPU usage constraints
    CPUUsage,
    /// Network bandwidth constraints
    NetworkBandwidth,
    /// Custom constraint
    Custom(String),
}

impl std::fmt::Display for PerformanceConstraintType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PerformanceConstraintType::ResponseTime => write!(f, "response-time"),
            PerformanceConstraintType::Throughput => write!(f, "throughput"),
            PerformanceConstraintType::Latency => write!(f, "latency"),
            PerformanceConstraintType::ResourceUtilization => write!(f, "resource-utilization"),
            PerformanceConstraintType::Availability => write!(f, "availability"),
            PerformanceConstraintType::Reliability => write!(f, "reliability"),
            PerformanceConstraintType::Scalability => write!(f, "scalability"),
            PerformanceConstraintType::MemoryUsage => write!(f, "memory-usage"),
            PerformanceConstraintType::CPUUsage => write!(f, "cpu-usage"),
            PerformanceConstraintType::NetworkBandwidth => write!(f, "network-bandwidth"),
            PerformanceConstraintType::Custom(name) => write!(f, "custom-{}", name),
        }
    }
}

/// Constraint verification result
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintResult {
    /// Constraint is satisfied
    Satisfied,
    /// Constraint is violated
    Violated,
    /// Constraint is marginal (close to violation)
    Marginal,
    /// Constraint cannot be verified
    Unknown,
    /// Constraint is not applicable
    NotApplicable,
}

/// Constraint value representation
#[derive(Debug, Clone)]
pub enum ConstraintValue {
    /// Numeric value with units
    Numeric { value: f64, units: String },
    /// Range of values
    Range { min: f64, max: f64, units: String },
    /// Percentage value
    Percentage(f64),
    /// Boolean value
    Boolean(bool),
    /// Textual value
    Text(String),
    /// Duration value
    Duration(Duration),
}

/// Performance optimization opportunity
#[derive(Debug, Clone)]
pub struct PerformanceOptimization {
    /// Optimization identifier
    pub id: String,
    /// Description of the optimization
    pub description: String,
    /// Expected performance impact
    pub expected_impact: f64,
    /// Implementation complexity
    pub complexity: OptimizationComplexity,
    /// Priority level
    pub priority: OptimizationPriority,
}

/// Optimization complexity levels
#[derive(Debug, Clone, PartialEq)]
pub enum OptimizationComplexity {
    /// Low complexity, easy to implement
    Low,
    /// Medium complexity
    Medium,
    /// High complexity, significant effort required
    High,
    /// Very high complexity, major refactoring needed
    VeryHigh,
}

/// Optimization priority levels
#[derive(Debug, Clone, PartialEq)]
pub enum OptimizationPriority {
    /// Critical - implement immediately
    Critical,
    /// High priority
    High,
    /// Medium priority
    Medium,
    /// Low priority
    Low,
}

/// Performance verification configuration
#[derive(Debug, Clone)]
pub struct PerformanceVerificationConfig {
    /// Enable timing analysis
    pub enable_timing_analysis: bool,
    /// Enable throughput analysis  
    pub enable_throughput_analysis: bool,
    /// Enable resource analysis
    pub enable_resource_analysis: bool,
    /// Enable QoS analysis
    pub enable_qos_analysis: bool,
    /// Enable SLA compliance checking
    pub enable_sla_compliance: bool,
    /// Analysis timeout
    pub analysis_timeout: Duration,
    /// Maximum memory usage for analysis
    pub max_memory_mb: usize,
}

impl Default for PerformanceVerificationConfig {
    fn default() -> Self {
        Self {
            enable_timing_analysis: true,
            enable_throughput_analysis: true,
            enable_resource_analysis: true,
            enable_qos_analysis: true,
            enable_sla_compliance: true,
            analysis_timeout: Duration::from_secs(300),
            max_memory_mb: 1024,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constraint_type_display() {
        assert_eq!(
            PerformanceConstraintType::ResponseTime.to_string(),
            "response-time"
        );
        assert_eq!(
            PerformanceConstraintType::Custom("api".to_string()).to_string(),
            "custom-api"
        );
    }

    #[test]
    fn test_verification_status() {
        let status = VerificationStatus::Passed;
        assert_eq!(status, VerificationStatus::Passed);
        assert_ne!(status, VerificationStatus::Failed);
    }

    #[test]
    fn test_constraint_result() {
        let result = ConstraintResult::Satisfied;
        assert_eq!(result, ConstraintResult::Satisfied);
        assert_ne!(result, ConstraintResult::Violated);
    }

    #[test]
    fn test_optimization_priority_ordering() {
        use OptimizationPriority::*;
        assert!(Critical != High);
        assert!(High != Medium);
        assert!(Medium != Low);
    }

    #[test]
    fn test_default_config() {
        let config = PerformanceVerificationConfig::default();
        assert!(config.enable_timing_analysis);
        assert!(config.enable_throughput_analysis);
        assert_eq!(config.max_memory_mb, 1024);
    }

    #[test]
    fn test_constraint_value_creation() {
        let numeric = ConstraintValue::Numeric {
            value: 100.0,
            units: "ms".to_string(),
        };
        let percentage = ConstraintValue::Percentage(95.5);
        let duration = ConstraintValue::Duration(Duration::from_millis(500));

        match numeric {
            ConstraintValue::Numeric { value, units } => {
                assert_eq!(value, 100.0);
                assert_eq!(units, "ms");
            }
            _ => panic!("Expected Numeric variant"),
        }

        match percentage {
            ConstraintValue::Percentage(val) => assert_eq!(val, 95.5),
            _ => panic!("Expected Percentage variant"),
        }

        match duration {
            ConstraintValue::Duration(d) => assert_eq!(d, Duration::from_millis(500)),
            _ => panic!("Expected Duration variant"),
        }
    }
}
