//! F₆: RossNet Scoring System
//!
//! Implements the RossNet scoring algorithm: μ_f≜σ(θ·sim+fit+aff)

use super::types::*;
use crate::{
    error::AispResult,
    pocket_architecture::ContentHash,
};
use std::collections::HashMap;
use std::time::Instant;

/// F₆: RossNet Scoring System
/// Implements: μ_f≜σ(θ·sim+fit+aff) with neural network activation
pub struct RossNetScorer {
    /// Weighted scoring parameters
    weights: RossNetWeights,
    /// Similarity calculation engine
    similarity_engine: SimilarityEngine,
    /// Fitness evaluation engine
    fitness_evaluator: FitnessEvaluator,
    /// Affinity tracking system
    affinity_tracker: AffinityTracker,
    /// Performance and accuracy statistics
    scoring_stats: ScoringStatistics,
}

/// Similarity calculation engine with multiple algorithms
pub struct SimilarityEngine {
    /// Vector-based similarity calculator
    vector_calculator: VectorSimilarityCalculator,
    /// Semantic similarity calculator
    semantic_calculator: SemanticSimilarityCalculator,
    /// Structural similarity calculator
    structural_calculator: StructuralSimilarityCalculator,
}

/// Fitness evaluation engine
pub struct FitnessEvaluator {
    /// Evaluation criteria weights
    criteria_weights: HashMap<String, f64>,
}

/// Affinity tracking system
pub struct AffinityTracker {
    /// Historical affinity scores
    affinity_history: HashMap<(ContentHash, ContentHash), Vec<f64>>,
    /// Temporal decay parameters
    decay_factors: HashMap<String, f64>,
}

/// Performance and accuracy statistics
#[derive(Debug, Clone, Default)]
pub struct ScoringStatistics {
    /// Total scoring operations performed
    pub total_scores_calculated: usize,
    /// Average scoring time in milliseconds
    pub average_scoring_time_ms: f64,
    /// Score distribution statistics
    pub score_distribution: HashMap<String, usize>, // ranges -> count
    /// Accuracy metrics when ground truth available
    pub accuracy_metrics: HashMap<String, f64>,
}

/// Vector-based similarity calculator
pub struct VectorSimilarityCalculator;

/// Semantic similarity calculator
pub struct SemanticSimilarityCalculator;

/// Structural similarity calculator
pub struct StructuralSimilarityCalculator;

impl RossNetScorer {
    /// Create new RossNet scorer with default configuration
    pub fn new() -> Self {
        Self {
            weights: RossNetWeights::default(),
            similarity_engine: SimilarityEngine::new(),
            fitness_evaluator: FitnessEvaluator::new(),
            affinity_tracker: AffinityTracker::new(),
            scoring_stats: ScoringStatistics::default(),
        }
    }

    /// Create scorer with custom weights
    pub fn with_weights(weights: RossNetWeights) -> Self {
        Self {
            weights,
            similarity_engine: SimilarityEngine::new(),
            fitness_evaluator: FitnessEvaluator::new(),
            affinity_tracker: AffinityTracker::new(),
            scoring_stats: ScoringStatistics::default(),
        }
    }

    /// Calculate RossNet score: μ_f≜σ(θ·sim+fit+aff)
    pub fn calculate_score(
        &mut self,
        content_a: &ContentHash,
        content_b: &ContentHash,
        context: &FitnessContext,
    ) -> AispResult<ScoringFeedback> {
        let start_time = Instant::now();

        // Calculate similarity component
        let similarity_score = self
            .similarity_engine
            .calculate_similarity(content_a, content_b)?;

        // Calculate fitness component
        let fitness_score = self
            .fitness_evaluator
            .evaluate_fitness(content_a, content_b, context)?;

        // Calculate affinity component
        let affinity_score = self.affinity_tracker.get_affinity(*content_a, *content_b);

        // Apply weighted combination: θ·sim+fit+aff
        let raw_score = self.weights.similarity_weight * similarity_score
            + self.weights.fitness_weight * fitness_score
            + self.weights.affinity_weight * affinity_score;

        // Apply diversity bonus and consistency penalty
        let diversity_bonus = self.calculate_diversity_bonus(content_a, content_b)?;
        let consistency_penalty = self.calculate_consistency_penalty(content_a, content_b)?;

        let adjusted_score = raw_score + self.weights.diversity_bonus * diversity_bonus
            - self.weights.consistency_penalty * consistency_penalty;

        // Apply sigmoid activation: σ(x) = 1/(1+e^(-x))
        let final_score = self.sigmoid_activation(adjusted_score);

        // Create detailed feedback
        let mut feedback_components = HashMap::new();
        feedback_components.insert("similarity".to_string(), similarity_score);
        feedback_components.insert("fitness".to_string(), fitness_score);
        feedback_components.insert("affinity".to_string(), affinity_score);
        feedback_components.insert("diversity_bonus".to_string(), diversity_bonus);
        feedback_components.insert("consistency_penalty".to_string(), consistency_penalty);
        feedback_components.insert("raw_score".to_string(), raw_score);

        let improvement_suggestions = self.generate_improvement_suggestions(&feedback_components);
        let confidence_level = self.calculate_confidence_level(&feedback_components);

        let feedback = ScoringFeedback {
            score: final_score,
            feedback_components,
            improvement_suggestions,
            confidence_level,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };

        // Update statistics
        let elapsed_ms = start_time.elapsed().as_millis() as f64;
        self.update_scoring_statistics(final_score, elapsed_ms);

        Ok(feedback)
    }

    /// Batch score multiple content pairs
    pub fn batch_score(
        &mut self,
        pairs: &[(ContentHash, ContentHash)],
        context: &FitnessContext,
    ) -> AispResult<Vec<ScoringFeedback>> {
        let mut results = Vec::with_capacity(pairs.len());

        for (content_a, content_b) in pairs {
            let feedback = self.calculate_score(content_a, content_b, context)?;
            results.push(feedback);
        }

        Ok(results)
    }

    /// Update scoring weights based on feedback
    pub fn update_weights(&mut self, weight_adjustments: &HashMap<String, f64>) {
        for (component, adjustment) in weight_adjustments {
            match component.as_str() {
                "similarity" => self.weights.similarity_weight = (*adjustment).clamp(0.0, 1.0),
                "fitness" => self.weights.fitness_weight = (*adjustment).clamp(0.0, 1.0),
                "affinity" => self.weights.affinity_weight = (*adjustment).clamp(0.0, 1.0),
                "diversity_bonus" => self.weights.diversity_bonus = (*adjustment).clamp(0.0, 1.0),
                "consistency_penalty" => {
                    self.weights.consistency_penalty = (*adjustment).clamp(0.0, 1.0)
                }
                _ => {} // Ignore unknown components
            }
        }

        // Normalize weights to ensure they sum to reasonable values
        self.normalize_weights();
    }

    /// Get current scoring statistics
    pub fn get_statistics(&self) -> &ScoringStatistics {
        &self.scoring_stats
    }

    /// Reset statistics
    pub fn reset_statistics(&mut self) {
        self.scoring_stats = ScoringStatistics::default();
    }

    /// Calculate diversity bonus
    fn calculate_diversity_bonus(
        &self,
        content_a: &ContentHash,
        content_b: &ContentHash,
    ) -> AispResult<f64> {
        // Simple diversity metric based on hash distance
        let hash_a = u64::from_le_bytes([
            content_a[0],
            content_a[1],
            content_a[2],
            content_a[3],
            content_a[4],
            content_a[5],
            content_a[6],
            content_a[7],
        ]);
        let hash_b = u64::from_le_bytes([
            content_b[0],
            content_b[1],
            content_b[2],
            content_b[3],
            content_b[4],
            content_b[5],
            content_b[6],
            content_b[7],
        ]);
        let xor_distance = hash_a ^ hash_b;
        let diversity = (xor_distance.count_ones() as f64) / 64.0; // Normalize to [0,1]
        Ok(diversity)
    }

    /// Calculate consistency penalty
    fn calculate_consistency_penalty(
        &self,
        content_a: &ContentHash,
        content_b: &ContentHash,
    ) -> AispResult<f64> {
        // Simple consistency check - low penalty for similar hashes
        let hash_a = u64::from_le_bytes([
            content_a[0],
            content_a[1],
            content_a[2],
            content_a[3],
            content_a[4],
            content_a[5],
            content_a[6],
            content_a[7],
        ]);
        let hash_b = u64::from_le_bytes([
            content_b[0],
            content_b[1],
            content_b[2],
            content_b[3],
            content_b[4],
            content_b[5],
            content_b[6],
            content_b[7],
        ]);
        let similarity = 1.0 - ((hash_a ^ hash_b).count_ones() as f64) / 64.0;
        let penalty = if similarity > 0.9 { 0.1 } else { 0.0 };
        Ok(penalty)
    }

    /// Apply sigmoid activation function
    fn sigmoid_activation(&self, x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }

    /// Generate improvement suggestions
    fn generate_improvement_suggestions(&self, components: &HashMap<String, f64>) -> Vec<String> {
        let mut suggestions = Vec::new();

        if let Some(&similarity) = components.get("similarity") {
            if similarity < 0.3 {
                suggestions.push("Consider improving content similarity".to_string());
            }
        }

        if let Some(&fitness) = components.get("fitness") {
            if fitness < 0.5 {
                suggestions.push("Fitness score could be improved".to_string());
            }
        }

        if let Some(&affinity) = components.get("affinity") {
            if affinity < 0.0 {
                suggestions
                    .push("Negative affinity detected - check interaction history".to_string());
            }
        }

        suggestions
    }

    /// Calculate confidence level
    fn calculate_confidence_level(&self, components: &HashMap<String, f64>) -> f64 {
        let variance = self.calculate_component_variance(components);
        
        (1.0 / (1.0 + variance)).clamp(0.1, 1.0)
    }

    /// Calculate variance in score components
    fn calculate_component_variance(&self, components: &HashMap<String, f64>) -> f64 {
        let values: Vec<f64> = components.values().copied().collect();
        if values.is_empty() {
            return 0.0;
        }

        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let variance =
            values.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / values.len() as f64;
        variance
    }

    /// Update scoring statistics
    fn update_scoring_statistics(&mut self, score: f64, elapsed_ms: f64) {
        self.scoring_stats.total_scores_calculated += 1;

        // Update average scoring time using exponential moving average
        // Ensure minimum measurable time to prevent zero division issues
        let actual_elapsed = elapsed_ms.max(0.001); // At least 1 microsecond
        let alpha = 0.1;
        self.scoring_stats.average_scoring_time_ms =
            alpha * actual_elapsed + (1.0 - alpha) * self.scoring_stats.average_scoring_time_ms;

        // Update score distribution
        let range = match score {
            s if s < 0.2 => "0.0-0.2",
            s if s < 0.4 => "0.2-0.4",
            s if s < 0.6 => "0.4-0.6",
            s if s < 0.8 => "0.6-0.8",
            _ => "0.8-1.0",
        };
        *self
            .scoring_stats
            .score_distribution
            .entry(range.to_string())
            .or_insert(0) += 1;
    }

    /// Normalize weights to ensure consistency
    fn normalize_weights(&mut self) {
        // Only normalize the main scoring weights, not bonus/penalty factors
        let total = self.weights.similarity_weight
            + self.weights.fitness_weight
            + self.weights.affinity_weight;

        if total > 0.0 {
            self.weights.similarity_weight /= total;
            self.weights.fitness_weight /= total;
            self.weights.affinity_weight /= total;
        }

        // Ensure diversity bonus and consistency penalty stay within reasonable bounds
        self.weights.diversity_bonus = self.weights.diversity_bonus.clamp(0.0, 1.0);
        self.weights.consistency_penalty = self.weights.consistency_penalty.clamp(0.0, 1.0);
    }
}

impl Default for SimilarityEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SimilarityEngine {
    /// Create new similarity engine
    pub fn new() -> Self {
        Self {
            vector_calculator: VectorSimilarityCalculator,
            semantic_calculator: SemanticSimilarityCalculator,
            structural_calculator: StructuralSimilarityCalculator,
        }
    }

    /// Calculate overall similarity score
    pub fn calculate_similarity(
        &self,
        content_a: &ContentHash,
        content_b: &ContentHash,
    ) -> AispResult<f64> {
        let vector_sim = self.vector_calculator.calculate(content_a, content_b)?;
        let semantic_sim = self.semantic_calculator.calculate(content_a, content_b)?;
        let structural_sim = self.structural_calculator.calculate(content_a, content_b)?;

        // Weighted combination of similarity measures
        let combined_similarity = 0.4 * vector_sim + 0.4 * semantic_sim + 0.2 * structural_sim;
        Ok(combined_similarity.clamp(0.0, 1.0))
    }
}

impl VectorSimilarityCalculator {
    /// Calculate vector-based similarity
    pub fn calculate(&self, content_a: &ContentHash, content_b: &ContentHash) -> AispResult<f64> {
        // Simple hash-based similarity
        let hash_a = u64::from_le_bytes([
            content_a[0],
            content_a[1],
            content_a[2],
            content_a[3],
            content_a[4],
            content_a[5],
            content_a[6],
            content_a[7],
        ]);
        let hash_b = u64::from_le_bytes([
            content_b[0],
            content_b[1],
            content_b[2],
            content_b[3],
            content_b[4],
            content_b[5],
            content_b[6],
            content_b[7],
        ]);
        let different_bits = (hash_a ^ hash_b).count_ones();
        let common_bits = 64 - different_bits;
        let similarity = common_bits as f64 / 64.0;
        Ok(similarity)
    }
}

impl SemanticSimilarityCalculator {
    /// Calculate semantic similarity
    pub fn calculate(&self, content_a: &ContentHash, content_b: &ContentHash) -> AispResult<f64> {
        // Placeholder semantic similarity - in real implementation would use embeddings
        let hash_a = u64::from_le_bytes([
            content_a[0],
            content_a[1],
            content_a[2],
            content_a[3],
            content_a[4],
            content_a[5],
            content_a[6],
            content_a[7],
        ]);
        let hash_b = u64::from_le_bytes([
            content_b[0],
            content_b[1],
            content_b[2],
            content_b[3],
            content_b[4],
            content_b[5],
            content_b[6],
            content_b[7],
        ]);
        let semantic_distance = ((hash_a.wrapping_sub(hash_b)) as f64).abs() / u64::MAX as f64;
        let similarity = 1.0 - semantic_distance;
        Ok(similarity.clamp(0.0, 1.0))
    }
}

impl StructuralSimilarityCalculator {
    /// Calculate structural similarity
    pub fn calculate(&self, content_a: &ContentHash, content_b: &ContentHash) -> AispResult<f64> {
        // Placeholder structural similarity
        let hash_a = u64::from_le_bytes([
            content_a[0],
            content_a[1],
            content_a[2],
            content_a[3],
            content_a[4],
            content_a[5],
            content_a[6],
            content_a[7],
        ]);
        let hash_b = u64::from_le_bytes([
            content_b[0],
            content_b[1],
            content_b[2],
            content_b[3],
            content_b[4],
            content_b[5],
            content_b[6],
            content_b[7],
        ]);
        let structural_score = if hash_a % 3 == hash_b % 3 { 0.8 } else { 0.2 };
        Ok(structural_score)
    }
}

impl Default for FitnessEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl FitnessEvaluator {
    /// Create new fitness evaluator
    pub fn new() -> Self {
        let mut criteria_weights = HashMap::new();
        criteria_weights.insert("performance".to_string(), 0.3);
        criteria_weights.insert("reliability".to_string(), 0.3);
        criteria_weights.insert("maintainability".to_string(), 0.2);
        criteria_weights.insert("scalability".to_string(), 0.2);

        Self { criteria_weights }
    }

    /// Evaluate fitness score
    pub fn evaluate_fitness(
        &self,
        _content_a: &ContentHash,
        _content_b: &ContentHash,
        context: &FitnessContext,
    ) -> AispResult<f64> {
        let mut weighted_score = 0.0;
        let mut total_weight = 0.0;

        for (criterion, &weight) in &self.criteria_weights {
            if let Some(&metric_value) = context.performance_metrics.get(criterion) {
                weighted_score += weight * metric_value;
                total_weight += weight;
            }
        }

        let fitness = if total_weight > 0.0 {
            weighted_score / total_weight
        } else {
            0.5 // Default neutral fitness
        };

        Ok(fitness.clamp(0.0, 1.0))
    }

    /// Update criterion weight
    pub fn update_criterion_weight(&mut self, criterion: String, weight: f64) {
        self.criteria_weights
            .insert(criterion, weight.clamp(0.0, 1.0));
    }
}

impl Default for AffinityTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl AffinityTracker {
    /// Create new affinity tracker
    pub fn new() -> Self {
        let mut decay_factors = HashMap::new();
        decay_factors.insert("temporal".to_string(), 0.95);
        decay_factors.insert("frequency".to_string(), 0.98);

        Self {
            affinity_history: HashMap::new(),
            decay_factors,
        }
    }

    /// Get current affinity score
    pub fn get_affinity(&self, content_a: ContentHash, content_b: ContentHash) -> f64 {
        let key = (content_a, content_b);
        if let Some(history) = self.affinity_history.get(&key) {
            if let Some(&latest) = history.last() {
                return latest;
            }
        }
        0.0 // Default neutral affinity
    }

    /// Update affinity based on interaction
    pub fn update_affinity(
        &mut self,
        content_a: ContentHash,
        content_b: ContentHash,
        new_affinity: f64,
    ) {
        let key = (content_a, content_b);
        let history = self.affinity_history.entry(key).or_default();

        // Apply temporal decay to existing values
        let temporal_decay = self.decay_factors.get("temporal").copied().unwrap_or(0.95);
        for affinity in history.iter_mut() {
            *affinity *= temporal_decay;
        }

        history.push(new_affinity);

        // Keep only recent history (limit memory usage)
        if history.len() > 100 {
            history.drain(0..50);
        }
    }

    /// Get affinity trend (increasing/decreasing)
    pub fn get_affinity_trend(&self, content_a: ContentHash, content_b: ContentHash) -> f64 {
        let key = (content_a, content_b);
        if let Some(history) = self.affinity_history.get(&key) {
            if history.len() >= 2 {
                let recent = history[history.len() - 1];
                let previous = history[history.len() - 2];
                return recent - previous;
            }
        }
        0.0
    }
}

impl Default for RossNetScorer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_context() -> FitnessContext {
        let mut metrics = HashMap::new();
        metrics.insert("performance".to_string(), 0.8);
        metrics.insert("reliability".to_string(), 0.9);
        metrics.insert("maintainability".to_string(), 0.7);

        FitnessContext {
            evaluation_criteria: vec!["performance".to_string(), "reliability".to_string()],
            performance_metrics: metrics,
            constraints: vec!["memory < 1GB".to_string()],
        }
    }

    #[test]
    fn test_rossnet_scorer_creation() {
        let scorer = RossNetScorer::new();
        assert_eq!(scorer.weights.similarity_weight, 0.4);
        assert_eq!(scorer.weights.fitness_weight, 0.3);
        assert_eq!(scorer.weights.affinity_weight, 0.3);
    }

    #[test]
    fn test_rossnet_score_calculation() {
        let mut scorer = RossNetScorer::new();
        let context = create_test_context();
        let mut content_a = [0u8; 32];
        let mut content_b = [0u8; 32];
        content_a[0..8].copy_from_slice(&12345u64.to_le_bytes());
        content_b[0..8].copy_from_slice(&67890u64.to_le_bytes());

        let result = scorer.calculate_score(&content_a, &content_b, &context);
        assert!(result.is_ok());

        let feedback = result.unwrap();
        assert!(feedback.score >= 0.0 && feedback.score <= 1.0);
        assert!(feedback.confidence_level > 0.0);
        assert!(!feedback.feedback_components.is_empty());
    }

    #[test]
    fn test_similarity_engine() {
        let engine = SimilarityEngine::new();
        let mut content_a = [0u8; 32];
        let mut content_b = [0u8; 32];
        content_a[0..8].copy_from_slice(&12345u64.to_le_bytes());
        content_b[0..8].copy_from_slice(&12345u64.to_le_bytes()); // Same content

        let similarity = engine.calculate_similarity(&content_a, &content_b).unwrap();
        assert!(similarity > 0.8); // Should be high for identical content
    }

    #[test]
    fn test_vector_similarity_calculator() {
        let calculator = VectorSimilarityCalculator;
        let mut content_a = [0u8; 32];
        let mut content_b = [0u8; 32];
        content_a[0..8].copy_from_slice(&0b1010101010101010u64.to_le_bytes());
        content_b[0..8].copy_from_slice(&0b1010101010101010u64.to_le_bytes());

        let similarity = calculator.calculate(&content_a, &content_b).unwrap();
        assert_eq!(similarity, 1.0); // Identical hashes should have similarity 1.0
    }

    #[test]
    fn test_fitness_evaluator() {
        let evaluator = FitnessEvaluator::new();
        let context = create_test_context();
        let mut content_a = [0u8; 32];
        let mut content_b = [0u8; 32];
        content_a[0..8].copy_from_slice(&123u64.to_le_bytes());
        content_b[0..8].copy_from_slice(&456u64.to_le_bytes());

        let fitness = evaluator
            .evaluate_fitness(&content_a, &content_b, &context)
            .unwrap();
        assert!((0.0..=1.0).contains(&fitness));
    }

    #[test]
    fn test_affinity_tracker() {
        let mut tracker = AffinityTracker::new();
        let mut content_a = [0u8; 32];
        let mut content_b = [0u8; 32];
        content_a[0..8].copy_from_slice(&111u64.to_le_bytes());
        content_b[0..8].copy_from_slice(&222u64.to_le_bytes());

        // Initial affinity should be 0
        let initial_affinity = tracker.get_affinity(content_a, content_b);
        assert_eq!(initial_affinity, 0.0);

        // Update affinity
        tracker.update_affinity(content_a, content_b, 0.75);
        let updated_affinity = tracker.get_affinity(content_a, content_b);
        assert_eq!(updated_affinity, 0.75);

        // Check trend calculation
        tracker.update_affinity(content_a, content_b, 0.85);
        let trend = tracker.get_affinity_trend(content_a, content_b);
        assert!(trend > 0.0); // Should be positive (increasing)
    }

    #[test]
    fn test_sigmoid_activation() {
        let scorer = RossNetScorer::new();

        // Test sigmoid properties
        assert_eq!(scorer.sigmoid_activation(0.0), 0.5);
        assert!(scorer.sigmoid_activation(-10.0) < 0.1);
        assert!(scorer.sigmoid_activation(10.0) > 0.9);
    }

    #[test]
    fn test_batch_scoring() {
        let mut scorer = RossNetScorer::new();
        let context = create_test_context();

        let mut content_pairs = Vec::new();
        for (a, b) in [(123u64, 456u64), (789u64, 101u64), (112u64, 131u64)] {
            let mut content_a = [0u8; 32];
            let mut content_b = [0u8; 32];
            content_a[0..8].copy_from_slice(&a.to_le_bytes());
            content_b[0..8].copy_from_slice(&b.to_le_bytes());
            content_pairs.push((content_a, content_b));
        }
        let pairs = content_pairs;

        let results = scorer.batch_score(&pairs, &context).unwrap();
        assert_eq!(results.len(), 3);

        for feedback in results {
            assert!(feedback.score >= 0.0 && feedback.score <= 1.0);
        }
    }

    #[test]
    fn test_weight_updates() {
        let mut scorer = RossNetScorer::new();
        let _original_similarity_weight = scorer.weights.similarity_weight;

        let mut adjustments = HashMap::new();
        adjustments.insert("similarity".to_string(), 0.6);
        adjustments.insert("fitness".to_string(), 0.3);
        adjustments.insert("affinity".to_string(), 0.1);

        scorer.update_weights(&adjustments);

        // Weights should be normalized
        let total = scorer.weights.similarity_weight
            + scorer.weights.fitness_weight
            + scorer.weights.affinity_weight;
        // Use a reasonable epsilon for floating point comparison (not machine epsilon)
        assert!((total - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_improvement_suggestions() {
        let scorer = RossNetScorer::new();

        let mut poor_components = HashMap::new();
        poor_components.insert("similarity".to_string(), 0.1);
        poor_components.insert("fitness".to_string(), 0.2);
        poor_components.insert("affinity".to_string(), -0.1);

        let suggestions = scorer.generate_improvement_suggestions(&poor_components);
        assert_eq!(suggestions.len(), 3); // Should have suggestions for all poor components
    }

    #[test]
    fn test_statistics_tracking() {
        let mut scorer = RossNetScorer::new();
        let context = create_test_context();
        let mut content_a = [0u8; 32];
        let mut content_b = [0u8; 32];
        content_a[0..8].copy_from_slice(&123u64.to_le_bytes());
        content_b[0..8].copy_from_slice(&456u64.to_le_bytes());

        // Initial stats
        assert_eq!(scorer.get_statistics().total_scores_calculated, 0);

        // Calculate a score
        let _ = scorer.calculate_score(&content_a, &content_b, &context);

        // Stats should be updated
        let stats = scorer.get_statistics();
        assert_eq!(stats.total_scores_calculated, 1);
        assert!(stats.average_scoring_time_ms > 0.0);
        assert!(!stats.score_distribution.is_empty());
    }

    #[test]
    fn test_diversity_bonus() {
        let scorer = RossNetScorer::new();

        // Very different hashes should have high diversity
        let mut content_a = [0u8; 32];
        let mut content_b = [0u8; 32];
        content_a[0..8].copy_from_slice(&0x0000000000000000u64.to_le_bytes());
        content_b[0..8].copy_from_slice(&0xFFFFFFFFFFFFFFFFu64.to_le_bytes());
        let diversity = scorer
            .calculate_diversity_bonus(&content_a, &content_b)
            .unwrap();
        assert!(diversity > 0.8);

        // Identical hashes should have low diversity
        let mut content_c = [0u8; 32];
        let mut content_d = [0u8; 32];
        content_c[0..8].copy_from_slice(&12345u64.to_le_bytes());
        content_d[0..8].copy_from_slice(&12345u64.to_le_bytes());
        let low_diversity = scorer
            .calculate_diversity_bonus(&content_c, &content_d)
            .unwrap();
        assert!(low_diversity < 0.2);
    }

    #[test]
    fn test_consistency_penalty() {
        let scorer = RossNetScorer::new();

        // Very similar hashes should have penalty
        let mut content_a = [0u8; 32];
        let mut content_b = [0u8; 32];
        content_a[0..8].copy_from_slice(&12345u64.to_le_bytes());
        content_b[0..8].copy_from_slice(&12345u64.to_le_bytes());
        let penalty = scorer
            .calculate_consistency_penalty(&content_a, &content_b)
            .unwrap();
        assert!(penalty > 0.0);

        // Very different hashes should have no penalty
        let mut content_c = [0u8; 32];
        let mut content_d = [0u8; 32];
        content_c[0..8].copy_from_slice(&0x0000000000000000u64.to_le_bytes());
        content_d[0..8].copy_from_slice(&0xFFFFFFFFFFFFFFFFu64.to_le_bytes());
        let no_penalty = scorer
            .calculate_consistency_penalty(&content_c, &content_d)
            .unwrap();
        assert_eq!(no_penalty, 0.0);
    }
}
