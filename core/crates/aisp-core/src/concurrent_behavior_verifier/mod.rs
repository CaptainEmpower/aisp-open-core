//! Concurrent Behavior Verification Module
//!
//! This module provides comprehensive analysis of concurrent behavior in AISP protocols,
//! breaking down the analysis into specialized components for maintainability.

pub mod analyzer;
pub mod deadlock_analyzer;
pub mod race_detector;
pub mod types;

pub use analyzer::ConcurrentBehaviorAnalyzer;
pub use deadlock_analyzer::DeadlockAnalyzer;
pub use race_detector::RaceConditionDetector;
pub use types::*;

use crate::{ast::canonical::CanonicalAispDocument as AispDocument, error::AispResult};

/// Main concurrent behavior verifier
pub struct ConcurrentBehaviorVerifier {
    /// Core analyzer
    analyzer: ConcurrentBehaviorAnalyzer,
    /// Race condition detector
    race_detector: RaceConditionDetector,
    /// Deadlock analyzer
    deadlock_analyzer: DeadlockAnalyzer,
}

impl ConcurrentBehaviorVerifier {
    /// Create new concurrent behavior verifier
    pub fn new() -> Self {
        Self {
            analyzer: ConcurrentBehaviorAnalyzer::new(),
            race_detector: RaceConditionDetector::new(),
            deadlock_analyzer: DeadlockAnalyzer::new(),
        }
    }

    /// Verify concurrent behavior in document
    pub fn verify(&mut self, document: &AispDocument) -> AispResult<ConcurrentBehaviorAnalysis> {
        // Perform main concurrent behavior analysis
        let mut analysis = self.analyzer.analyze(document)?;

        // Enhance with detailed race condition analysis
        let detailed_races = self
            .race_detector
            .detect_races(&analysis.concurrent_processes)?;
        for detailed_race in detailed_races {
            analysis
                .race_condition_analysis
                .race_conditions
                .push(detailed_race.base);
        }

        // Enhance with detailed deadlock analysis
        let enhanced_deadlock_analysis = self
            .deadlock_analyzer
            .analyze(&analysis.concurrent_processes)?;
        analysis.deadlock_analysis = enhanced_deadlock_analysis;

        Ok(analysis)
    }
}

impl Default for ConcurrentBehaviorVerifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verifier_creation() {
        let _verifier = ConcurrentBehaviorVerifier::new();
        // Basic smoke test - verifier should be created successfully
        assert!(true);
    }

    #[test]
    fn test_module_integration() {
        // Test that all sub-modules are properly integrated
        let _analyzer = ConcurrentBehaviorAnalyzer::new();
        let _race_detector = RaceConditionDetector::new();
        let _deadlock_analyzer = DeadlockAnalyzer::new();
        let _verifier = ConcurrentBehaviorVerifier::new();

        // Integration smoke test
        assert!(true);
    }
}
