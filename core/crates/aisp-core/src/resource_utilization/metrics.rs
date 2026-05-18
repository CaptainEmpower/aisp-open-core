//! Resource Metrics Collection
//!
//! Specialized metrics collection and aggregation for resource monitoring.

use super::types::*;
use crate::error::AispResult;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Metrics collector for resource monitoring
pub struct MetricsCollector {
    /// Active metric streams
    metric_streams: HashMap<ResourceType, MetricStream>,
    /// Collection configuration
    config: MetricsConfig,
}

/// Configuration for metrics collection
#[derive(Debug, Clone)]
pub struct MetricsConfig {
    /// Buffer size for metric history
    pub buffer_size: usize,
    /// Automatic cleanup threshold
    pub cleanup_threshold: Duration,
    /// Enable real-time processing
    pub real_time_processing: bool,
}

/// Stream of metrics for a specific resource
#[derive(Debug)]
pub struct MetricStream {
    /// Resource type
    pub resource_type: ResourceType,
    /// Historical measurements
    pub measurements: Vec<ResourceMeasurement>,
    /// Stream statistics
    pub statistics: StreamStatistics,
    /// Last update time
    pub last_update: Instant,
}

/// Statistics for a metric stream
#[derive(Debug, Clone)]
pub struct StreamStatistics {
    /// Count of measurements
    pub count: usize,
    /// Minimum value
    pub min: f64,
    /// Maximum value
    pub max: f64,
    /// Average value
    pub average: f64,
    /// Standard deviation
    pub std_deviation: f64,
    /// Current trend
    pub trend: UtilizationTrend,
}

impl MetricsCollector {
    /// Create new metrics collector
    pub fn new() -> Self {
        Self {
            metric_streams: HashMap::new(),
            config: MetricsConfig::default(),
        }
    }

    /// Create collector with custom configuration
    pub fn with_config(config: MetricsConfig) -> Self {
        Self {
            metric_streams: HashMap::new(),
            config,
        }
    }

    /// Add measurement to stream
    pub fn add_measurement(
        &mut self,
        resource_type: ResourceType,
        measurement: ResourceMeasurement,
    ) -> AispResult<()> {
        let stream = self
            .metric_streams
            .entry(resource_type.clone())
            .or_insert_with(|| MetricStream::new(resource_type));

        stream.add_measurement(measurement);

        // Perform cleanup if needed
        if stream.measurements.len() > self.config.buffer_size {
            stream.cleanup_old_measurements(self.config.cleanup_threshold);
        }

        // Update statistics if real-time processing is enabled
        if self.config.real_time_processing {
            stream.update_statistics();
        }

        Ok(())
    }

    /// Get current statistics for resource type
    pub fn get_statistics(&self, resource_type: &ResourceType) -> Option<&StreamStatistics> {
        self.metric_streams
            .get(resource_type)
            .map(|stream| &stream.statistics)
    }

    /// Get recent measurements for resource type
    pub fn get_recent_measurements(
        &self,
        resource_type: &ResourceType,
        duration: Duration,
    ) -> Vec<&ResourceMeasurement> {
        if let Some(stream) = self.metric_streams.get(resource_type) {
            let cutoff = Instant::now() - duration;
            stream
                .measurements
                .iter()
                .filter(|m| m.timestamp > cutoff)
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Generate aggregated metrics summary
    pub fn generate_summary(&self) -> MetricsSummary {
        let mut summary = MetricsSummary {
            total_streams: self.metric_streams.len(),
            active_resources: Vec::new(),
            overall_statistics: OverallStatistics::default(),
            resource_correlations: HashMap::new(),
        };

        // Collect active resources and calculate overall statistics
        let mut total_measurements = 0;
        let mut sum_averages = 0.0;
        let mut max_utilization: f64 = 0.0;

        for (resource_type, stream) in &self.metric_streams {
            summary.active_resources.push(resource_type.clone());
            total_measurements += stream.statistics.count;
            sum_averages += stream.statistics.average;
            max_utilization = max_utilization.max(stream.statistics.max);
        }

        if !self.metric_streams.is_empty() {
            summary.overall_statistics.average_utilization =
                sum_averages / self.metric_streams.len() as f64;
            summary.overall_statistics.peak_utilization = max_utilization;
            summary.overall_statistics.total_measurements = total_measurements;
        }

        // Calculate simple correlations between resources
        summary.resource_correlations = self.calculate_correlations();

        summary
    }

    /// Calculate correlations between different resource types
    fn calculate_correlations(&self) -> HashMap<(ResourceType, ResourceType), f64> {
        let mut correlations = HashMap::new();

        let resource_types: Vec<_> = self.metric_streams.keys().cloned().collect();

        for i in 0..resource_types.len() {
            for j in (i + 1)..resource_types.len() {
                let resource_a = &resource_types[i];
                let resource_b = &resource_types[j];

                if let (Some(stream_a), Some(stream_b)) = (
                    self.metric_streams.get(resource_a),
                    self.metric_streams.get(resource_b),
                ) {
                    let correlation = self.calculate_pearson_correlation(stream_a, stream_b);
                    correlations.insert((resource_a.clone(), resource_b.clone()), correlation);
                }
            }
        }

        correlations
    }

    /// Calculate Pearson correlation coefficient between two metric streams
    fn calculate_pearson_correlation(
        &self,
        stream_a: &MetricStream,
        stream_b: &MetricStream,
    ) -> f64 {
        let min_len = stream_a.measurements.len().min(stream_b.measurements.len());
        if min_len < 2 {
            return 0.0;
        }

        let values_a: Vec<f64> = stream_a
            .measurements
            .iter()
            .take(min_len)
            .map(|m| m.utilization)
            .collect();
        let values_b: Vec<f64> = stream_b
            .measurements
            .iter()
            .take(min_len)
            .map(|m| m.utilization)
            .collect();

        let mean_a = values_a.iter().sum::<f64>() / values_a.len() as f64;
        let mean_b = values_b.iter().sum::<f64>() / values_b.len() as f64;

        let mut numerator = 0.0;
        let mut sum_sq_a = 0.0;
        let mut sum_sq_b = 0.0;

        for i in 0..values_a.len() {
            let diff_a = values_a[i] - mean_a;
            let diff_b = values_b[i] - mean_b;
            numerator += diff_a * diff_b;
            sum_sq_a += diff_a * diff_a;
            sum_sq_b += diff_b * diff_b;
        }

        let denominator = (sum_sq_a * sum_sq_b).sqrt();
        if denominator == 0.0 {
            0.0
        } else {
            numerator / denominator
        }
    }
}

impl MetricStream {
    /// Create new metric stream for resource type
    pub fn new(resource_type: ResourceType) -> Self {
        Self {
            resource_type,
            measurements: Vec::new(),
            statistics: StreamStatistics::default(),
            last_update: Instant::now(),
        }
    }

    /// Add measurement to stream
    pub fn add_measurement(&mut self, measurement: ResourceMeasurement) {
        self.measurements.push(measurement);
        self.last_update = Instant::now();
    }

    /// Clean up old measurements based on time threshold
    pub fn cleanup_old_measurements(&mut self, threshold: Duration) {
        let cutoff = Instant::now() - threshold;
        self.measurements.retain(|m| m.timestamp > cutoff);
    }

    /// Update stream statistics
    pub fn update_statistics(&mut self) {
        if self.measurements.is_empty() {
            return;
        }

        let values: Vec<f64> = self.measurements.iter().map(|m| m.utilization).collect();

        let count = values.len();
        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let average = values.iter().sum::<f64>() / count as f64;

        let variance = values.iter().map(|v| (v - average).powi(2)).sum::<f64>() / count as f64;
        let std_deviation = variance.sqrt();

        // Calculate trend
        let trend = if count > 1 {
            let first = values[0];
            let last = values[count - 1];
            let change_rate = (last - first) / count as f64;

            if change_rate > 0.01 {
                UtilizationTrend::Increasing(change_rate)
            } else if change_rate < -0.01 {
                UtilizationTrend::Decreasing(-change_rate)
            } else {
                UtilizationTrend::Stable(std_deviation)
            }
        } else {
            UtilizationTrend::Stable(0.0)
        };

        self.statistics = StreamStatistics {
            count,
            min,
            max,
            average,
            std_deviation,
            trend,
        };
    }
}

/// Summary of all collected metrics
#[derive(Debug, Clone)]
pub struct MetricsSummary {
    /// Total number of metric streams
    pub total_streams: usize,
    /// List of active resource types
    pub active_resources: Vec<ResourceType>,
    /// Overall statistics across all resources
    pub overall_statistics: OverallStatistics,
    /// Correlations between resource types
    pub resource_correlations: HashMap<(ResourceType, ResourceType), f64>,
}

/// Overall statistics across all resources
#[derive(Debug, Clone)]
pub struct OverallStatistics {
    /// Average utilization across all resources
    pub average_utilization: f64,
    /// Peak utilization seen across all resources
    pub peak_utilization: f64,
    /// Total number of measurements
    pub total_measurements: usize,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            buffer_size: 1000,
            cleanup_threshold: Duration::from_secs(3600), // 1 hour
            real_time_processing: true,
        }
    }
}

impl Default for StreamStatistics {
    fn default() -> Self {
        Self {
            count: 0,
            min: 0.0,
            max: 0.0,
            average: 0.0,
            std_deviation: 0.0,
            trend: UtilizationTrend::Stable(0.0),
        }
    }
}

impl Default for OverallStatistics {
    fn default() -> Self {
        Self {
            average_utilization: 0.0,
            peak_utilization: 0.0,
            total_measurements: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_collector_creation() {
        let collector = MetricsCollector::new();
        assert_eq!(collector.config.buffer_size, 1000);
        assert!(collector.config.real_time_processing);
        assert_eq!(collector.metric_streams.len(), 0);
    }

    #[test]
    fn test_add_measurement() {
        let mut collector = MetricsCollector::new();

        let measurement = ResourceMeasurement {
            timestamp: Instant::now(),
            utilization: 0.75,
            raw_value: 75.0,
            capacity: 100.0,
        };

        let result = collector.add_measurement(ResourceType::Memory, measurement);
        assert!(result.is_ok());
        assert_eq!(collector.metric_streams.len(), 1);

        let stream = collector.metric_streams.get(&ResourceType::Memory).unwrap();
        assert_eq!(stream.measurements.len(), 1);
        assert_eq!(stream.measurements[0].utilization, 0.75);
    }

    #[test]
    fn test_stream_statistics_update() {
        let mut stream = MetricStream::new(ResourceType::CPU);

        // Add multiple measurements
        for i in 1..=5 {
            let measurement = ResourceMeasurement {
                timestamp: Instant::now(),
                utilization: i as f64 * 0.1,
                raw_value: i as f64 * 10.0,
                capacity: 100.0,
            };
            stream.add_measurement(measurement);
        }

        stream.update_statistics();

        assert_eq!(stream.statistics.count, 5);
        assert_eq!(stream.statistics.min, 0.1);
        assert_eq!(stream.statistics.max, 0.5);
        assert_eq!(stream.statistics.average, 0.3);
        assert!(matches!(
            stream.statistics.trend,
            UtilizationTrend::Increasing(_)
        ));
    }

    #[test]
    fn test_recent_measurements_filter() {
        let mut collector = MetricsCollector::new();
        let now = Instant::now();

        // Add old measurement
        let old_measurement = ResourceMeasurement {
            timestamp: now - Duration::from_secs(3600),
            utilization: 0.1,
            raw_value: 10.0,
            capacity: 100.0,
        };

        // Add recent measurement
        let recent_measurement = ResourceMeasurement {
            timestamp: now,
            utilization: 0.9,
            raw_value: 90.0,
            capacity: 100.0,
        };

        collector
            .add_measurement(ResourceType::Memory, old_measurement)
            .unwrap();
        collector
            .add_measurement(ResourceType::Memory, recent_measurement)
            .unwrap();

        // Get measurements from last 30 minutes
        let recent =
            collector.get_recent_measurements(&ResourceType::Memory, Duration::from_secs(1800));
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].utilization, 0.9);
    }

    #[test]
    fn test_correlation_calculation() {
        let mut collector = MetricsCollector::new();

        // Add correlated measurements for two resources
        for i in 1..=10 {
            let value = i as f64 * 0.1;

            let measurement_a = ResourceMeasurement {
                timestamp: Instant::now(),
                utilization: value,
                raw_value: value * 100.0,
                capacity: 100.0,
            };

            let measurement_b = ResourceMeasurement {
                timestamp: Instant::now(),
                utilization: value + 0.05, // Slightly offset but correlated
                raw_value: (value + 0.05) * 100.0,
                capacity: 100.0,
            };

            collector
                .add_measurement(ResourceType::Memory, measurement_a)
                .unwrap();
            collector
                .add_measurement(ResourceType::CPU, measurement_b)
                .unwrap();
        }

        let summary = collector.generate_summary();
        assert_eq!(summary.total_streams, 2);
        assert_eq!(summary.active_resources.len(), 2);

        // Should find high correlation
        let correlation = summary
            .resource_correlations
            .get(&(ResourceType::Memory, ResourceType::CPU));
        assert!(correlation.is_some());
        let corr_value = *correlation.unwrap();
        assert!(
            corr_value > 0.8,
            "Expected high correlation, got {}",
            corr_value
        );
    }

    #[test]
    fn test_buffer_cleanup() {
        let config = MetricsConfig {
            buffer_size: 3,
            cleanup_threshold: Duration::from_secs(1),
            real_time_processing: false,
        };

        let mut collector = MetricsCollector::with_config(config);

        // Add more measurements than buffer size
        for i in 1..=5 {
            let measurement = ResourceMeasurement {
                timestamp: Instant::now(),
                utilization: i as f64 * 0.1,
                raw_value: i as f64 * 10.0,
                capacity: 100.0,
            };

            collector
                .add_measurement(ResourceType::Memory, measurement)
                .unwrap();
        }

        let stream = collector.metric_streams.get(&ResourceType::Memory).unwrap();
        assert!(
            stream.measurements.len() <= 3,
            "Buffer should limit measurements"
        );
    }
}
