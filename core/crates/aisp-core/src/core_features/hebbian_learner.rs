//! F₇: Enhanced Hebbian Learning System
//!
//! Implements Hebbian learning with 10:1 penalty ratio: ⊕→+1;⊖→-10

use super::types::*;
use crate::{
    error::AispResult,
    pocket_architecture::{ContentHash, InteractionResult},
};
// Used only by tests (as `content_hash::from_u64`); gated to avoid a lib-build warning.
#[cfg(test)]
use crate::pocket_architecture::content_hash;
use std::collections::HashMap;

/// F₇: Enhanced Hebbian Learning System
/// Implements: ⊕→+1;⊖→-10 for adaptive affinity learning
pub struct EnhancedHebbianLearner {
    /// Success reward magnitude
    success_reward: f64,
    /// Failure penalty magnitude (negative)
    failure_penalty: f64,
    /// Learning rate for adaptation speed
    learning_rate: f64,
    /// Temporal decay factor
    decay_factor: f64,
    /// Affinity matrix for pocket pairs
    affinity_matrix: HashMap<(ContentHash, ContentHash), f64>,
    /// Learning performance statistics
    learning_stats: HebbianStatistics,
    /// Confidence tracking for predictions
    confidence_tracker: ConfidenceTracker,
}

/// Confidence tracker for prediction accuracy
pub struct ConfidenceTracker {
    /// Confidence scores for each pocket pair
    confidence_scores: HashMap<(ContentHash, ContentHash), f64>,
    /// Update frequency tracking
    update_frequencies: HashMap<(ContentHash, ContentHash), usize>,
    /// Prediction accuracy history
    accuracy_history: Vec<f64>,
}

impl EnhancedHebbianLearner {
    /// Create new Hebbian learner with 10:1 penalty ratio
    pub fn new() -> Self {
        Self {
            success_reward: 1.0,
            failure_penalty: -10.0,
            learning_rate: 0.1,
            decay_factor: 0.99,
            affinity_matrix: HashMap::new(),
            learning_stats: HebbianStatistics::default(),
            confidence_tracker: ConfidenceTracker::new(),
        }
    }

    /// Create learner with custom parameters
    pub fn with_parameters(
        success_reward: f64,
        failure_penalty: f64,
        learning_rate: f64,
        decay_factor: f64,
    ) -> Self {
        Self {
            success_reward: success_reward.max(0.0),
            failure_penalty: failure_penalty.min(0.0),
            learning_rate: learning_rate.clamp(0.0, 1.0),
            decay_factor: decay_factor.clamp(0.0, 1.0),
            affinity_matrix: HashMap::new(),
            learning_stats: HebbianStatistics::default(),
            confidence_tracker: ConfidenceTracker::new(),
        }
    }

    /// Update affinity with Hebbian rule: ⊕→+1;⊖→-10
    pub fn update_affinity(
        &mut self,
        pocket_a: ContentHash,
        pocket_b: ContentHash,
        interaction_result: InteractionResult,
    ) -> AispResult<f64> {
        let key = (pocket_a, pocket_b);
        let current_affinity = self.affinity_matrix.get(&key).copied().unwrap_or(0.0);

        // Apply Hebbian rule
        let delta = match interaction_result {
            InteractionResult::Success => {
                self.learning_stats.successful_interactions += 1;
                self.success_reward
            }
            InteractionResult::Failure => {
                self.learning_stats.failed_interactions += 1;
                self.failure_penalty
            }
        };

        // Update with learning rate and decay: A_new = decay * A_old + lr * delta
        let new_affinity = (current_affinity * self.decay_factor) + (self.learning_rate * delta);

        // Apply bounds to prevent overflow
        let bounded_affinity = new_affinity.clamp(-100.0, 100.0);

        self.affinity_matrix.insert(key, bounded_affinity);

        // Update confidence based on interaction result
        self.confidence_tracker
            .update_confidence(pocket_a, pocket_b, interaction_result);

        // Update statistics
        self.learning_stats.total_updates += 1;
        self.update_convergence_metrics();

        Ok(bounded_affinity)
    }

    /// Get current affinity between two pockets
    pub fn get_affinity(&self, pocket_a: ContentHash, pocket_b: ContentHash) -> f64 {
        self.affinity_matrix
            .get(&(pocket_a, pocket_b))
            .copied()
            .unwrap_or(0.0)
    }

    /// Predict interaction success based on affinity
    pub fn predict_interaction_success(
        &self,
        pocket_a: ContentHash,
        pocket_b: ContentHash,
    ) -> (bool, f64) {
        let affinity = self.get_affinity(pocket_a, pocket_b);
        let confidence = self.confidence_tracker.get_confidence(pocket_a, pocket_b);

        // Positive affinity suggests success
        let prediction = affinity > 0.0;

        (prediction, confidence)
    }

    /// Batch update multiple affinities
    pub fn batch_update(
        &mut self,
        interactions: &[(ContentHash, ContentHash, InteractionResult)],
    ) -> AispResult<Vec<f64>> {
        let mut new_affinities = Vec::with_capacity(interactions.len());

        for &(pocket_a, pocket_b, result) in interactions {
            let affinity = self.update_affinity(pocket_a, pocket_b, result)?;
            new_affinities.push(affinity);
        }

        Ok(new_affinities)
    }

    /// Get top affinity pairs (most positive relationships)
    pub fn get_top_affinities(&self, limit: usize) -> Vec<((ContentHash, ContentHash), f64)> {
        let mut affinity_pairs: Vec<_> = self
            .affinity_matrix
            .iter()
            .map(|(&pair, &affinity)| (pair, affinity))
            .collect();

        affinity_pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        affinity_pairs.truncate(limit);

        affinity_pairs
    }

    /// Get bottom affinity pairs (most negative relationships)
    pub fn get_bottom_affinities(&self, limit: usize) -> Vec<((ContentHash, ContentHash), f64)> {
        let mut affinity_pairs: Vec<_> = self
            .affinity_matrix
            .iter()
            .filter(|(_, &affinity)| affinity < 0.0) // Only negative affinities
            .map(|(&pair, &affinity)| (pair, affinity))
            .collect();

        affinity_pairs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        affinity_pairs.truncate(limit);

        affinity_pairs
    }

    /// Check if affinity has converged for a specific pair
    pub fn has_converged(&self, threshold: f64) -> bool {
        if self.learning_stats.convergence_metrics.len() < 10 {
            return false;
        }

        let recent_changes: Vec<f64> = self
            .learning_stats
            .convergence_metrics
            .windows(2)
            .map(|w| (w[1] - w[0]).abs())
            .collect();

        recent_changes.iter().all(|&change| change < threshold)
    }

    /// Reset learning state
    pub fn reset(&mut self) {
        self.affinity_matrix.clear();
        self.learning_stats = HebbianStatistics::default();
        self.confidence_tracker.reset();
    }

    /// Adjust learning parameters
    pub fn adjust_learning_rate(&mut self, new_rate: f64) {
        self.learning_rate = new_rate.clamp(0.0, 1.0);
    }

    /// Adjust decay factor
    pub fn adjust_decay_factor(&mut self, new_decay: f64) {
        self.decay_factor = new_decay.clamp(0.0, 1.0);
    }

    /// Get learning statistics
    pub fn get_statistics(&self) -> &HebbianStatistics {
        &self.learning_stats
    }

    /// Get affinity matrix summary
    pub fn get_affinity_summary(&self) -> AffinitySummary {
        let affinities: Vec<f64> = self.affinity_matrix.values().copied().collect();

        if affinities.is_empty() {
            return AffinitySummary::default();
        }

        let mean = affinities.iter().sum::<f64>() / affinities.len() as f64;
        let variance =
            affinities.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / affinities.len() as f64;
        let std_dev = variance.sqrt();

        let positive_relationships = affinities.iter().filter(|&&x| x > 0.0).count();
        let negative_relationships = affinities.iter().filter(|&&x| x < 0.0).count();

        let mut sorted_affinities = affinities;
        sorted_affinities.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        AffinitySummary {
            total_pairs: self.affinity_matrix.len(),
            mean_affinity: mean,
            std_deviation: std_dev,
            min_affinity: sorted_affinities.first().copied().unwrap_or(0.0),
            max_affinity: sorted_affinities.last().copied().unwrap_or(0.0),
            median_affinity: if sorted_affinities.len().is_multiple_of(2) {
                let mid = sorted_affinities.len() / 2;
                (sorted_affinities[mid - 1] + sorted_affinities[mid]) / 2.0
            } else {
                sorted_affinities[sorted_affinities.len() / 2]
            },
            positive_relationships,
            negative_relationships,
        }
    }

    /// Apply forgetting mechanism to old affinities
    pub fn apply_forgetting(&mut self, forgetting_rate: f64) {
        let forgetting_factor = 1.0 - forgetting_rate.clamp(0.0, 1.0);

        for affinity in self.affinity_matrix.values_mut() {
            *affinity *= forgetting_factor;
        }
    }

    /// Update convergence metrics
    fn update_convergence_metrics(&mut self) {
        let average_affinity = if self.affinity_matrix.is_empty() {
            0.0
        } else {
            self.affinity_matrix.values().sum::<f64>() / self.affinity_matrix.len() as f64
        };

        self.learning_stats.average_affinity = average_affinity;
        self.learning_stats
            .convergence_metrics
            .push(average_affinity);

        // Keep only recent metrics (limit memory usage)
        if self.learning_stats.convergence_metrics.len() > 1000 {
            self.learning_stats.convergence_metrics.drain(0..500);
        }
    }
}

/// Summary statistics for affinity matrix
#[derive(Debug, Clone, Default)]
pub struct AffinitySummary {
    pub total_pairs: usize,
    pub mean_affinity: f64,
    pub std_deviation: f64,
    pub min_affinity: f64,
    pub max_affinity: f64,
    pub median_affinity: f64,
    pub positive_relationships: usize,
    pub negative_relationships: usize,
}

impl ConfidenceTracker {
    /// Create new confidence tracker
    pub fn new() -> Self {
        Self {
            confidence_scores: HashMap::new(),
            update_frequencies: HashMap::new(),
            accuracy_history: Vec::new(),
        }
    }

    /// Update confidence based on interaction result
    pub fn update_confidence(
        &mut self,
        pocket_a: ContentHash,
        pocket_b: ContentHash,
        result: InteractionResult,
    ) {
        let key = (pocket_a, pocket_b);

        // Increment frequency counter
        *self.update_frequencies.entry(key).or_insert(0) += 1;

        // Calculate confidence based on frequency and consistency
        let frequency = self.update_frequencies[&key] as f64;
        let _frequency_confidence = (frequency / (frequency + 5.0)).min(1.0);

        // Update running confidence score
        let current_confidence = self.confidence_scores.get(&key).copied().unwrap_or(0.5);
        let result_confidence = match result {
            InteractionResult::Success => 0.8,
            InteractionResult::Failure => 0.6,
        };

        // Exponential moving average
        let alpha = 0.2;
        let new_confidence = alpha * result_confidence + (1.0 - alpha) * current_confidence;
        // Use additive frequency bonus instead of multiplicative to prevent over-dampening
        let frequency_bonus = (frequency / 10.0).min(0.3);
        let final_confidence = (new_confidence + frequency_bonus).clamp(0.1, 1.0);

        self.confidence_scores.insert(key, final_confidence);
    }

    /// Get confidence score for a pocket pair
    pub fn get_confidence(&self, pocket_a: ContentHash, pocket_b: ContentHash) -> f64 {
        let key = (pocket_a, pocket_b);
        self.confidence_scores.get(&key).copied().unwrap_or(0.1)
    }

    /// Update accuracy history
    pub fn update_accuracy(&mut self, accuracy: f64) {
        self.accuracy_history.push(accuracy.clamp(0.0, 1.0));

        // Keep only recent history
        if self.accuracy_history.len() > 100 {
            self.accuracy_history.drain(0..50);
        }
    }

    /// Get average accuracy
    pub fn get_average_accuracy(&self) -> f64 {
        if self.accuracy_history.is_empty() {
            0.5
        } else {
            self.accuracy_history.iter().sum::<f64>() / self.accuracy_history.len() as f64
        }
    }

    /// Reset confidence tracker
    pub fn reset(&mut self) {
        self.confidence_scores.clear();
        self.update_frequencies.clear();
        self.accuracy_history.clear();
    }
}

impl Default for EnhancedHebbianLearner {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ConfidenceTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hebbian_learner_creation() {
        let learner = EnhancedHebbianLearner::new();
        assert_eq!(learner.success_reward, 1.0);
        assert_eq!(learner.failure_penalty, -10.0);
        assert_eq!(learner.learning_rate, 0.1);
        assert_eq!(learner.decay_factor, 0.99);
    }

    #[test]
    fn test_hebbian_learner_with_custom_parameters() {
        let learner = EnhancedHebbianLearner::with_parameters(2.0, -5.0, 0.2, 0.95);
        assert_eq!(learner.success_reward, 2.0);
        assert_eq!(learner.failure_penalty, -5.0);
        assert_eq!(learner.learning_rate, 0.2);
        assert_eq!(learner.decay_factor, 0.95);
    }

    #[test]
    fn test_affinity_update_success() {
        let mut learner = EnhancedHebbianLearner::new();
        let pocket_a = content_hash::from_u64(123);
        let pocket_b = content_hash::from_u64(456);

        // Initial affinity should be 0
        assert_eq!(learner.get_affinity(pocket_a, pocket_b), 0.0);

        // Update with successful interaction
        let result = learner.update_affinity(pocket_a, pocket_b, InteractionResult::Success);
        assert!(result.is_ok());

        let new_affinity = result.unwrap();
        assert!(new_affinity > 0.0);
        assert_eq!(learner.get_affinity(pocket_a, pocket_b), new_affinity);

        // Check statistics
        assert_eq!(learner.learning_stats.successful_interactions, 1);
        assert_eq!(learner.learning_stats.total_updates, 1);
    }

    #[test]
    fn test_affinity_update_failure() {
        let mut learner = EnhancedHebbianLearner::new();
        let pocket_a = content_hash::from_u64(123);
        let pocket_b = content_hash::from_u64(456);

        // Update with failed interaction
        let result = learner.update_affinity(pocket_a, pocket_b, InteractionResult::Failure);
        assert!(result.is_ok());

        let new_affinity = result.unwrap();
        assert!(new_affinity < 0.0);
        assert_eq!(learner.get_affinity(pocket_a, pocket_b), new_affinity);

        // Check statistics
        assert_eq!(learner.learning_stats.failed_interactions, 1);
        assert_eq!(learner.learning_stats.total_updates, 1);
    }

    #[test]
    fn test_10_to_1_penalty_ratio() {
        let mut learner = EnhancedHebbianLearner::new();
        let pocket_a = content_hash::from_u64(111);
        let pocket_b = content_hash::from_u64(222);

        // One success
        let success_affinity = learner
            .update_affinity(pocket_a, pocket_b, InteractionResult::Success)
            .unwrap();

        let pocket_c = content_hash::from_u64(333);
        let pocket_d = content_hash::from_u64(444);

        // One failure
        let failure_affinity = learner
            .update_affinity(pocket_c, pocket_d, InteractionResult::Failure)
            .unwrap();

        // Failure penalty should be 10x larger in magnitude
        assert!((failure_affinity.abs() / success_affinity.abs() - 10.0).abs() < 1.0);
    }

    #[test]
    fn test_prediction() {
        let mut learner = EnhancedHebbianLearner::new();
        let pocket_a = content_hash::from_u64(555);
        let pocket_b = content_hash::from_u64(666);

        // Initial prediction should be neutral
        let (prediction, confidence) = learner.predict_interaction_success(pocket_a, pocket_b);
        assert!(!prediction); // 0.0 affinity means false prediction
        assert!(confidence > 0.0);

        // After positive interactions, should predict success
        learner
            .update_affinity(pocket_a, pocket_b, InteractionResult::Success)
            .unwrap();
        learner
            .update_affinity(pocket_a, pocket_b, InteractionResult::Success)
            .unwrap();

        let (positive_prediction, higher_confidence) =
            learner.predict_interaction_success(pocket_a, pocket_b);
        assert!(positive_prediction);
        assert!(higher_confidence >= confidence);
    }

    #[test]
    fn test_batch_update() {
        let mut learner = EnhancedHebbianLearner::new();

        let interactions = vec![
            (
                content_hash::from_u64(1),
                content_hash::from_u64(2),
                InteractionResult::Success,
            ),
            (
                content_hash::from_u64(3),
                content_hash::from_u64(4),
                InteractionResult::Failure,
            ),
            (
                content_hash::from_u64(5),
                content_hash::from_u64(6),
                InteractionResult::Success,
            ),
        ];

        let affinities = learner.batch_update(&interactions).unwrap();
        assert_eq!(affinities.len(), 3);
        assert!(affinities[0] > 0.0); // Success
        assert!(affinities[1] < 0.0); // Failure
        assert!(affinities[2] > 0.0); // Success

        assert_eq!(learner.learning_stats.total_updates, 3);
    }

    #[test]
    fn test_top_and_bottom_affinities() {
        let mut learner = EnhancedHebbianLearner::new();

        // Create some relationships
        learner
            .update_affinity(
                content_hash::from_u64(1),
                content_hash::from_u64(2),
                InteractionResult::Success,
            )
            .unwrap();
        learner
            .update_affinity(
                content_hash::from_u64(1),
                content_hash::from_u64(2),
                InteractionResult::Success,
            )
            .unwrap();

        learner
            .update_affinity(
                content_hash::from_u64(3),
                content_hash::from_u64(4),
                InteractionResult::Failure,
            )
            .unwrap();
        learner
            .update_affinity(
                content_hash::from_u64(3),
                content_hash::from_u64(4),
                InteractionResult::Failure,
            )
            .unwrap();

        learner
            .update_affinity(
                content_hash::from_u64(5),
                content_hash::from_u64(6),
                InteractionResult::Success,
            )
            .unwrap();

        let top_affinities = learner.get_top_affinities(2);
        let bottom_affinities = learner.get_bottom_affinities(2);

        assert_eq!(top_affinities.len(), 2);
        assert_eq!(bottom_affinities.len(), 1); // Only one negative relationship

        // Top should have positive affinities
        assert!(top_affinities[0].1 > 0.0);
        assert!(top_affinities[1].1 > 0.0);

        // Bottom should have negative affinities
        assert!(bottom_affinities[0].1 < 0.0);
    }

    #[test]
    fn test_convergence_detection() {
        let mut learner = EnhancedHebbianLearner::new();
        let pocket_a = content_hash::from_u64(777);
        let pocket_b = content_hash::from_u64(888);

        // Should not converge initially
        assert!(!learner.has_converged(0.01));

        // Add consistent updates
        for _ in 0..20 {
            learner
                .update_affinity(pocket_a, pocket_b, InteractionResult::Success)
                .unwrap();
        }

        // Should eventually converge
        assert!(learner.has_converged(0.1));
    }

    #[test]
    fn test_parameter_adjustment() {
        let mut learner = EnhancedHebbianLearner::new();

        learner.adjust_learning_rate(0.5);
        assert_eq!(learner.learning_rate, 0.5);

        learner.adjust_decay_factor(0.8);
        assert_eq!(learner.decay_factor, 0.8);

        // Test bounds
        learner.adjust_learning_rate(2.0); // Should cap at 1.0
        assert_eq!(learner.learning_rate, 1.0);

        learner.adjust_learning_rate(-0.1); // Should floor at 0.0
        assert_eq!(learner.learning_rate, 0.0);
    }

    #[test]
    fn test_affinity_bounds() {
        let mut learner = EnhancedHebbianLearner::new();
        let pocket_a = content_hash::from_u64(999);
        let pocket_b = content_hash::from_u64(1000);

        // Many failures should not go below -100
        for _ in 0..50 {
            learner
                .update_affinity(pocket_a, pocket_b, InteractionResult::Failure)
                .unwrap();
        }

        let final_affinity = learner.get_affinity(pocket_a, pocket_b);
        assert!(final_affinity >= -100.0);

        // Reset and test upper bound
        learner.reset();

        // Many successes should not go above 100
        for _ in 0..50 {
            learner
                .update_affinity(pocket_a, pocket_b, InteractionResult::Success)
                .unwrap();
        }

        let final_affinity = learner.get_affinity(pocket_a, pocket_b);
        assert!(final_affinity <= 100.0);
    }

    #[test]
    fn test_confidence_tracker() {
        let mut tracker = ConfidenceTracker::new();
        let pocket_a = content_hash::from_u64(111);
        let pocket_b = content_hash::from_u64(222);

        // Initial confidence should be low
        let initial_confidence = tracker.get_confidence(pocket_a, pocket_b);
        assert_eq!(initial_confidence, 0.1);

        // Update with interaction results
        tracker.update_confidence(pocket_a, pocket_b, InteractionResult::Success);
        tracker.update_confidence(pocket_a, pocket_b, InteractionResult::Success);
        tracker.update_confidence(pocket_a, pocket_b, InteractionResult::Success);

        let updated_confidence = tracker.get_confidence(pocket_a, pocket_b);
        assert!(updated_confidence > initial_confidence);
    }

    #[test]
    fn test_affinity_summary() {
        let mut learner = EnhancedHebbianLearner::new();

        // Add various relationships
        learner
            .update_affinity(
                content_hash::from_u64(1),
                content_hash::from_u64(2),
                InteractionResult::Success,
            )
            .unwrap();
        learner
            .update_affinity(
                content_hash::from_u64(3),
                content_hash::from_u64(4),
                InteractionResult::Failure,
            )
            .unwrap();
        learner
            .update_affinity(
                content_hash::from_u64(5),
                content_hash::from_u64(6),
                InteractionResult::Success,
            )
            .unwrap();
        learner
            .update_affinity(
                content_hash::from_u64(7),
                content_hash::from_u64(8),
                InteractionResult::Failure,
            )
            .unwrap();

        let summary = learner.get_affinity_summary();

        assert_eq!(summary.total_pairs, 4);
        assert_eq!(summary.positive_relationships, 2);
        assert_eq!(summary.negative_relationships, 2);
        assert!(summary.min_affinity < 0.0);
        assert!(summary.max_affinity > 0.0);
    }

    #[test]
    fn test_forgetting_mechanism() {
        let mut learner = EnhancedHebbianLearner::new();
        let pocket_a = content_hash::from_u64(123);
        let pocket_b = content_hash::from_u64(456);

        // Build up some affinity
        learner
            .update_affinity(pocket_a, pocket_b, InteractionResult::Success)
            .unwrap();
        let initial_affinity = learner.get_affinity(pocket_a, pocket_b);

        // Apply forgetting
        learner.apply_forgetting(0.1); // 10% forgetting rate
        let forgotten_affinity = learner.get_affinity(pocket_a, pocket_b);

        // Affinity should decrease
        assert!(forgotten_affinity < initial_affinity);
        assert!(forgotten_affinity > 0.0); // Should still be positive
    }

    #[test]
    fn test_reset() {
        let mut learner = EnhancedHebbianLearner::new();
        let pocket_a = content_hash::from_u64(789);
        let pocket_b = content_hash::from_u64(101);

        // Add some data
        learner
            .update_affinity(pocket_a, pocket_b, InteractionResult::Success)
            .unwrap();
        assert!(learner.get_affinity(pocket_a, pocket_b) > 0.0);
        assert!(learner.learning_stats.total_updates > 0);

        // Reset
        learner.reset();

        // Everything should be cleared
        assert_eq!(learner.get_affinity(pocket_a, pocket_b), 0.0);
        assert_eq!(learner.learning_stats.total_updates, 0);
        assert_eq!(learner.affinity_matrix.len(), 0);
    }
}
