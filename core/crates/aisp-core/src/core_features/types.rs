//! Core Features Types
//!
//! Type definitions and common structures for AISP core features.

use crate::{
    error::{AispError, AispResult},
    pocket_architecture::ContentHash,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// F₄: Four-State Binding System
/// Implements: Δ⊗λ∈{0,1,2,3} for API compatibility classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BindingState {
    /// ⊥: Crash - Logical contradiction, fatal error (0)
    Crash = 0,
    /// ∅: Null - Socket mismatch, connection fails (1)
    Null = 1,
    /// λ: Adapt - Type mismatch, adaptation possible (2)
    Adapt = 2,
    /// ⊤: Zero - Perfect compatibility, no adaptation needed (3)
    Zero = 3,
}

/// Type signature for binding analysis
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeSignature {
    pub base_type: String,
    pub parameters: Vec<String>,
    pub constraints: Vec<String>,
    pub semantic_hash: u64,
}

/// Contradiction pattern for logical consistency
#[derive(Debug, Clone)]
pub struct ContradictionPattern {
    pub pattern_name: String,
    pub antecedent: String,
    pub consequent: String,
    pub contradiction_proof: String,
}

/// Logical axiom for validation
#[derive(Debug, Clone)]
pub struct LogicalAxiom {
    pub axiom_name: String,
    pub statement: String,
    pub proof_certificate: String,
}

/// Socket interface definition
#[derive(Debug, Clone)]
pub struct SocketInterface {
    pub interface_id: String,
    pub required_methods: Vec<String>,
    pub provided_capabilities: Vec<String>,
    pub communication_protocol: String,
}

/// Compatibility level between components
#[derive(Debug, Clone, PartialEq)]
pub enum CompatibilityLevel {
    Perfect,      // Direct compatibility
    Adaptable,    // Requires adaptation
    Incompatible, // No compatibility possible
}

/// Adaptation strategy for type conversion
#[derive(Debug, Clone)]
pub struct AdaptationStrategy {
    pub strategy_id: String,
    pub source_type: String,
    pub target_type: String,
    pub adaptation_code: String,
    pub cost_estimate: f64,
    pub success_probability: f64,
}

/// RossNet scoring weights configuration
#[derive(Debug, Clone)]
pub struct RossNetWeights {
    pub similarity_weight: f64,
    pub fitness_weight: f64,
    pub affinity_weight: f64,
    pub diversity_bonus: f64,
    pub consistency_penalty: f64,
}

/// Fitness evaluation context
#[derive(Debug, Clone)]
pub struct FitnessContext {
    pub evaluation_criteria: Vec<String>,
    pub performance_metrics: HashMap<String, f64>,
    pub constraints: Vec<String>,
}

/// Scoring feedback for iterative improvement
#[derive(Debug, Clone)]
pub struct ScoringFeedback {
    pub score: f64,
    pub feedback_components: HashMap<String, f64>,
    pub improvement_suggestions: Vec<String>,
    pub confidence_level: f64,
    pub timestamp: u64,
}

/// Hebbian learning statistics
#[derive(Debug, Clone, Default)]
pub struct HebbianStatistics {
    pub successful_interactions: usize,
    pub failed_interactions: usize,
    pub total_updates: usize,
    pub average_affinity: f64,
    pub convergence_metrics: Vec<f64>,
}

/// Semantic vector for drift detection
#[derive(Debug, Clone)]
pub struct SemanticVector {
    pub dimensions: Vec<f64>,
    pub semantic_hash: u64,
    pub metadata: HashMap<String, String>,
}

/// Drift correction strategy
#[derive(Debug, Clone)]
pub struct DriftCorrectionStrategy {
    pub strategy_name: String,
    pub correction_vector: SemanticVector,
    pub application_threshold: f64,
    pub success_rate: f64,
}

/// Drift monitoring statistics
#[derive(Debug, Clone, Default)]
pub struct DriftMonitoringStats {
    pub drift_events_detected: usize,
    pub corrections_applied: usize,
    pub average_drift_magnitude: f64,
    pub monitoring_duration_ms: u64,
    pub stability_periods: Vec<u64>,
}

/// Optimization rule for recursive optimization
#[derive(Debug, Clone)]
pub struct OptimizationRule {
    pub rule_id: String,
    pub pattern: String,
    pub transformation: String,
    pub quality_improvement: f64,
    pub application_count: usize,
}

/// Optimization statistics tracking
#[derive(Debug, Clone, Default)]
pub struct OptimizationStatistics {
    pub total_optimizations: usize,
    pub successful_optimizations: usize,
    pub average_quality_improvement: f64,
    pub optimization_time_ms: u64,
    pub convergence_iterations: usize,
}

/// Optimized document result
#[derive(Debug, Clone)]
pub struct OptimizedDocument {
    pub content: String,
    pub quality_score: f64,
    pub optimization_steps: Vec<String>,
    pub metadata: HashMap<String, String>,
}

/// Bridge template for synthesis
#[derive(Debug, Clone)]
pub struct BridgeTemplate {
    pub template_id: String,
    pub source_pattern: String,
    pub target_pattern: String,
    pub transformation_logic: String,
}

/// Bridge synthesis strategy
#[derive(Debug, Clone)]
pub struct SynthesisStrategy {
    pub strategy_name: String,
    pub applicability_score: f64,
    pub transformation_complexity: f64,
}

/// Generated bridge result
#[derive(Debug, Clone)]
pub struct GeneratedBridge {
    pub bridge_code: String,
    pub confidence_score: f64,
    pub verification_status: BridgeVerificationStatus,
    pub performance_estimate: BridgePerformance,
}

/// Bridge verification status
#[derive(Debug, Clone, PartialEq)]
pub enum BridgeVerificationStatus {
    Verified,
    Partial,
    Failed,
    Untested,
}

/// Bridge performance metrics
#[derive(Debug, Clone)]
pub struct BridgePerformance {
    pub execution_time_estimate: f64,
    pub memory_usage_estimate: f64,
    pub success_probability: f64,
    pub error_handling_coverage: f64,
}

/// Synthesis statistics
#[derive(Debug, Clone, Default)]
pub struct SynthesisStatistics {
    pub bridges_generated: usize,
    pub bridges_verified: usize,
    pub average_synthesis_time: f64,
    pub success_rate: f64,
}

/// Drift detection result
#[derive(Debug, Clone)]
pub struct DriftDetection {
    pub drift_detected: bool,
    pub magnitude: f64,
    pub direction: SemanticVector,
    pub confidence: f64,
}

impl Default for RossNetWeights {
    fn default() -> Self {
        Self {
            similarity_weight: 0.4,
            fitness_weight: 0.3,
            affinity_weight: 0.3,
            diversity_bonus: 0.1,
            consistency_penalty: 0.2,
        }
    }
}

impl TypeSignature {
    /// Create new type signature
    pub fn new(base_type: String) -> Self {
        Self {
            base_type,
            parameters: Vec::new(),
            constraints: Vec::new(),
            semantic_hash: 0,
        }
    }

    /// Add parameter to type signature
    pub fn with_parameter(mut self, parameter: String) -> Self {
        self.parameters.push(parameter);
        self
    }

    /// Add constraint to type signature
    pub fn with_constraint(mut self, constraint: String) -> Self {
        self.constraints.push(constraint);
        self
    }

    /// Calculate semantic hash
    pub fn calculate_semantic_hash(&mut self) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        self.base_type.hash(&mut hasher);
        self.parameters.hash(&mut hasher);
        self.constraints.hash(&mut hasher);
        self.semantic_hash = hasher.finish();
    }
}

impl BindingState {
    /// Convert binding state to numeric value
    pub fn to_numeric(self) -> u8 {
        self as u8
    }

    /// Check if binding state allows execution
    pub fn allows_execution(self) -> bool {
        matches!(self, BindingState::Zero | BindingState::Adapt)
    }

    /// Check if binding state requires adaptation
    pub fn requires_adaptation(self) -> bool {
        self == BindingState::Adapt
    }

    /// Check if binding state is terminal (error state)
    pub fn is_terminal(self) -> bool {
        matches!(self, BindingState::Crash | BindingState::Null)
    }
}

impl SemanticVector {
    /// Create new semantic vector
    pub fn new(dimensions: Vec<f64>) -> Self {
        let mut vector = Self {
            dimensions,
            semantic_hash: 0,
            metadata: HashMap::new(),
        };
        vector.calculate_hash();
        vector
    }

    /// Calculate magnitude of vector
    pub fn magnitude(&self) -> f64 {
        self.dimensions.iter().map(|&x| x * x).sum::<f64>().sqrt()
    }

    /// Normalize vector to unit length
    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag > 0.0 {
            for dim in &mut self.dimensions {
                *dim /= mag;
            }
            self.calculate_hash();
        }
    }

    /// Calculate dot product with another vector
    pub fn dot_product(&self, other: &SemanticVector) -> AispResult<f64> {
        if self.dimensions.len() != other.dimensions.len() {
            return Err(AispError::validation_error(
                "Vector dimensions must match for dot product".to_string(),
            ));
        }

        Ok(self
            .dimensions
            .iter()
            .zip(other.dimensions.iter())
            .map(|(&a, &b)| a * b)
            .sum())
    }

    /// Calculate distance to another vector
    pub fn distance(&self, other: &SemanticVector) -> AispResult<f64> {
        if self.dimensions.len() != other.dimensions.len() {
            return Err(AispError::validation_error(
                "Vector dimensions must match for distance calculation".to_string(),
            ));
        }

        let squared_diff: f64 = self
            .dimensions
            .iter()
            .zip(other.dimensions.iter())
            .map(|(&a, &b)| (a - b).powi(2))
            .sum();

        Ok(squared_diff.sqrt())
    }

    fn calculate_hash(&mut self) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        for &dim in &self.dimensions {
            dim.to_bits().hash(&mut hasher);
        }
        self.semantic_hash = hasher.finish();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binding_state_properties() {
        assert_eq!(BindingState::Crash.to_numeric(), 0);
        assert_eq!(BindingState::Null.to_numeric(), 1);
        assert_eq!(BindingState::Adapt.to_numeric(), 2);
        assert_eq!(BindingState::Zero.to_numeric(), 3);

        assert!(!BindingState::Crash.allows_execution());
        assert!(!BindingState::Null.allows_execution());
        assert!(BindingState::Adapt.allows_execution());
        assert!(BindingState::Zero.allows_execution());

        assert!(!BindingState::Zero.requires_adaptation());
        assert!(BindingState::Adapt.requires_adaptation());

        assert!(BindingState::Crash.is_terminal());
        assert!(BindingState::Null.is_terminal());
        assert!(!BindingState::Zero.is_terminal());
        assert!(!BindingState::Adapt.is_terminal());
    }

    #[test]
    fn test_type_signature_creation() {
        let mut sig = TypeSignature::new("Function".to_string())
            .with_parameter("String".to_string())
            .with_parameter("Int".to_string())
            .with_constraint("Pure".to_string());

        sig.calculate_semantic_hash();
        assert_ne!(sig.semantic_hash, 0);
        assert_eq!(sig.base_type, "Function");
        assert_eq!(sig.parameters.len(), 2);
        assert_eq!(sig.constraints.len(), 1);
    }

    #[test]
    fn test_semantic_vector_operations() {
        let mut vec1 = SemanticVector::new(vec![3.0, 4.0, 0.0]);
        let vec2 = SemanticVector::new(vec![1.0, 0.0, 0.0]);

        assert_eq!(vec1.magnitude(), 5.0);

        vec1.normalize();
        assert!((vec1.magnitude() - 1.0).abs() < f64::EPSILON);

        let dot_result = vec1.dot_product(&vec2).unwrap();
        assert!((dot_result - 0.6).abs() < 0.1);

        let distance = vec1.distance(&vec2).unwrap();
        assert!(distance > 0.0);
    }

    #[test]
    fn test_rossnet_weights_default() {
        let weights = RossNetWeights::default();
        assert_eq!(weights.similarity_weight, 0.4);
        assert_eq!(weights.fitness_weight, 0.3);
        assert_eq!(weights.affinity_weight, 0.3);
        assert_eq!(weights.diversity_bonus, 0.1);
        assert_eq!(weights.consistency_penalty, 0.2);
    }

    #[test]
    fn test_vector_dimension_mismatch() {
        let vec1 = SemanticVector::new(vec![1.0, 2.0]);
        let vec2 = SemanticVector::new(vec![1.0, 2.0, 3.0]);

        assert!(vec1.dot_product(&vec2).is_err());
        assert!(vec1.distance(&vec2).is_err());
    }

    #[test]
    fn test_bridge_verification_status() {
        let status = BridgeVerificationStatus::Verified;
        assert_eq!(status, BridgeVerificationStatus::Verified);
        assert_ne!(status, BridgeVerificationStatus::Failed);
    }

    #[test]
    fn test_hebbian_statistics_default() {
        let stats = HebbianStatistics::default();
        assert_eq!(stats.successful_interactions, 0);
        assert_eq!(stats.failed_interactions, 0);
        assert_eq!(stats.total_updates, 0);
        assert_eq!(stats.average_affinity, 0.0);
        assert!(stats.convergence_metrics.is_empty());
    }

    #[test]
    fn test_optimization_statistics_default() {
        let stats = OptimizationStatistics::default();
        assert_eq!(stats.total_optimizations, 0);
        assert_eq!(stats.successful_optimizations, 0);
        assert_eq!(stats.average_quality_improvement, 0.0);
        assert_eq!(stats.optimization_time_ms, 0);
        assert_eq!(stats.convergence_iterations, 0);
    }

    #[test]
    fn test_drift_monitoring_stats_default() {
        let stats = DriftMonitoringStats::default();
        assert_eq!(stats.drift_events_detected, 0);
        assert_eq!(stats.corrections_applied, 0);
        assert_eq!(stats.average_drift_magnitude, 0.0);
        assert_eq!(stats.monitoring_duration_ms, 0);
        assert!(stats.stability_periods.is_empty());
    }
}
