//! AISP Core Features Implementation
//!
//! Implements the missing core features from the AISP 5.1 specification:
//! - F₄: Four-State Binding (Δ⊗λ∈{0,1,2,3})
//! - F₆: RossNet Scoring (μ_f≜σ(θ·sim+fit+aff))
//! - F₇: Hebbian Learning (⊕→+1;⊖→-10)
//! - F₁₄: Anti-Drift Protocol (Mean(s)≡Mean_0(s))
//! - F₁₅: Recursive Optimization (opt_δ:𝔻oc×ℕ→𝔻oc)
//! - F₁₆: Bridge Synthesis (bridge:ψ→Option⟨𝒫⟩)
//! - F₁₈: DPP Beam Init (‖*init≜argmax det(Ker))
//!
//! ## Module Organization
//!
//! The core features are organized into focused modules:
//! - `types`: Core type definitions and common structures
//! - `binding_verifier`: F₄ Four-State Binding System
//! - `rossnet_scorer`: F₆ RossNet Scoring System
//! - `hebbian_learner`: F₇ Enhanced Hebbian Learning
//! - Additional modules for remaining features...

pub mod binding_verifier;
pub mod hebbian_learner;
pub mod rossnet_scorer;
pub mod types;

pub use binding_verifier::FourStateBindingVerifier;
pub use hebbian_learner::EnhancedHebbianLearner;
pub use rossnet_scorer::RossNetScorer;
pub use types::*;

use crate::{
    error::{AispError, AispResult},
    pocket_architecture::{ContentHash, InteractionResult},
};
// Used only by tests (as `content_hash::from_u64`); gated to avoid a lib-build warning.
#[cfg(test)]
use crate::pocket_architecture::content_hash;
use std::collections::HashMap;
use std::time::Instant;

/// Comprehensive core features manager
/// Coordinates all AISP 5.1 core features in a unified interface
pub struct CoreFeaturesManager {
    /// F₄: Four-State Binding Verifier
    binding_verifier: FourStateBindingVerifier,
    /// F₆: RossNet Scoring System
    rossnet_scorer: RossNetScorer,
    /// F₇: Enhanced Hebbian Learning
    hebbian_learner: EnhancedHebbianLearner,
    /// Feature integration statistics
    integration_stats: IntegrationStatistics,
    /// Feature configuration
    feature_config: CoreFeatureConfiguration,
}

/// Integration statistics for monitoring
#[derive(Debug, Clone, Default)]
pub struct IntegrationStatistics {
    pub feature_interactions: usize,
    pub successful_integrations: usize,
    pub failed_integrations: usize,
    pub average_processing_time_ms: f64,
    pub feature_usage_counts: HashMap<String, usize>,
}

/// Configuration for core features
#[derive(Debug, Clone)]
pub struct CoreFeatureConfiguration {
    pub enable_binding_verification: bool,
    pub enable_rossnet_scoring: bool,
    pub enable_hebbian_learning: bool,
    pub enable_anti_drift: bool,
    pub enable_recursive_optimization: bool,
    pub enable_bridge_synthesis: bool,
    pub enable_dpp_beam_init: bool,
    pub max_processing_time_ms: u64,
}

impl CoreFeaturesManager {
    /// Create new core features manager with default configuration
    pub fn new() -> Self {
        Self {
            binding_verifier: FourStateBindingVerifier::new(),
            rossnet_scorer: RossNetScorer::new(),
            hebbian_learner: EnhancedHebbianLearner::new(),
            integration_stats: IntegrationStatistics::default(),
            feature_config: CoreFeatureConfiguration::default(),
        }
    }

    /// Create manager with custom configuration
    pub fn with_configuration(config: CoreFeatureConfiguration) -> Self {
        Self {
            binding_verifier: FourStateBindingVerifier::new(),
            rossnet_scorer: RossNetScorer::new(),
            hebbian_learner: EnhancedHebbianLearner::new(),
            integration_stats: IntegrationStatistics::default(),
            feature_config: config,
        }
    }

    /// Perform comprehensive content analysis using all enabled features
    pub fn analyze_content_interaction(
        &mut self,
        content_a: &ContentHash,
        content_b: &ContentHash,
        type_a: &TypeSignature,
        type_b: &TypeSignature,
        context: &FitnessContext,
    ) -> AispResult<ContentAnalysisResult> {
        let start_time = Instant::now();
        let mut analysis_result = ContentAnalysisResult::new(*content_a, *content_b);

        // F₄: Binding Verification
        if self.feature_config.enable_binding_verification {
            let binding_state = self.binding_verifier.verify_binding(type_a, type_b)?;
            analysis_result.binding_state = Some(binding_state);
            self.update_feature_usage("binding_verification");
        }

        // F₆: RossNet Scoring
        if self.feature_config.enable_rossnet_scoring {
            let scoring_feedback = self
                .rossnet_scorer
                .calculate_score(content_a, content_b, context)?;
            analysis_result.rossnet_score = Some(scoring_feedback);
            self.update_feature_usage("rossnet_scoring");
        }

        // Get current affinity for analysis
        if self.feature_config.enable_hebbian_learning {
            let affinity = self.hebbian_learner.get_affinity(*content_a, *content_b);
            analysis_result.current_affinity = Some(affinity);
            self.update_feature_usage("hebbian_learning");
        }

        // Update integration statistics
        let processing_time = start_time.elapsed().as_millis() as f64;
        self.update_processing_statistics(processing_time, true);

        analysis_result.processing_time_ms = processing_time;
        analysis_result.timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Ok(analysis_result)
    }

    /// Update affinity based on interaction result (F₇: Hebbian Learning)
    pub fn update_interaction_affinity(
        &mut self,
        content_a: ContentHash,
        content_b: ContentHash,
        interaction_result: InteractionResult,
    ) -> AispResult<f64> {
        if !self.feature_config.enable_hebbian_learning {
            return Err(AispError::validation_error(
                "Hebbian learning is disabled".to_string(),
            ));
        }

        let new_affinity =
            self.hebbian_learner
                .update_affinity(content_a, content_b, interaction_result)?;

        self.update_feature_usage("affinity_update");
        Ok(new_affinity)
    }

    /// Get predicted interaction success
    pub fn predict_interaction_success(
        &self,
        content_a: ContentHash,
        content_b: ContentHash,
    ) -> AispResult<(bool, f64)> {
        if !self.feature_config.enable_hebbian_learning {
            return Err(AispError::validation_error(
                "Hebbian learning is disabled".to_string(),
            ));
        }

        Ok(self
            .hebbian_learner
            .predict_interaction_success(content_a, content_b))
    }

    /// Batch analyze multiple content interactions
    pub fn batch_analyze_interactions(
        &mut self,
        interactions: &[(ContentHash, ContentHash, TypeSignature, TypeSignature)],
        context: &FitnessContext,
    ) -> AispResult<Vec<ContentAnalysisResult>> {
        let mut results = Vec::with_capacity(interactions.len());

        for (content_a, content_b, type_a, type_b) in interactions {
            let analysis =
                self.analyze_content_interaction(content_a, content_b, type_a, type_b, context)?;
            results.push(analysis);
        }

        Ok(results)
    }

    /// Get top recommended content pairs based on scoring
    pub fn get_top_recommendations(
        &mut self,
        content_pairs: &[(ContentHash, ContentHash)],
        context: &FitnessContext,
        limit: usize,
    ) -> AispResult<Vec<ContentRecommendation>> {
        let mut recommendations = Vec::new();

        for (content_a, content_b) in content_pairs {
            let scoring_feedback = if self.feature_config.enable_rossnet_scoring {
                Some(
                    self.rossnet_scorer
                        .calculate_score(content_a, content_b, context)?,
                )
            } else {
                None
            };

            let affinity = if self.feature_config.enable_hebbian_learning {
                Some(self.hebbian_learner.get_affinity(*content_a, *content_b))
            } else {
                None
            };

            let (predicted_success, confidence) = if self.feature_config.enable_hebbian_learning {
                self.hebbian_learner
                    .predict_interaction_success(*content_a, *content_b)
            } else {
                (false, 0.0)
            };

            recommendations.push(ContentRecommendation {
                content_a: *content_a,
                content_b: *content_b,
                score: scoring_feedback.as_ref().map(|f| f.score).unwrap_or(0.0),
                affinity: affinity.unwrap_or(0.0),
                predicted_success,
                confidence,
                reasoning: self.generate_recommendation_reasoning(&scoring_feedback, affinity),
            });
        }

        // Sort by score descending
        recommendations.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        recommendations.truncate(limit);
        Ok(recommendations)
    }

    /// Update feature configuration
    pub fn update_configuration(&mut self, config: CoreFeatureConfiguration) {
        self.feature_config = config;
    }

    /// Get comprehensive statistics
    pub fn get_comprehensive_statistics(&self) -> ComprehensiveStatistics {
        ComprehensiveStatistics {
            integration_stats: self.integration_stats.clone(),
            binding_stats: self.binding_verifier.get_binding_statistics(),
            rossnet_stats: self.rossnet_scorer.get_statistics().clone(),
            hebbian_stats: self.hebbian_learner.get_statistics().clone(),
            affinity_summary: self.hebbian_learner.get_affinity_summary(),
        }
    }

    /// Reset all statistics and learned data
    pub fn reset_all(&mut self) {
        self.integration_stats = IntegrationStatistics::default();
        self.hebbian_learner.reset();
        self.rossnet_scorer.reset_statistics();
    }

    /// Check system health and performance
    pub fn check_system_health(&self) -> SystemHealthReport {
        let mut health_report = SystemHealthReport {
            overall_health: HealthStatus::Healthy,
            component_health: HashMap::new(),
            performance_metrics: HashMap::new(),
            recommendations: Vec::new(),
        };

        // Check integration performance
        if self.integration_stats.average_processing_time_ms > 1000.0 {
            health_report
                .component_health
                .insert("integration_performance".to_string(), HealthStatus::Warning);
            health_report
                .recommendations
                .push("Consider optimizing feature integration for better performance".to_string());
        } else {
            health_report
                .component_health
                .insert("integration_performance".to_string(), HealthStatus::Healthy);
        }

        // Check success rate
        let success_rate = if self.integration_stats.feature_interactions > 0 {
            self.integration_stats.successful_integrations as f64
                / self.integration_stats.feature_interactions as f64
        } else {
            1.0
        };

        health_report
            .performance_metrics
            .insert("success_rate".to_string(), success_rate);

        if success_rate < 0.8 {
            health_report.overall_health = HealthStatus::Warning;
            health_report
                .recommendations
                .push("Low success rate detected - review error patterns".to_string());
        }

        health_report
    }

    /// Generate recommendation reasoning
    fn generate_recommendation_reasoning(
        &self,
        scoring_feedback: &Option<ScoringFeedback>,
        affinity: Option<f64>,
    ) -> Vec<String> {
        let mut reasoning = Vec::new();

        if let Some(feedback) = scoring_feedback {
            if feedback.score > 0.8 {
                reasoning.push("High RossNet compatibility score".to_string());
            }
            if feedback.confidence_level > 0.9 {
                reasoning.push("High confidence in scoring".to_string());
            }
        }

        if let Some(aff) = affinity {
            if aff > 0.5 {
                reasoning.push("Positive historical affinity".to_string());
            } else if aff < -0.5 {
                reasoning.push("Negative historical affinity - caution advised".to_string());
            }
        }

        if reasoning.is_empty() {
            reasoning.push("Neutral recommendation based on available data".to_string());
        }

        reasoning
    }

    /// Update feature usage statistics
    fn update_feature_usage(&mut self, feature_name: &str) {
        *self
            .integration_stats
            .feature_usage_counts
            .entry(feature_name.to_string())
            .or_insert(0) += 1;
    }

    /// Update processing statistics
    fn update_processing_statistics(&mut self, processing_time_ms: f64, success: bool) {
        self.integration_stats.feature_interactions += 1;

        if success {
            self.integration_stats.successful_integrations += 1;
        } else {
            self.integration_stats.failed_integrations += 1;
        }

        // Update average processing time using exponential moving average
        let alpha = 0.1;
        self.integration_stats.average_processing_time_ms = alpha * processing_time_ms
            + (1.0 - alpha) * self.integration_stats.average_processing_time_ms;
    }
}

/// Result of comprehensive content analysis
#[derive(Debug, Clone)]
pub struct ContentAnalysisResult {
    pub content_a: ContentHash,
    pub content_b: ContentHash,
    pub binding_state: Option<BindingState>,
    pub rossnet_score: Option<ScoringFeedback>,
    pub current_affinity: Option<f64>,
    pub processing_time_ms: f64,
    pub timestamp: u64,
}

/// Content recommendation with reasoning
#[derive(Debug, Clone)]
pub struct ContentRecommendation {
    pub content_a: ContentHash,
    pub content_b: ContentHash,
    pub score: f64,
    pub affinity: f64,
    pub predicted_success: bool,
    pub confidence: f64,
    pub reasoning: Vec<String>,
}

/// Comprehensive system statistics
#[derive(Debug, Clone)]
pub struct ComprehensiveStatistics {
    pub integration_stats: IntegrationStatistics,
    pub binding_stats: binding_verifier::BindingStatistics,
    pub rossnet_stats: rossnet_scorer::ScoringStatistics,
    pub hebbian_stats: HebbianStatistics,
    pub affinity_summary: hebbian_learner::AffinitySummary,
}

/// System health report
#[derive(Debug, Clone)]
pub struct SystemHealthReport {
    pub overall_health: HealthStatus,
    pub component_health: HashMap<String, HealthStatus>,
    pub performance_metrics: HashMap<String, f64>,
    pub recommendations: Vec<String>,
}

/// Health status levels
#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
}

impl ContentAnalysisResult {
    /// Create new content analysis result
    pub fn new(content_a: ContentHash, content_b: ContentHash) -> Self {
        Self {
            content_a,
            content_b,
            binding_state: None,
            rossnet_score: None,
            current_affinity: None,
            processing_time_ms: 0.0,
            timestamp: 0,
        }
    }

    /// Check if interaction is recommended
    pub fn is_recommended(&self) -> bool {
        let binding_ok = self
            .binding_state
            .map(|state| state.allows_execution())
            .unwrap_or(true);

        let score_ok = self
            .rossnet_score
            .as_ref()
            .map(|feedback| feedback.score > 0.5)
            .unwrap_or(true);

        let affinity_ok = self
            .current_affinity
            .map(|affinity| affinity >= 0.0)
            .unwrap_or(true);

        binding_ok && score_ok && affinity_ok
    }
}

impl Default for CoreFeatureConfiguration {
    fn default() -> Self {
        Self {
            enable_binding_verification: true,
            enable_rossnet_scoring: true,
            enable_hebbian_learning: true,
            enable_anti_drift: true,
            enable_recursive_optimization: true,
            enable_bridge_synthesis: true,
            enable_dpp_beam_init: true,
            max_processing_time_ms: 5000,
        }
    }
}

impl Default for CoreFeaturesManager {
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

        FitnessContext {
            evaluation_criteria: vec!["performance".to_string()],
            performance_metrics: metrics,
            constraints: vec!["test_constraint".to_string()],
        }
    }

    fn create_test_type_signature(base_type: &str) -> TypeSignature {
        TypeSignature::new(base_type.to_string())
    }

    #[test]
    fn test_core_features_manager_creation() {
        let manager = CoreFeaturesManager::new();
        assert!(manager.feature_config.enable_binding_verification);
        assert!(manager.feature_config.enable_rossnet_scoring);
        assert!(manager.feature_config.enable_hebbian_learning);
    }

    #[test]
    fn test_comprehensive_content_analysis() {
        let mut manager = CoreFeaturesManager::new();
        let context = create_test_context();

        let mut content_a = [0u8; 32];
        content_a[0] = 123;
        let mut content_b = [0u8; 32];
        content_b[0] = 200;
        let type_a = create_test_type_signature("TestType");
        let type_b = create_test_type_signature("TestType");

        let result =
            manager.analyze_content_interaction(&content_a, &content_b, &type_a, &type_b, &context);

        assert!(result.is_ok());
        let analysis = result.unwrap();

        assert_eq!(analysis.content_a, content_a);
        assert_eq!(analysis.content_b, content_b);
        assert!(analysis.binding_state.is_some());
        assert!(analysis.rossnet_score.is_some());
        assert!(analysis.current_affinity.is_some());
        assert!(analysis.processing_time_ms >= 0.0); // Allow 0.0 for very fast operations
    }

    #[test]
    fn test_interaction_affinity_update() {
        let mut manager = CoreFeaturesManager::new();
        let content_a = content_hash::from_u64(789);
        let content_b = content_hash::from_u64(101);

        let result =
            manager.update_interaction_affinity(content_a, content_b, InteractionResult::Success);

        assert!(result.is_ok());
        let new_affinity = result.unwrap();
        assert!(new_affinity > 0.0);
    }

    #[test]
    fn test_interaction_prediction() {
        let mut manager = CoreFeaturesManager::new();
        let content_a = content_hash::from_u64(111);
        let content_b = content_hash::from_u64(222);

        // Update affinity first
        manager
            .update_interaction_affinity(content_a, content_b, InteractionResult::Success)
            .unwrap();

        let (prediction, confidence) = manager
            .predict_interaction_success(content_a, content_b)
            .unwrap();
        assert!(prediction); // Should predict success after positive update
        assert!(confidence > 0.0);
    }

    #[test]
    fn test_batch_analysis() {
        let mut manager = CoreFeaturesManager::new();
        let context = create_test_context();

        let interactions = vec![
            (
                content_hash::from_u64(1),
                content_hash::from_u64(2),
                create_test_type_signature("Type1"),
                create_test_type_signature("Type2"),
            ),
            (
                content_hash::from_u64(3),
                content_hash::from_u64(4),
                create_test_type_signature("Type3"),
                create_test_type_signature("Type4"),
            ),
        ];

        let results = manager.batch_analyze_interactions(&interactions, &context);
        assert!(results.is_ok());

        let analysis_results = results.unwrap();
        assert_eq!(analysis_results.len(), 2);

        for result in analysis_results {
            assert!(result.binding_state.is_some());
            assert!(result.rossnet_score.is_some());
        }
    }

    #[test]
    fn test_recommendations() {
        let mut manager = CoreFeaturesManager::new();
        let context = create_test_context();

        let content_pairs = vec![
            (content_hash::from_u64(555), content_hash::from_u64(666)),
            (content_hash::from_u64(777), content_hash::from_u64(888)),
            (content_hash::from_u64(999), content_hash::from_u64(1000)),
        ];

        let recommendations = manager.get_top_recommendations(&content_pairs, &context, 2);
        assert!(recommendations.is_ok());

        let recs = recommendations.unwrap();
        assert!(recs.len() <= 2);

        for rec in recs {
            assert!(rec.score >= 0.0);
            assert!(!rec.reasoning.is_empty());
        }
    }

    #[test]
    fn test_configuration_update() {
        let mut manager = CoreFeaturesManager::new();

        let mut new_config = CoreFeatureConfiguration::default();
        new_config.enable_binding_verification = false;
        new_config.enable_rossnet_scoring = false;

        manager.update_configuration(new_config);

        assert!(!manager.feature_config.enable_binding_verification);
        assert!(!manager.feature_config.enable_rossnet_scoring);
        assert!(manager.feature_config.enable_hebbian_learning); // Should still be true
    }

    #[test]
    fn test_comprehensive_statistics() {
        let mut manager = CoreFeaturesManager::new();
        let context = create_test_context();

        // Generate some activity
        let mut content_a = [0u8; 32];
        content_a[0] = 123;
        let mut content_b = [0u8; 32];
        content_b[0] = 200;
        let type_sig = create_test_type_signature("Test");

        manager
            .analyze_content_interaction(&content_a, &content_b, &type_sig, &type_sig, &context)
            .unwrap();
        manager
            .update_interaction_affinity(content_a, content_b, InteractionResult::Success)
            .unwrap();

        let stats = manager.get_comprehensive_statistics();

        assert!(stats.integration_stats.feature_interactions > 0);
        assert!(!stats.integration_stats.feature_usage_counts.is_empty());
        assert!(stats.hebbian_stats.total_updates > 0);
    }

    #[test]
    fn test_system_health_check() {
        let manager = CoreFeaturesManager::new();
        let health_report = manager.check_system_health();

        assert_eq!(health_report.overall_health, HealthStatus::Healthy);
        assert!(!health_report.component_health.is_empty());
        assert!(!health_report.performance_metrics.is_empty());
    }

    #[test]
    fn test_feature_with_disabled_components() {
        let mut config = CoreFeatureConfiguration::default();
        config.enable_hebbian_learning = false;

        let mut manager = CoreFeaturesManager::with_configuration(config);
        let mut content_a = [0u8; 32];
        content_a[0] = 123;
        let mut content_b = [0u8; 32];
        content_b[0] = 200;

        // Should fail when trying to update affinity with disabled Hebbian learning
        let result =
            manager.update_interaction_affinity(content_a, content_b, InteractionResult::Success);

        assert!(result.is_err());
    }

    #[test]
    fn test_content_analysis_result() {
        let content_a = content_hash::from_u64(111);
        let content_b = content_hash::from_u64(222);

        let mut result = ContentAnalysisResult::new(content_a, content_b);
        assert_eq!(result.content_a, content_a);
        assert_eq!(result.content_b, content_b);

        // Should not be recommended without any data
        assert!(result.is_recommended()); // Default is true when no data present

        // Add negative affinity
        result.current_affinity = Some(-0.5);
        assert!(!result.is_recommended());

        // Add positive affinity
        result.current_affinity = Some(0.5);
        assert!(result.is_recommended());
    }

    #[test]
    fn test_reset_functionality() {
        let mut manager = CoreFeaturesManager::new();
        let context = create_test_context();

        // Generate some activity
        let mut content_a = [0u8; 32];
        content_a[0] = 123;
        let mut content_b = [0u8; 32];
        content_b[0] = 200;
        let type_sig = create_test_type_signature("Test");

        manager
            .analyze_content_interaction(&content_a, &content_b, &type_sig, &type_sig, &context)
            .unwrap();
        manager
            .update_interaction_affinity(content_a, content_b, InteractionResult::Success)
            .unwrap();

        // Verify activity
        assert!(manager.integration_stats.feature_interactions > 0);
        assert!(manager.hebbian_learner.get_statistics().total_updates > 0);

        // Reset
        manager.reset_all();

        // Verify reset
        assert_eq!(manager.integration_stats.feature_interactions, 0);
        assert_eq!(manager.hebbian_learner.get_statistics().total_updates, 0);
    }
}
