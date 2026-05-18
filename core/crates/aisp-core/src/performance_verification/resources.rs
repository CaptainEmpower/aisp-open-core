//! Resource Bound Analysis
//!
//! Focused module for resource utilization and constraint analysis.

use std::collections::HashMap;
use std::time::Duration;

/// Resource bound analysis
#[derive(Debug, Clone)]
pub struct ResourceBoundAnalysis {
    /// CPU resource analysis
    pub cpu_analysis: ResourceUtilization,
    /// Memory resource analysis
    pub memory_analysis: ResourceUtilization,
    /// Network resource analysis
    pub network_analysis: ResourceUtilization,
    /// Storage resource analysis
    pub storage_analysis: ResourceUtilization,
    /// Custom resource analysis
    pub custom_resources: HashMap<String, ResourceUtilization>,
    /// Resource contention analysis
    pub contention_analysis: ResourceContentionAnalysis,
}

/// Resource utilization metrics
#[derive(Debug, Clone)]
pub struct ResourceUtilization {
    /// Resource identifier
    pub resource_id: String,
    /// Current utilization percentage
    pub current_utilization: f64,
    /// Average utilization over analysis period
    pub average_utilization: f64,
    /// Peak utilization observed
    pub peak_utilization: f64,
    /// Utilization threshold warnings
    pub warning_threshold: f64,
    /// Utilization threshold critical
    pub critical_threshold: f64,
    /// Resource capacity information
    pub capacity: ResourceCapacity,
    /// Utilization trend
    pub trend: UtilizationTrend,
}

/// Resource capacity information
#[derive(Debug, Clone)]
pub struct ResourceCapacity {
    /// Total available capacity
    pub total_capacity: u64,
    /// Currently allocated capacity
    pub allocated_capacity: u64,
    /// Available capacity
    pub available_capacity: u64,
    /// Capacity units
    pub units: String,
    /// Capacity is elastic/scalable
    pub scalable: bool,
}

/// Resource utilization trend
#[derive(Debug, Clone)]
pub struct UtilizationTrend {
    /// Trend direction over time
    pub direction: TrendDirection,
    /// Rate of change per hour
    pub hourly_rate: f64,
    /// Predicted time to threshold
    pub time_to_warning: Option<Duration>,
    /// Predicted time to critical
    pub time_to_critical: Option<Duration>,
}

/// Trend direction for resource utilization
#[derive(Debug, Clone, PartialEq)]
pub enum TrendDirection {
    /// Resource usage increasing
    Increasing,
    /// Resource usage stable
    Stable,
    /// Resource usage decreasing
    Decreasing,
    /// Resource usage fluctuating
    Fluctuating,
    /// Trend unclear
    Unknown,
}

/// Resource contention analysis
#[derive(Debug, Clone)]
pub struct ResourceContentionAnalysis {
    /// Contention events detected
    pub contention_events: Vec<ContentionEvent>,
    /// Overall contention level
    pub overall_contention_level: ContentionLevel,
    /// Most contended resources
    pub high_contention_resources: Vec<String>,
    /// Contention impact assessment
    pub impact_assessment: ContentionImpact,
}

/// Resource contention event
#[derive(Debug, Clone)]
pub struct ContentionEvent {
    /// Event identifier
    pub id: String,
    /// Resource involved
    pub resource_id: String,
    /// Timestamp of contention
    pub timestamp: Duration,
    /// Duration of contention
    pub duration: Duration,
    /// Number of competing processes
    pub competing_processes: usize,
    /// Severity of contention
    pub severity: ContentionSeverity,
}

/// Contention severity levels
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ContentionSeverity {
    /// Low contention
    Low,
    /// Medium contention
    Medium,
    /// High contention
    High,
    /// Critical contention
    Critical,
}

/// Overall resource contention level
#[derive(Debug, Clone, PartialEq)]
pub enum ContentionLevel {
    /// No significant contention
    None,
    /// Low level contention
    Low,
    /// Moderate contention
    Moderate,
    /// High contention affecting performance
    High,
    /// Severe contention causing degradation
    Severe,
}

/// Contention impact assessment
#[derive(Debug, Clone)]
pub struct ContentionImpact {
    /// Performance impact percentage
    pub performance_impact: f64,
    /// Throughput reduction percentage
    pub throughput_impact: f64,
    /// Latency increase percentage
    pub latency_impact: f64,
    /// Affected operations
    pub affected_operations: Vec<String>,
}

impl Default for ResourceBoundAnalysis {
    fn default() -> Self {
        Self {
            cpu_analysis: ResourceUtilization::default_cpu(),
            memory_analysis: ResourceUtilization::default_memory(),
            network_analysis: ResourceUtilization::default_network(),
            storage_analysis: ResourceUtilization::default_storage(),
            custom_resources: HashMap::new(),
            contention_analysis: ResourceContentionAnalysis::default(),
        }
    }
}

impl ResourceUtilization {
    /// Create default CPU resource utilization
    pub fn default_cpu() -> Self {
        Self {
            resource_id: "cpu".to_string(),
            current_utilization: 0.0,
            average_utilization: 0.0,
            peak_utilization: 0.0,
            warning_threshold: 70.0,
            critical_threshold: 90.0,
            capacity: ResourceCapacity::default_cpu(),
            trend: UtilizationTrend::default(),
        }
    }

    /// Create default memory resource utilization
    pub fn default_memory() -> Self {
        Self {
            resource_id: "memory".to_string(),
            current_utilization: 0.0,
            average_utilization: 0.0,
            peak_utilization: 0.0,
            warning_threshold: 80.0,
            critical_threshold: 95.0,
            capacity: ResourceCapacity::default_memory(),
            trend: UtilizationTrend::default(),
        }
    }

    /// Create default network resource utilization
    pub fn default_network() -> Self {
        Self {
            resource_id: "network".to_string(),
            current_utilization: 0.0,
            average_utilization: 0.0,
            peak_utilization: 0.0,
            warning_threshold: 75.0,
            critical_threshold: 90.0,
            capacity: ResourceCapacity::default_network(),
            trend: UtilizationTrend::default(),
        }
    }

    /// Create default storage resource utilization
    pub fn default_storage() -> Self {
        Self {
            resource_id: "storage".to_string(),
            current_utilization: 0.0,
            average_utilization: 0.0,
            peak_utilization: 0.0,
            warning_threshold: 85.0,
            critical_threshold: 95.0,
            capacity: ResourceCapacity::default_storage(),
            trend: UtilizationTrend::default(),
        }
    }
}

impl ResourceCapacity {
    fn default_cpu() -> Self {
        Self {
            total_capacity: 100,
            allocated_capacity: 0,
            available_capacity: 100,
            units: "percent".to_string(),
            scalable: false,
        }
    }

    fn default_memory() -> Self {
        Self {
            total_capacity: 8 * 1024 * 1024 * 1024, // 8GB
            allocated_capacity: 0,
            available_capacity: 8 * 1024 * 1024 * 1024,
            units: "bytes".to_string(),
            scalable: true,
        }
    }

    fn default_network() -> Self {
        Self {
            total_capacity: 1_000_000_000, // 1Gbps
            allocated_capacity: 0,
            available_capacity: 1_000_000_000,
            units: "bps".to_string(),
            scalable: false,
        }
    }

    fn default_storage() -> Self {
        Self {
            total_capacity: 500 * 1024 * 1024 * 1024, // 500GB
            allocated_capacity: 0,
            available_capacity: 500 * 1024 * 1024 * 1024,
            units: "bytes".to_string(),
            scalable: true,
        }
    }
}

impl Default for UtilizationTrend {
    fn default() -> Self {
        Self {
            direction: TrendDirection::Unknown,
            hourly_rate: 0.0,
            time_to_warning: None,
            time_to_critical: None,
        }
    }
}

impl Default for ResourceContentionAnalysis {
    fn default() -> Self {
        Self {
            contention_events: Vec::new(),
            overall_contention_level: ContentionLevel::None,
            high_contention_resources: Vec::new(),
            impact_assessment: ContentionImpact::default(),
        }
    }
}

impl Default for ContentionImpact {
    fn default() -> Self {
        Self {
            performance_impact: 0.0,
            throughput_impact: 0.0,
            latency_impact: 0.0,
            affected_operations: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trend_direction() {
        assert_eq!(TrendDirection::Increasing, TrendDirection::Increasing);
        assert_ne!(TrendDirection::Increasing, TrendDirection::Decreasing);
    }

    #[test]
    fn test_contention_severity_ordering() {
        use ContentionSeverity::*;
        assert!(Low < Medium);
        assert!(Medium < High);
        assert!(High < Critical);
    }

    #[test]
    fn test_contention_level() {
        assert_eq!(ContentionLevel::None, ContentionLevel::None);
        assert_ne!(ContentionLevel::None, ContentionLevel::High);
    }

    #[test]
    fn test_default_resource_utilization() {
        let cpu = ResourceUtilization::default_cpu();
        assert_eq!(cpu.resource_id, "cpu");
        assert_eq!(cpu.warning_threshold, 70.0);
        assert_eq!(cpu.critical_threshold, 90.0);

        let memory = ResourceUtilization::default_memory();
        assert_eq!(memory.resource_id, "memory");
        assert_eq!(memory.warning_threshold, 80.0);
    }

    #[test]
    fn test_resource_capacity() {
        let cpu_capacity = ResourceCapacity::default_cpu();
        assert_eq!(cpu_capacity.units, "percent");
        assert!(!cpu_capacity.scalable);

        let memory_capacity = ResourceCapacity::default_memory();
        assert_eq!(memory_capacity.units, "bytes");
        assert!(memory_capacity.scalable);
    }

    #[test]
    fn test_contention_event() {
        let event = ContentionEvent {
            id: "test-event".to_string(),
            resource_id: "cpu".to_string(),
            timestamp: Duration::from_secs(100),
            duration: Duration::from_millis(500),
            competing_processes: 4,
            severity: ContentionSeverity::Medium,
        };

        assert_eq!(event.resource_id, "cpu");
        assert_eq!(event.severity, ContentionSeverity::Medium);
        assert_eq!(event.competing_processes, 4);
    }
}
