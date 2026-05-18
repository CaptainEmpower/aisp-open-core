//! Resource Utilization Analysis Module
//!
//! This module provides comprehensive analysis of resource utilization in AISP protocols,
//! breaking down the analysis into specialized components for maintainability.

pub mod analyzer;
pub mod forecaster;
pub mod metrics;
pub mod optimizer;
pub mod types;

pub use analyzer::ResourceUtilizationAnalyzer;
pub use forecaster::ResourceForecaster;
pub use metrics::MetricsCollector;
pub use optimizer::ResourceOptimizer;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_integration() {
        // Test that all modules are properly integrated
        let _analyzer = ResourceUtilizationAnalyzer::new();
        let _metrics = MetricsCollector::new();
        let _optimizer = ResourceOptimizer::new();
        let _forecaster = ResourceForecaster::new();

        // Basic smoke test
        assert!(true);
    }
}
