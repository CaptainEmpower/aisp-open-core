//! Performance Degradation Analysis
//!
//! Focused module for detecting and analyzing performance degradation.

use std::time::Duration;

/// Performance degradation analysis
#[derive(Debug, Clone, Default)]
pub struct PerformanceDegradationAnalysis {
    /// Overall degradation assessment
    pub overall_assessment: DegradationAssessment,
    /// Degradation patterns detected
    pub degradation_patterns: Vec<DegradationPattern>,
    /// Performance regression analysis
    pub regression_analysis: RegressionAnalysis,
    /// Degradation alerts
    pub alerts: Vec<DegradationAlert>,
    /// Recovery recommendations
    pub recovery_recommendations: Vec<RecoveryRecommendation>,
}

/// Overall degradation assessment
#[derive(Debug, Clone)]
pub struct DegradationAssessment {
    /// Degradation severity level
    pub severity: DegradationSeverity,
    /// Overall degradation score (0.0 = no degradation, 1.0 = severe)
    pub degradation_score: f64,
    /// Primary degradation factors
    pub primary_factors: Vec<DegradationFactor>,
    /// Degradation timeline
    pub timeline: DegradationTimeline,
    /// Affected services
    pub affected_services: Vec<String>,
}

/// Degradation severity levels
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum DegradationSeverity {
    /// No significant degradation
    None,
    /// Minor degradation
    Minor,
    /// Moderate degradation
    Moderate,
    /// Significant degradation
    Significant,
    /// Severe degradation
    Severe,
    /// Critical degradation
    Critical,
}

/// Factors contributing to degradation
#[derive(Debug, Clone)]
pub struct DegradationFactor {
    /// Factor identifier
    pub id: String,
    /// Factor name
    pub name: String,
    /// Contribution percentage to overall degradation
    pub contribution_percentage: f64,
    /// Factor category
    pub category: FactorCategory,
    /// Impact description
    pub impact_description: String,
}

/// Categories of degradation factors
#[derive(Debug, Clone, PartialEq)]
pub enum FactorCategory {
    /// Resource exhaustion
    ResourceExhaustion,
    /// Load increase
    LoadIncrease,
    /// Configuration change
    ConfigurationChange,
    /// Software regression
    SoftwareRegression,
    /// Infrastructure issue
    Infrastructure,
    /// External dependency
    ExternalDependency,
    /// Data growth
    DataGrowth,
    /// Unknown factor
    Unknown,
}

/// Timeline of degradation events
#[derive(Debug, Clone)]
pub struct DegradationTimeline {
    /// When degradation started
    pub degradation_start: Option<Duration>,
    /// When degradation was detected
    pub detection_time: Option<Duration>,
    /// Current duration of degradation
    pub current_duration: Duration,
    /// Key degradation milestones
    pub milestones: Vec<DegradationMilestone>,
}

/// Degradation milestone event
#[derive(Debug, Clone)]
pub struct DegradationMilestone {
    /// Milestone timestamp
    pub timestamp: Duration,
    /// Milestone description
    pub description: String,
    /// Degradation level at milestone
    pub degradation_level: f64,
    /// Associated metric
    pub metric: String,
}

/// Pattern of performance degradation
#[derive(Debug, Clone)]
pub struct DegradationPattern {
    /// Pattern identifier
    pub id: String,
    /// Pattern type
    pub pattern_type: PatternType,
    /// Pattern description
    pub description: String,
    /// Confidence in pattern detection
    pub confidence: f64,
    /// Pattern recurrence frequency
    pub frequency: PatternFrequency,
    /// Associated metrics
    pub associated_metrics: Vec<String>,
}

/// Types of degradation patterns
#[derive(Debug, Clone, PartialEq)]
pub enum PatternType {
    /// Gradual degradation over time
    GradualDecline,
    /// Sudden drop in performance
    SuddenDrop,
    /// Periodic degradation cycles
    PeriodicCycles,
    /// Memory leak pattern
    MemoryLeak,
    /// Resource saturation pattern
    ResourceSaturation,
    /// Load-related degradation
    LoadRelated,
    /// Time-based degradation
    TimeBased,
}

/// Pattern recurrence frequency
#[derive(Debug, Clone, PartialEq)]
pub enum PatternFrequency {
    /// One-time occurrence
    OneTime,
    /// Rare occurrences
    Rare,
    /// Occasional occurrences
    Occasional,
    /// Regular occurrences
    Regular,
    /// Frequent occurrences
    Frequent,
}

/// Performance regression analysis
#[derive(Debug, Clone)]
pub struct RegressionAnalysis {
    /// Statistical regression model
    pub regression_model: RegressionModel,
    /// Baseline performance metrics
    pub baseline_metrics: BaselineMetrics,
    /// Current performance metrics
    pub current_metrics: CurrentMetrics,
    /// Performance delta analysis
    pub delta_analysis: DeltaAnalysis,
    /// Regression confidence
    pub regression_confidence: f64,
}

/// Regression model information
#[derive(Debug, Clone)]
pub struct RegressionModel {
    /// Model type used
    pub model_type: String,
    /// Model accuracy
    pub accuracy: f64,
    /// Model prediction horizon
    pub prediction_horizon: Duration,
    /// Model confidence interval
    pub confidence_interval: (f64, f64),
}

/// Baseline performance metrics
#[derive(Debug, Clone)]
pub struct BaselineMetrics {
    /// Baseline response time
    pub response_time: f64,
    /// Baseline throughput
    pub throughput: f64,
    /// Baseline error rate
    pub error_rate: f64,
    /// Baseline resource utilization
    pub resource_utilization: f64,
    /// Baseline measurement period
    pub measurement_period: Duration,
}

/// Current performance metrics
#[derive(Debug, Clone)]
pub struct CurrentMetrics {
    /// Current response time
    pub response_time: f64,
    /// Current throughput
    pub throughput: f64,
    /// Current error rate
    pub error_rate: f64,
    /// Current resource utilization
    pub resource_utilization: f64,
    /// Current measurement period
    pub measurement_period: Duration,
}

/// Performance delta analysis
#[derive(Debug, Clone)]
pub struct DeltaAnalysis {
    /// Response time change percentage
    pub response_time_delta: f64,
    /// Throughput change percentage
    pub throughput_delta: f64,
    /// Error rate change percentage
    pub error_rate_delta: f64,
    /// Resource utilization change percentage
    pub resource_utilization_delta: f64,
    /// Overall performance change
    pub overall_performance_delta: f64,
}

/// Degradation alert
#[derive(Debug, Clone)]
pub struct DegradationAlert {
    /// Alert identifier
    pub id: String,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Alert message
    pub message: String,
    /// Alert timestamp
    pub timestamp: Duration,
    /// Affected component
    pub affected_component: String,
    /// Metric that triggered alert
    pub triggering_metric: String,
    /// Recommended actions
    pub recommended_actions: Vec<String>,
}

/// Alert severity levels
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum AlertSeverity {
    /// Informational alert
    Info,
    /// Warning alert
    Warning,
    /// Error alert
    Error,
    /// Critical alert
    Critical,
    /// Emergency alert
    Emergency,
}

/// Recovery recommendation
#[derive(Debug, Clone)]
pub struct RecoveryRecommendation {
    /// Recommendation identifier
    pub id: String,
    /// Recommendation priority
    pub priority: RecommendationPriority,
    /// Recommended action
    pub action: String,
    /// Expected impact
    pub expected_impact: f64,
    /// Implementation complexity
    pub complexity: ImplementationComplexity,
    /// Estimated time to implement
    pub estimated_time: Duration,
}

/// Priority levels for recommendations
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum RecommendationPriority {
    /// Low priority
    Low,
    /// Medium priority
    Medium,
    /// High priority
    High,
    /// Critical priority
    Critical,
    /// Emergency priority
    Emergency,
}

/// Implementation complexity levels
#[derive(Debug, Clone, PartialEq)]
pub enum ImplementationComplexity {
    /// Very easy to implement
    VeryEasy,
    /// Easy to implement
    Easy,
    /// Moderate complexity
    Moderate,
    /// Complex to implement
    Complex,
    /// Very complex to implement
    VeryComplex,
}

impl Default for DegradationAssessment {
    fn default() -> Self {
        Self {
            severity: DegradationSeverity::None,
            degradation_score: 0.0,
            primary_factors: Vec::new(),
            timeline: DegradationTimeline::default(),
            affected_services: Vec::new(),
        }
    }
}

impl Default for DegradationTimeline {
    fn default() -> Self {
        Self {
            degradation_start: None,
            detection_time: None,
            current_duration: Duration::ZERO,
            milestones: Vec::new(),
        }
    }
}

impl Default for RegressionAnalysis {
    fn default() -> Self {
        Self {
            regression_model: RegressionModel::default(),
            baseline_metrics: BaselineMetrics::default(),
            current_metrics: CurrentMetrics::default(),
            delta_analysis: DeltaAnalysis::default(),
            regression_confidence: 0.0,
        }
    }
}

impl Default for RegressionModel {
    fn default() -> Self {
        Self {
            model_type: "linear".to_string(),
            accuracy: 0.0,
            prediction_horizon: Duration::from_secs(3600), // 1 hour
            confidence_interval: (0.0, 0.0),
        }
    }
}

impl Default for BaselineMetrics {
    fn default() -> Self {
        Self {
            response_time: 0.0,
            throughput: 0.0,
            error_rate: 0.0,
            resource_utilization: 0.0,
            measurement_period: Duration::from_secs(3600),
        }
    }
}

impl Default for CurrentMetrics {
    fn default() -> Self {
        Self {
            response_time: 0.0,
            throughput: 0.0,
            error_rate: 0.0,
            resource_utilization: 0.0,
            measurement_period: Duration::from_secs(3600),
        }
    }
}

impl Default for DeltaAnalysis {
    fn default() -> Self {
        Self {
            response_time_delta: 0.0,
            throughput_delta: 0.0,
            error_rate_delta: 0.0,
            resource_utilization_delta: 0.0,
            overall_performance_delta: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degradation_severity_ordering() {
        use DegradationSeverity::*;
        assert!(None < Minor);
        assert!(Minor < Moderate);
        assert!(Moderate < Significant);
        assert!(Significant < Severe);
        assert!(Severe < Critical);
    }

    #[test]
    fn test_alert_severity_ordering() {
        use AlertSeverity::*;
        assert!(Info < Warning);
        assert!(Warning < Error);
        assert!(Error < Critical);
        assert!(Critical < Emergency);
    }

    #[test]
    fn test_recommendation_priority_ordering() {
        use RecommendationPriority::*;
        assert!(Low < Medium);
        assert!(Medium < High);
        assert!(High < Critical);
        assert!(Critical < Emergency);
    }

    #[test]
    fn test_pattern_type() {
        assert_eq!(PatternType::GradualDecline, PatternType::GradualDecline);
        assert_ne!(PatternType::GradualDecline, PatternType::SuddenDrop);
    }

    #[test]
    fn test_pattern_frequency() {
        assert_eq!(PatternFrequency::Regular, PatternFrequency::Regular);
        assert_ne!(PatternFrequency::Regular, PatternFrequency::Rare);
    }

    #[test]
    fn test_factor_category() {
        assert_eq!(
            FactorCategory::ResourceExhaustion,
            FactorCategory::ResourceExhaustion
        );
        assert_ne!(
            FactorCategory::ResourceExhaustion,
            FactorCategory::LoadIncrease
        );
    }

    #[test]
    fn test_implementation_complexity() {
        assert_eq!(
            ImplementationComplexity::Easy,
            ImplementationComplexity::Easy
        );
        assert_ne!(
            ImplementationComplexity::Easy,
            ImplementationComplexity::Complex
        );
    }

    #[test]
    fn test_default_degradation_analysis() {
        let analysis = PerformanceDegradationAnalysis::default();
        assert_eq!(
            analysis.overall_assessment.severity,
            DegradationSeverity::None
        );
        assert_eq!(analysis.overall_assessment.degradation_score, 0.0);
        assert!(analysis.degradation_patterns.is_empty());
    }
}
