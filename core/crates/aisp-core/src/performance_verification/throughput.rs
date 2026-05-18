//! Throughput Analysis
//!
//! Focused module for throughput-related performance metrics and analysis.

use std::time::Duration;

/// Throughput analysis results
#[derive(Debug, Clone)]
pub struct ThroughputAnalysis {
    /// Current throughput
    pub current_throughput: ThroughputMetrics,
    /// Maximum observed throughput
    pub max_throughput: ThroughputMetrics,
    /// Average throughput over analysis period
    pub average_throughput: ThroughputMetrics,
    /// Throughput targets and compliance
    pub targets: Vec<ThroughputTarget>,
    /// Throughput trend over time
    pub trend: ThroughputTrend,
    /// Bottleneck analysis
    pub bottlenecks: Vec<ThroughputBottleneck>,
}

/// Throughput metrics
#[derive(Debug, Clone)]
pub struct ThroughputMetrics {
    /// Requests per second
    pub requests_per_second: f64,
    /// Transactions per second
    pub transactions_per_second: f64,
    /// Data throughput (bytes per second)
    pub bytes_per_second: u64,
    /// Operations per second
    pub operations_per_second: f64,
    /// Success rate percentage
    pub success_rate: f64,
    /// Error rate percentage
    pub error_rate: f64,
}

/// Throughput target
#[derive(Debug, Clone)]
pub struct ThroughputTarget {
    /// Target identifier
    pub id: String,
    /// Target description
    pub description: String,
    /// Target throughput value
    pub target_value: f64,
    /// Target units
    pub units: String,
    /// Current achievement level
    pub achievement_level: f64,
    /// Compliance status
    pub compliance_status: ComplianceStatus,
}

/// Compliance status
#[derive(Debug, Clone, PartialEq)]
pub enum ComplianceStatus {
    /// Target is being met
    Compliant,
    /// Target is close to being met
    NearCompliant,
    /// Target is not being met
    NonCompliant,
    /// Status is unknown
    Unknown,
}

/// Throughput trend analysis
#[derive(Debug, Clone)]
pub struct ThroughputTrend {
    /// Trend direction
    pub direction: TrendDirection,
    /// Rate of change
    pub rate_of_change: f64,
    /// Trend confidence level
    pub confidence: f64,
    /// Projected throughput
    pub projection: ThroughputProjection,
}

/// Trend direction
#[derive(Debug, Clone, PartialEq)]
pub enum TrendDirection {
    /// Throughput is increasing
    Increasing,
    /// Throughput is stable
    Stable,
    /// Throughput is decreasing
    Decreasing,
    /// Throughput is volatile
    Volatile,
    /// Trend is unknown
    Unknown,
}

/// Throughput projection
#[derive(Debug, Clone)]
pub struct ThroughputProjection {
    /// Projected value in 1 hour
    pub one_hour: f64,
    /// Projected value in 1 day
    pub one_day: f64,
    /// Projected value in 1 week
    pub one_week: f64,
    /// Confidence in projections
    pub projection_confidence: f64,
}

/// Throughput bottleneck
#[derive(Debug, Clone)]
pub struct ThroughputBottleneck {
    /// Bottleneck identifier
    pub id: String,
    /// Component causing bottleneck
    pub component: String,
    /// Impact on throughput (percentage reduction)
    pub impact_percentage: f64,
    /// Bottleneck type
    pub bottleneck_type: BottleneckType,
    /// Recommended actions
    pub recommendations: Vec<String>,
}

/// Types of throughput bottlenecks
#[derive(Debug, Clone, PartialEq)]
pub enum BottleneckType {
    /// CPU bottleneck
    CPU,
    /// Memory bottleneck
    Memory,
    /// I/O bottleneck
    IO,
    /// Network bottleneck
    Network,
    /// Database bottleneck
    Database,
    /// Application logic bottleneck
    Application,
    /// External dependency bottleneck
    External,
    /// Configuration bottleneck
    Configuration,
}

impl Default for ThroughputAnalysis {
    fn default() -> Self {
        Self {
            current_throughput: ThroughputMetrics::default(),
            max_throughput: ThroughputMetrics::default(),
            average_throughput: ThroughputMetrics::default(),
            targets: Vec::new(),
            trend: ThroughputTrend::default(),
            bottlenecks: Vec::new(),
        }
    }
}

impl Default for ThroughputMetrics {
    fn default() -> Self {
        Self {
            requests_per_second: 0.0,
            transactions_per_second: 0.0,
            bytes_per_second: 0,
            operations_per_second: 0.0,
            success_rate: 100.0,
            error_rate: 0.0,
        }
    }
}

impl Default for ThroughputTrend {
    fn default() -> Self {
        Self {
            direction: TrendDirection::Unknown,
            rate_of_change: 0.0,
            confidence: 0.0,
            projection: ThroughputProjection::default(),
        }
    }
}

impl Default for ThroughputProjection {
    fn default() -> Self {
        Self {
            one_hour: 0.0,
            one_day: 0.0,
            one_week: 0.0,
            projection_confidence: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compliance_status() {
        assert_eq!(ComplianceStatus::Compliant, ComplianceStatus::Compliant);
        assert_ne!(ComplianceStatus::Compliant, ComplianceStatus::NonCompliant);
    }

    #[test]
    fn test_trend_direction() {
        assert_eq!(TrendDirection::Increasing, TrendDirection::Increasing);
        assert_ne!(TrendDirection::Increasing, TrendDirection::Decreasing);
    }

    #[test]
    fn test_bottleneck_type() {
        assert_eq!(BottleneckType::CPU, BottleneckType::CPU);
        assert_ne!(BottleneckType::CPU, BottleneckType::Memory);
    }

    #[test]
    fn test_default_throughput_analysis() {
        let analysis = ThroughputAnalysis::default();
        assert_eq!(analysis.current_throughput.requests_per_second, 0.0);
        assert_eq!(analysis.trend.direction, TrendDirection::Unknown);
        assert!(analysis.targets.is_empty());
    }

    #[test]
    fn test_default_throughput_metrics() {
        let metrics = ThroughputMetrics::default();
        assert_eq!(metrics.requests_per_second, 0.0);
        assert_eq!(metrics.success_rate, 100.0);
        assert_eq!(metrics.error_rate, 0.0);
    }

    #[test]
    fn test_throughput_target_creation() {
        let target = ThroughputTarget {
            id: "api-rps".to_string(),
            description: "API requests per second".to_string(),
            target_value: 1000.0,
            units: "rps".to_string(),
            achievement_level: 0.95,
            compliance_status: ComplianceStatus::Compliant,
        };

        assert_eq!(target.target_value, 1000.0);
        assert_eq!(target.compliance_status, ComplianceStatus::Compliant);
    }

    #[test]
    fn test_bottleneck_creation() {
        let bottleneck = ThroughputBottleneck {
            id: "db-conn-pool".to_string(),
            component: "Database Connection Pool".to_string(),
            impact_percentage: 25.0,
            bottleneck_type: BottleneckType::Database,
            recommendations: vec!["Increase connection pool size".to_string()],
        };

        assert_eq!(bottleneck.bottleneck_type, BottleneckType::Database);
        assert_eq!(bottleneck.impact_percentage, 25.0);
        assert_eq!(bottleneck.recommendations.len(), 1);
    }
}
