//! Performance Monitor
//!
//! Performance tracking and optimization for verification pipeline
//! Implements SRP by focusing solely on performance monitoring

use super::types::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Optimization engine for performance tuning
#[derive(Debug, Clone)]
pub struct OptimizationEngine {
    pub optimization_strategies: Vec<String>,
}

/// Alerting system for performance notifications
#[derive(Debug, Clone)]
pub struct AlertingSystem {
    pub alert_channels: Vec<String>,
}

/// Profiling data collection
#[derive(Debug, Clone)]
pub struct ProfilingData {
    pub profiling_samples: Vec<String>,
}

/// Performance monitor for tracking verification pipeline metrics
pub struct PerformanceMonitor {
    stage_timings: HashMap<VerificationStage, Duration>,
    resource_usage: ResourceUsage,
    performance_thresholds: PerformanceThresholds,
    optimization_engine: OptimizationEngine,
    alerting_system: AlertingSystem,
    profiling_data: ProfilingData,
}

/// Resource usage tracking
#[derive(Debug, Clone)]
pub struct ResourceUsage {
    pub memory_usage_mb: usize,
    pub cpu_usage_percent: f64,
    pub disk_io_mb: usize,
    pub network_io_mb: usize,
    pub peak_memory_mb: usize,
}

/// Performance thresholds for alerting
#[derive(Debug, Clone)]
pub struct PerformanceThresholds {
    pub max_verification_time_ms: u64,
    pub max_memory_usage_mb: usize,
    pub max_cpu_usage_percent: f64,
    pub min_throughput_docs_per_sec: f64,
}

impl PerformanceMonitor {
    /// Create new performance monitor with default thresholds
    pub fn new() -> Self {
        Self {
            stage_timings: HashMap::new(),
            resource_usage: ResourceUsage::default(),
            performance_thresholds: PerformanceThresholds::default(),
            optimization_engine: OptimizationEngine {
                optimization_strategies: vec![],
            },
            alerting_system: AlertingSystem {
                alert_channels: vec![],
            },
            profiling_data: ProfilingData {
                profiling_samples: vec![],
            },
        }
    }

    /// Create monitor with detailed metrics collection
    pub fn with_detailed_metrics() -> Self {
        let mut monitor = Self::new();
        monitor.enable_detailed_profiling();
        monitor
    }

    /// Record stage completion
    pub fn record_stage_completion(&mut self, stage: VerificationStage) {
        let duration = Duration::from_millis(100); // Mock timing
        self.stage_timings.insert(stage, duration);
        self.update_resource_usage();
        self.check_performance_thresholds();
    }

    /// Record stage timing
    pub fn record_stage_timing(&mut self, stage: VerificationStage, duration: Duration) {
        // Check if timing exceeds threshold before inserting
        if duration.as_millis() > self.performance_thresholds.max_verification_time_ms as u128 {
            self.trigger_performance_alert(&stage, duration);
        }

        self.stage_timings.insert(stage, duration);
    }

    /// Get performance metrics
    pub fn get_performance_metrics(&self) -> PerformanceMetrics {
        PerformanceMetrics {
            total_execution_time: self.calculate_total_time(),
            stage_timings: self.stage_timings.clone(),
            resource_usage: self.resource_usage.clone(),
            throughput: self.calculate_throughput(),
            efficiency_score: self.calculate_efficiency_score(),
        }
    }

    /// Calculate performance score
    pub fn calculate_performance_score(&self) -> f64 {
        let timing_score = self.calculate_timing_score();
        let resource_score = self.calculate_resource_score();
        let efficiency_score = self.calculate_efficiency_score();

        // Weighted average
        (timing_score * 0.4) + (resource_score * 0.3) + (efficiency_score * 0.3)
    }

    /// Start performance profiling for a stage
    pub fn start_stage_profiling(&mut self, stage: VerificationStage) -> StageProfiler {
        StageProfiler::new(stage)
    }

    /// Generate performance report
    pub fn generate_performance_report(&self) -> PerformanceReport {
        PerformanceReport {
            overall_score: self.calculate_performance_score(),
            stage_analysis: self.analyze_stage_performance(),
            resource_analysis: self.analyze_resource_usage(),
            optimization_suggestions: self.generate_optimization_suggestions(),
            performance_trends: self.analyze_performance_trends(),
        }
    }

    /// Enable detailed profiling
    fn enable_detailed_profiling(&mut self) {
        self.profiling_data.profiling_samples.extend(vec![
            "Memory allocation patterns".to_string(),
            "CPU usage spikes".to_string(),
            "I/O bottlenecks".to_string(),
        ]);
    }

    /// Update resource usage (mock implementation)
    fn update_resource_usage(&mut self) {
        self.resource_usage.memory_usage_mb = 128;
        self.resource_usage.cpu_usage_percent = 45.0;
        self.resource_usage.disk_io_mb = 10;
        self.resource_usage.network_io_mb = 5;

        if self.resource_usage.memory_usage_mb > self.resource_usage.peak_memory_mb {
            self.resource_usage.peak_memory_mb = self.resource_usage.memory_usage_mb;
        }
    }

    /// Check performance thresholds
    fn check_performance_thresholds(&self) {
        if self.resource_usage.memory_usage_mb > self.performance_thresholds.max_memory_usage_mb {
            self.trigger_memory_alert();
        }

        if self.resource_usage.cpu_usage_percent > self.performance_thresholds.max_cpu_usage_percent
        {
            self.trigger_cpu_alert();
        }
    }

    /// Calculate total execution time
    fn calculate_total_time(&self) -> Duration {
        self.stage_timings.values().sum()
    }

    /// Calculate throughput (mock implementation)
    fn calculate_throughput(&self) -> f64 {
        1.5 // Documents per second
    }

    /// Calculate timing score
    fn calculate_timing_score(&self) -> f64 {
        let total_time = self.calculate_total_time();
        let target_time =
            Duration::from_millis(self.performance_thresholds.max_verification_time_ms);

        if total_time <= target_time {
            1.0
        } else {
            (target_time.as_millis() as f64) / (total_time.as_millis() as f64)
        }
    }

    /// Calculate resource utilization score
    fn calculate_resource_score(&self) -> f64 {
        let memory_score = if self.resource_usage.memory_usage_mb
            <= self.performance_thresholds.max_memory_usage_mb
        {
            1.0
        } else {
            (self.performance_thresholds.max_memory_usage_mb as f64)
                / (self.resource_usage.memory_usage_mb as f64)
        };

        let cpu_score = if self.resource_usage.cpu_usage_percent
            <= self.performance_thresholds.max_cpu_usage_percent
        {
            1.0
        } else {
            self.performance_thresholds.max_cpu_usage_percent
                / self.resource_usage.cpu_usage_percent
        };

        (memory_score + cpu_score) / 2.0
    }

    /// Calculate efficiency score
    fn calculate_efficiency_score(&self) -> f64 {
        let throughput = self.calculate_throughput();
        if throughput >= self.performance_thresholds.min_throughput_docs_per_sec {
            1.0
        } else {
            throughput / self.performance_thresholds.min_throughput_docs_per_sec
        }
    }

    /// Analyze stage performance
    fn analyze_stage_performance(&self) -> Vec<StagePerformanceAnalysis> {
        self.stage_timings
            .iter()
            .map(|(stage, duration)| StagePerformanceAnalysis {
                stage: stage.clone(),
                execution_time: *duration,
                performance_rating: if duration.as_millis() < 500 {
                    "Excellent"
                } else {
                    "Good"
                }
                .to_string(),
                bottlenecks: vec![],
            })
            .collect()
    }

    /// Analyze resource usage patterns
    fn analyze_resource_usage(&self) -> ResourceAnalysis {
        ResourceAnalysis {
            memory_efficiency: (self.performance_thresholds.max_memory_usage_mb as f64
                - self.resource_usage.memory_usage_mb as f64)
                / self.performance_thresholds.max_memory_usage_mb as f64,
            cpu_efficiency: (self.performance_thresholds.max_cpu_usage_percent
                - self.resource_usage.cpu_usage_percent)
                / self.performance_thresholds.max_cpu_usage_percent,
            io_patterns: vec!["Sequential access".to_string()],
            optimization_opportunities: vec!["Consider memory pooling".to_string()],
        }
    }

    /// Generate optimization suggestions
    fn generate_optimization_suggestions(&self) -> Vec<String> {
        let mut suggestions = Vec::new();

        if self.resource_usage.memory_usage_mb > 200 {
            suggestions
                .push("Consider implementing memory pooling to reduce allocations".to_string());
        }

        if self.resource_usage.cpu_usage_percent > 70.0 {
            suggestions.push("Optimize CPU-intensive verification algorithms".to_string());
        }

        let total_time = self.calculate_total_time();
        if total_time.as_millis() > 2000 {
            suggestions.push(
                "Consider parallel processing for independent verification stages".to_string(),
            );
        }

        if suggestions.is_empty() {
            suggestions.push("Performance is within acceptable limits".to_string());
        }

        suggestions
    }

    /// Analyze performance trends (mock implementation)
    fn analyze_performance_trends(&self) -> Vec<String> {
        vec![
            "Performance stable over time".to_string(),
            "Memory usage trending upward".to_string(),
            "CPU utilization within normal range".to_string(),
        ]
    }

    /// Trigger performance alert
    fn trigger_performance_alert(&self, stage: &VerificationStage, duration: Duration) {
        // Mock alert implementation
    }

    /// Trigger memory usage alert
    fn trigger_memory_alert(&self) {
        // Mock alert implementation
    }

    /// Trigger CPU usage alert
    fn trigger_cpu_alert(&self) {
        // Mock alert implementation
    }
}

/// Stage profiler for detailed timing
pub struct StageProfiler {
    stage: VerificationStage,
    start_time: Instant,
}

impl StageProfiler {
    fn new(stage: VerificationStage) -> Self {
        Self {
            stage,
            start_time: Instant::now(),
        }
    }

    pub fn finish(self) -> (VerificationStage, Duration) {
        (self.stage, self.start_time.elapsed())
    }
}

/// Performance metrics collection
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub total_execution_time: Duration,
    pub stage_timings: HashMap<VerificationStage, Duration>,
    pub resource_usage: ResourceUsage,
    pub throughput: f64,
    pub efficiency_score: f64,
}

/// Comprehensive performance report
#[derive(Debug, Clone)]
pub struct PerformanceReport {
    pub overall_score: f64,
    pub stage_analysis: Vec<StagePerformanceAnalysis>,
    pub resource_analysis: ResourceAnalysis,
    pub optimization_suggestions: Vec<String>,
    pub performance_trends: Vec<String>,
}

/// Stage performance analysis
#[derive(Debug, Clone)]
pub struct StagePerformanceAnalysis {
    pub stage: VerificationStage,
    pub execution_time: Duration,
    pub performance_rating: String,
    pub bottlenecks: Vec<String>,
}

/// Resource usage analysis
#[derive(Debug, Clone)]
pub struct ResourceAnalysis {
    pub memory_efficiency: f64,
    pub cpu_efficiency: f64,
    pub io_patterns: Vec<String>,
    pub optimization_opportunities: Vec<String>,
}

impl Default for ResourceUsage {
    fn default() -> Self {
        Self {
            memory_usage_mb: 0,
            cpu_usage_percent: 0.0,
            disk_io_mb: 0,
            network_io_mb: 0,
            peak_memory_mb: 0,
        }
    }
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            max_verification_time_ms: 5000,
            max_memory_usage_mb: 512,
            max_cpu_usage_percent: 80.0,
            min_throughput_docs_per_sec: 1.0,
        }
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_monitor_creation() {
        let monitor = PerformanceMonitor::new();
        assert_eq!(monitor.stage_timings.len(), 0);
        assert_eq!(monitor.resource_usage.memory_usage_mb, 0);
    }

    #[test]
    fn test_stage_timing_recording() {
        let mut monitor = PerformanceMonitor::new();
        let duration = Duration::from_millis(250);

        monitor.record_stage_timing(VerificationStage::SemanticAnalysis, duration);

        assert_eq!(
            monitor.stage_timings[&VerificationStage::SemanticAnalysis],
            duration
        );
    }

    #[test]
    fn test_performance_score_calculation() {
        let mut monitor = PerformanceMonitor::new();
        monitor.record_stage_timing(
            VerificationStage::ParseValidation,
            Duration::from_millis(100),
        );
        monitor.record_stage_timing(
            VerificationStage::SemanticAnalysis,
            Duration::from_millis(200),
        );

        let score = monitor.calculate_performance_score();
        assert!(score > 0.0);
        assert!(score <= 1.0);
    }

    #[test]
    fn test_stage_profiler() {
        let profiler = StageProfiler::new(VerificationStage::BehavioralVerification);
        std::thread::sleep(Duration::from_millis(1));

        let (stage, duration) = profiler.finish();
        assert_eq!(stage, VerificationStage::BehavioralVerification);
        assert!(duration.as_millis() >= 1);
    }

    #[test]
    fn test_performance_report_generation() {
        let mut monitor = PerformanceMonitor::new();
        monitor.record_stage_timing(
            VerificationStage::ParseValidation,
            Duration::from_millis(100),
        );

        let report = monitor.generate_performance_report();
        assert!(report.overall_score >= 0.0);
        assert!(!report.optimization_suggestions.is_empty());
    }

    #[test]
    fn test_resource_usage_defaults() {
        let usage = ResourceUsage::default();
        assert_eq!(usage.memory_usage_mb, 0);
        assert_eq!(usage.cpu_usage_percent, 0.0);
    }

    #[test]
    fn test_performance_thresholds() {
        let thresholds = PerformanceThresholds::default();
        assert_eq!(thresholds.max_verification_time_ms, 5000);
        assert_eq!(thresholds.max_memory_usage_mb, 512);
    }
}
