//! Resource Utilization Types
//!
//! Core type definitions for resource utilization analysis.

use crate::error::AispResult;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Comprehensive resource utilization analysis result
#[derive(Debug, Clone)]
pub struct ResourceUtilizationAnalysis {
    /// Overall resource utilization summary
    pub utilization_summary: ResourceUtilizationSummary,
    /// Detailed analysis per resource type
    pub resource_analysis: HashMap<ResourceType, ResourceTypeAnalysis>,
    /// Resource allocation patterns
    pub allocation_patterns: Vec<AllocationPattern>,
    /// Resource bottlenecks
    pub bottlenecks: Vec<ResourceBottleneck>,
    /// Optimization recommendations
    pub optimization_recommendations: Vec<OptimizationRecommendation>,
    /// Resource forecasting
    pub forecasting: ResourceForecasting,
    /// Performance impact analysis
    pub performance_impact: ResourcePerformanceImpact,
    /// Analysis warnings
    pub warnings: Vec<String>,
}

/// Overall resource utilization summary
#[derive(Debug, Clone)]
pub struct ResourceUtilizationSummary {
    /// Overall efficiency score (0.0 to 1.0)
    pub efficiency_score: f64,
    /// Total resource count
    pub total_resources: usize,
    /// Utilized resources count
    pub utilized_resources: usize,
    /// Over-utilized resources count
    pub over_utilized_resources: usize,
    /// Under-utilized resources count
    pub under_utilized_resources: usize,
    /// Average utilization across all resources
    pub average_utilization: f64,
    /// Peak utilization during analysis period
    pub peak_utilization: f64,
    /// Resource stability score
    pub stability_score: f64,
}

/// Types of resources that can be analyzed
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Memory,
    CPU,
    NetworkBandwidth,
    DiskIO,
    NetworkConnections,
    ProcessHandles,
    FileDescriptors,
    ThreadCount,
    Custom(String),
}

/// Detailed analysis for a specific resource type
#[derive(Debug, Clone)]
pub struct ResourceTypeAnalysis {
    /// Resource type being analyzed
    pub resource_type: ResourceType,
    /// Current utilization level (0.0 to 1.0)
    pub current_utilization: f64,
    /// Peak utilization during analysis
    pub peak_utilization: f64,
    /// Average utilization over time
    pub average_utilization: f64,
    /// Resource capacity (units vary by type)
    pub capacity: f64,
    /// Current allocation (units vary by type)
    pub current_allocation: f64,
    /// Utilization trend over time
    pub trend: UtilizationTrend,
    /// Resource-specific metrics
    pub metrics: HashMap<String, f64>,
}

/// Resource allocation patterns
#[derive(Debug, Clone)]
pub struct AllocationPattern {
    /// Pattern identifier
    pub pattern_id: String,
    /// Resources involved in this pattern
    pub resources: Vec<ResourceType>,
    /// Pattern frequency
    pub frequency: f64,
    /// Efficiency of this pattern
    pub efficiency: f64,
    /// Description of the pattern
    pub description: String,
}

/// Resource bottlenecks
#[derive(Debug, Clone)]
pub struct ResourceBottleneck {
    /// Resource causing the bottleneck
    pub resource_type: ResourceType,
    /// Severity level
    pub severity: BottleneckSeverity,
    /// Impact on overall performance
    pub performance_impact: f64,
    /// Suggested resolution
    pub resolution: String,
    /// Estimated time to resolve
    pub estimated_resolution_time: Duration,
}

/// Optimization recommendations
#[derive(Debug, Clone)]
pub struct OptimizationRecommendation {
    /// Target resource type
    pub resource_type: ResourceType,
    /// Type of optimization
    pub optimization_type: OptimizationType,
    /// Estimated performance improvement
    pub estimated_improvement: f64,
    /// Implementation difficulty
    pub difficulty: ImplementationDifficulty,
    /// Recommendation description
    pub description: String,
    /// Priority level
    pub priority: RecommendationPriority,
}

/// Resource forecasting
#[derive(Debug, Clone)]
pub struct ResourceForecasting {
    /// Projected utilization trends
    pub projected_trends: HashMap<ResourceType, UtilizationTrend>,
    /// Projected resource needs
    pub projected_needs: HashMap<ResourceType, f64>,
    /// Confidence in forecasts
    pub forecast_confidence: f64,
    /// Forecasting time horizon
    pub time_horizon: Duration,
}

/// Performance impact analysis
#[derive(Debug, Clone)]
pub struct ResourcePerformanceImpact {
    /// Overall performance score
    pub overall_performance_score: f64,
    /// Resource-specific performance impacts
    pub resource_impacts: HashMap<ResourceType, f64>,
    /// Bottleneck performance costs
    pub bottleneck_costs: HashMap<ResourceType, f64>,
    /// Optimization potential
    pub optimization_potential: f64,
}

/// Individual resource measurement
#[derive(Debug, Clone)]
pub struct ResourceMeasurement {
    /// Timestamp of measurement
    pub timestamp: Instant,
    /// Resource utilization value
    pub utilization: f64,
    /// Raw resource value
    pub raw_value: f64,
    /// Resource capacity at time of measurement
    pub capacity: f64,
}

/// Utilization trend over time
#[derive(Debug, Clone)]
pub enum UtilizationTrend {
    Increasing(f64),    // Rate of increase
    Decreasing(f64),    // Rate of decrease
    Stable(f64),        // Variance level
    Volatile(f64),      // Volatility measure
    Cyclical(Duration), // Cycle period
}

/// Bottleneck severity levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BottleneckSeverity {
    Minor,
    Moderate,
    Severe,
    Critical,
}

/// Types of optimization
#[derive(Debug, Clone)]
pub enum OptimizationType {
    ScaleUp,
    ScaleDown,
    LoadBalance,
    Caching,
    Compression,
    Parallelization,
    Algorithm,
    DataStructure,
    Custom(String),
}

/// Implementation difficulty levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ImplementationDifficulty {
    Trivial,
    Easy,
    Moderate,
    Hard,
    Expert,
}

/// Recommendation priority levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RecommendationPriority {
    Low,
    Medium,
    High,
    Critical,
}

impl Default for ResourceUtilizationSummary {
    fn default() -> Self {
        Self {
            efficiency_score: 0.0,
            total_resources: 0,
            utilized_resources: 0,
            over_utilized_resources: 0,
            under_utilized_resources: 0,
            average_utilization: 0.0,
            peak_utilization: 0.0,
            stability_score: 0.0,
        }
    }
}

impl Default for ResourceForecasting {
    fn default() -> Self {
        Self {
            projected_trends: HashMap::new(),
            projected_needs: HashMap::new(),
            forecast_confidence: 0.0,
            time_horizon: Duration::from_secs(0),
        }
    }
}

impl Default for ResourcePerformanceImpact {
    fn default() -> Self {
        Self {
            overall_performance_score: 0.0,
            resource_impacts: HashMap::new(),
            bottleneck_costs: HashMap::new(),
            optimization_potential: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_type_variants() {
        let resources = [ResourceType::Memory,
            ResourceType::CPU,
            ResourceType::NetworkBandwidth,
            ResourceType::DiskIO,
            ResourceType::Custom("TestResource".to_string())];

        assert_eq!(resources.len(), 5);
        assert_eq!(resources[0], ResourceType::Memory);
        assert!(matches!(resources[4], ResourceType::Custom(_)));
    }

    #[test]
    fn test_bottleneck_severity_ordering() {
        assert!(BottleneckSeverity::Minor < BottleneckSeverity::Critical);
        assert!(BottleneckSeverity::Moderate < BottleneckSeverity::Severe);
    }

    #[test]
    fn test_default_implementations() {
        let summary = ResourceUtilizationSummary::default();
        assert_eq!(summary.efficiency_score, 0.0);
        assert_eq!(summary.total_resources, 0);

        let forecasting = ResourceForecasting::default();
        assert_eq!(forecasting.forecast_confidence, 0.0);
        assert_eq!(forecasting.time_horizon, Duration::from_secs(0));

        let performance = ResourcePerformanceImpact::default();
        assert_eq!(performance.overall_performance_score, 0.0);
        assert_eq!(performance.optimization_potential, 0.0);
    }

    #[test]
    fn test_utilization_trend_variants() {
        let trends = [UtilizationTrend::Increasing(0.1),
            UtilizationTrend::Decreasing(0.05),
            UtilizationTrend::Stable(0.02),
            UtilizationTrend::Volatile(0.5),
            UtilizationTrend::Cyclical(Duration::from_secs(3600))];

        assert_eq!(trends.len(), 5);
        assert!(matches!(trends[0], UtilizationTrend::Increasing(_)));
        assert!(matches!(trends[4], UtilizationTrend::Cyclical(_)));
    }

    #[test]
    fn test_optimization_type_custom() {
        let opt = OptimizationType::Custom("Neural Network Optimization".to_string());
        assert!(matches!(opt, OptimizationType::Custom(_)));
    }
}
