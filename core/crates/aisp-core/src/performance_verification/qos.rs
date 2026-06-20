//! Quality of Service Analysis
//!
//! Focused module for QoS metrics and service quality analysis.

/// Quality of Service analysis
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct QoSAnalysis {
    /// Service availability metrics
    pub availability: AvailabilityMetrics,
    /// Service reliability metrics
    pub reliability: ReliabilityMetrics,
    /// Performance quality metrics
    pub performance_quality: PerformanceQuality,
    /// Service level indicators
    pub service_level_indicators: Vec<ServiceLevelIndicator>,
    /// Quality degradation indicators
    pub degradation_indicators: QualityDegradationIndicators,
}

/// Service availability metrics
#[derive(Debug, Clone)]
pub struct AvailabilityMetrics {
    /// Uptime percentage
    pub uptime_percentage: f64,
    /// Total uptime duration
    pub total_uptime: std::time::Duration,
    /// Total downtime duration
    pub total_downtime: std::time::Duration,
    /// Number of outages
    pub outage_count: usize,
    /// Average outage duration
    pub average_outage_duration: std::time::Duration,
    /// Maximum outage duration
    pub max_outage_duration: std::time::Duration,
}

/// Service reliability metrics
#[derive(Debug, Clone)]
pub struct ReliabilityMetrics {
    /// Mean Time Between Failures (MTBF)
    pub mtbf: std::time::Duration,
    /// Mean Time To Recovery (MTTR)
    pub mttr: std::time::Duration,
    /// Failure rate per hour
    pub failure_rate: f64,
    /// Recovery success rate
    pub recovery_success_rate: f64,
    /// Service reliability score
    pub reliability_score: f64,
}

/// Performance quality metrics
#[derive(Debug, Clone)]
pub struct PerformanceQuality {
    /// Response time quality score
    pub response_time_score: f64,
    /// Throughput quality score
    pub throughput_score: f64,
    /// Latency quality score
    pub latency_score: f64,
    /// Overall performance quality score
    pub overall_score: f64,
    /// Quality trend
    pub quality_trend: QualityTrend,
}

/// Quality trend analysis
#[derive(Debug, Clone)]
pub struct QualityTrend {
    /// Trend direction
    pub direction: TrendDirection,
    /// Quality change rate per day
    pub daily_change_rate: f64,
    /// Trend confidence level
    pub confidence: f64,
}

/// Trend direction
#[derive(Debug, Clone, PartialEq)]
pub enum TrendDirection {
    /// Quality improving
    Improving,
    /// Quality stable
    Stable,
    /// Quality degrading
    Degrading,
    /// Quality volatile
    Volatile,
    /// Trend unknown
    Unknown,
}

/// Service Level Indicator
#[derive(Debug, Clone)]
pub struct ServiceLevelIndicator {
    /// SLI identifier
    pub id: String,
    /// SLI name
    pub name: String,
    /// Current value
    pub current_value: f64,
    /// Target value
    pub target_value: f64,
    /// Measurement units
    pub units: String,
    /// Compliance status
    pub compliance: SLICompliance,
    /// Measurement window
    pub measurement_window: std::time::Duration,
}

/// SLI compliance status
#[derive(Debug, Clone, PartialEq)]
pub enum SLICompliance {
    /// Meeting target
    Meeting,
    /// Close to target
    NearTarget,
    /// Missing target
    Missing,
    /// Severely missing target
    SeverelyMissing,
}

/// Quality degradation indicators
#[derive(Debug, Clone)]
pub struct QualityDegradationIndicators {
    /// Response time degradation
    pub response_time_degradation: f64,
    /// Throughput degradation
    pub throughput_degradation: f64,
    /// Error rate increase
    pub error_rate_increase: f64,
    /// Overall degradation score
    pub overall_degradation: f64,
    /// Degradation alerts
    pub alerts: Vec<QualityAlert>,
}

/// Quality alert
#[derive(Debug, Clone)]
pub struct QualityAlert {
    /// Alert identifier
    pub id: String,
    /// Alert level
    pub level: AlertLevel,
    /// Alert message
    pub message: String,
    /// Affected metric
    pub affected_metric: String,
    /// Current value
    pub current_value: f64,
    /// Threshold value
    pub threshold_value: f64,
}

/// Alert severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum AlertLevel {
    /// Information alert
    Info,
    /// Warning alert
    Warning,
    /// Error alert
    Error,
    /// Critical alert
    Critical,
}


impl Default for AvailabilityMetrics {
    fn default() -> Self {
        Self {
            uptime_percentage: 100.0,
            total_uptime: std::time::Duration::ZERO,
            total_downtime: std::time::Duration::ZERO,
            outage_count: 0,
            average_outage_duration: std::time::Duration::ZERO,
            max_outage_duration: std::time::Duration::ZERO,
        }
    }
}

impl Default for ReliabilityMetrics {
    fn default() -> Self {
        Self {
            mtbf: std::time::Duration::from_secs(86400), // 24 hours
            mttr: std::time::Duration::from_secs(300),   // 5 minutes
            failure_rate: 0.0,
            recovery_success_rate: 100.0,
            reliability_score: 1.0,
        }
    }
}

impl Default for PerformanceQuality {
    fn default() -> Self {
        Self {
            response_time_score: 1.0,
            throughput_score: 1.0,
            latency_score: 1.0,
            overall_score: 1.0,
            quality_trend: QualityTrend::default(),
        }
    }
}

impl Default for QualityTrend {
    fn default() -> Self {
        Self {
            direction: TrendDirection::Stable,
            daily_change_rate: 0.0,
            confidence: 1.0,
        }
    }
}

impl Default for QualityDegradationIndicators {
    fn default() -> Self {
        Self {
            response_time_degradation: 0.0,
            throughput_degradation: 0.0,
            error_rate_increase: 0.0,
            overall_degradation: 0.0,
            alerts: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trend_direction() {
        assert_eq!(TrendDirection::Improving, TrendDirection::Improving);
        assert_ne!(TrendDirection::Improving, TrendDirection::Degrading);
    }

    #[test]
    fn test_sli_compliance() {
        assert_eq!(SLICompliance::Meeting, SLICompliance::Meeting);
        assert_ne!(SLICompliance::Meeting, SLICompliance::Missing);
    }

    #[test]
    fn test_alert_level() {
        assert_eq!(AlertLevel::Warning, AlertLevel::Warning);
        assert_ne!(AlertLevel::Warning, AlertLevel::Critical);
    }

    #[test]
    fn test_default_availability_metrics() {
        let metrics = AvailabilityMetrics::default();
        assert_eq!(metrics.uptime_percentage, 100.0);
        assert_eq!(metrics.outage_count, 0);
    }

    #[test]
    fn test_default_reliability_metrics() {
        let metrics = ReliabilityMetrics::default();
        assert_eq!(metrics.recovery_success_rate, 100.0);
        assert_eq!(metrics.reliability_score, 1.0);
        assert_eq!(metrics.failure_rate, 0.0);
    }

    #[test]
    fn test_service_level_indicator() {
        let sli = ServiceLevelIndicator {
            id: "response-time-p99".to_string(),
            name: "99th percentile response time".to_string(),
            current_value: 150.0,
            target_value: 200.0,
            units: "ms".to_string(),
            compliance: SLICompliance::Meeting,
            measurement_window: std::time::Duration::from_secs(300),
        };

        assert_eq!(sli.compliance, SLICompliance::Meeting);
        assert!(sli.current_value < sli.target_value);
    }
}
