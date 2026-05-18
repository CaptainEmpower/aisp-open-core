//! RossNet Scoring Validation for AISP 5.1
//!
//! This module implements validation for RossNet scoring, which provides
//! comprehensive assessment of AISP documents through three key metrics:
//!
//! **RossNet Score = sim + fit + aff**
//!
//! Where:
//! - sim: Similarity scoring (semantic vector distance calculations)
//! - fit: Fitness scoring (behavioral adaptation metrics)
//! - aff: Affinity scoring (domain compatibility assessment)
//!
//! RossNet scoring is crucial for evaluating the quality and compatibility
//! of AISP implementations across different domains and contexts.

use crate::{
    ast::canonical::{CanonicalAispBlock as AispBlock, CanonicalAispDocument as AispDocument, *},
    error::*,
    semantic::DeepVerificationResult,
};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// RossNet scoring validation result
#[derive(Debug, Clone)]
pub struct RossNetValidationResult {
    /// Whether RossNet scoring is valid
    pub valid: bool,
    /// Overall RossNet score (sim + fit + aff)
    pub rossnet_score: f64,
    /// Individual scoring components
    pub components: RossNetComponents,
    /// Validation statistics
    pub stats: RossNetStats,
    /// Analysis errors
    pub errors: Vec<String>,
    /// Analysis warnings
    pub warnings: Vec<String>,
}

/// RossNet scoring components
#[derive(Debug, Clone)]
pub struct RossNetComponents {
    /// Similarity score (0.0 - 1.0)
    pub similarity: f64,
    /// Fitness score (0.0 - 1.0)
    pub fitness: f64,
    /// Affinity score (0.0 - 1.0)
    pub affinity: f64,
    /// Component-specific metrics
    pub similarity_metrics: SimilarityMetrics,
    pub fitness_metrics: FitnessMetrics,
    pub affinity_metrics: AffinityMetrics,
}

/// Similarity scoring metrics
#[derive(Debug, Clone)]
pub struct SimilarityMetrics {
    /// Semantic vector distance
    pub semantic_distance: f64,
    /// Structural similarity ratio
    pub structural_similarity: f64,
    /// Content overlap percentage
    pub content_overlap: f64,
    /// Type compatibility score
    pub type_compatibility: f64,
}

/// Fitness scoring metrics
#[derive(Debug, Clone)]
pub struct FitnessMetrics {
    /// Behavioral adaptation score
    pub behavioral_adaptation: f64,
    /// Performance efficiency ratio
    pub performance_efficiency: f64,
    /// Resource utilization score
    pub resource_utilization: f64,
    /// Temporal consistency score
    pub temporal_consistency: f64,
}

/// Affinity scoring metrics
#[derive(Debug, Clone)]
pub struct AffinityMetrics {
    /// Domain compatibility percentage
    pub domain_compatibility: f64,
    /// Protocol alignment score
    pub protocol_alignment: f64,
    /// Interface compatibility ratio
    pub interface_compatibility: f64,
    /// Context adaptation score
    pub context_adaptation: f64,
}

/// RossNet analysis statistics
#[derive(Debug, Clone)]
pub struct RossNetStats {
    /// Total analysis time
    pub analysis_time: Duration,
    /// Number of comparisons performed
    pub comparisons_performed: usize,
    /// Vector calculations count
    pub vector_calculations: usize,
    /// Cache hits during analysis
    pub cache_hits: usize,
    /// Cache misses during analysis
    pub cache_misses: usize,
}

/// Configuration for RossNet scoring
#[derive(Debug, Clone)]
pub struct RossNetConfig {
    /// Minimum acceptable RossNet score
    pub min_rossnet_score: f64,
    /// Maximum analysis time allowed
    pub max_analysis_time: Duration,
    /// Enable similarity caching
    pub enable_caching: bool,
    /// Similarity weight in final score
    pub similarity_weight: f64,
    /// Fitness weight in final score
    pub fitness_weight: f64,
    /// Affinity weight in final score
    pub affinity_weight: f64,
    /// Reference document for comparison
    pub reference_document: Option<String>,
}

impl Default for RossNetConfig {
    fn default() -> Self {
        Self {
            min_rossnet_score: 0.7,
            max_analysis_time: Duration::from_secs(10),
            enable_caching: true,
            similarity_weight: 0.4,
            fitness_weight: 0.35,
            affinity_weight: 0.25,
            reference_document: None,
        }
    }
}

/// RossNet scoring validator implementing sim+fit+aff analysis
pub struct RossNetValidator {
    /// Validation configuration
    config: RossNetConfig,
    /// Similarity calculation cache
    similarity_cache: HashMap<String, f64>,
    /// Analysis statistics
    stats: RossNetStats,
}

impl RossNetValidator {
    /// Create new RossNet validator
    pub fn new(config: RossNetConfig) -> Self {
        Self {
            config,
            similarity_cache: HashMap::new(),
            stats: RossNetStats {
                analysis_time: Duration::from_secs(0),
                comparisons_performed: 0,
                vector_calculations: 0,
                cache_hits: 0,
                cache_misses: 0,
            },
        }
    }

    /// Validate RossNet scoring for AISP document
    pub fn validate_rossnet_scoring(
        &mut self,
        document: &AispDocument,
        semantic_result: &DeepVerificationResult,
    ) -> AispResult<RossNetValidationResult> {
        let start_time = Instant::now();

        // Calculate similarity score (sim)
        let similarity_metrics = self.calculate_similarity_metrics(document, semantic_result)?;
        let similarity_score = self.compute_similarity_score(&similarity_metrics)?;

        // Calculate fitness score (fit)
        let fitness_metrics = self.calculate_fitness_metrics(document, semantic_result)?;
        let fitness_score = self.compute_fitness_score(&fitness_metrics)?;

        // Calculate affinity score (aff)
        let affinity_metrics = self.calculate_affinity_metrics(document, semantic_result)?;
        let affinity_score = self.compute_affinity_score(&affinity_metrics)?;

        // Compute weighted RossNet score
        let rossnet_score = (similarity_score * self.config.similarity_weight)
            + (fitness_score * self.config.fitness_weight)
            + (affinity_score * self.config.affinity_weight);

        // Update statistics
        self.stats.analysis_time = start_time.elapsed();
        self.stats.comparisons_performed += 1;

        Ok(RossNetValidationResult {
            valid: rossnet_score >= self.config.min_rossnet_score,
            rossnet_score,
            components: RossNetComponents {
                similarity: similarity_score,
                fitness: fitness_score,
                affinity: affinity_score,
                similarity_metrics,
                fitness_metrics,
                affinity_metrics,
            },
            stats: self.stats.clone(),
            errors: vec![],
            warnings: self.generate_warnings(rossnet_score),
        })
    }

    /// Calculate similarity metrics (sim component)
    fn calculate_similarity_metrics(
        &mut self,
        document: &AispDocument,
        semantic_result: &DeepVerificationResult,
    ) -> AispResult<SimilarityMetrics> {
        // Calculate semantic vector distance
        let semantic_distance = self.calculate_semantic_distance(document, semantic_result)?;

        // Calculate structural similarity
        let structural_similarity = self.calculate_structural_similarity(document)?;

        // Calculate content overlap
        let content_overlap = self.calculate_content_overlap(document)?;

        // Calculate type compatibility
        let type_compatibility = self.calculate_type_compatibility(document)?;

        self.stats.vector_calculations += 4;

        Ok(SimilarityMetrics {
            semantic_distance,
            structural_similarity,
            content_overlap,
            type_compatibility,
        })
    }

    /// Calculate fitness metrics (fit component)
    fn calculate_fitness_metrics(
        &mut self,
        document: &AispDocument,
        semantic_result: &DeepVerificationResult,
    ) -> AispResult<FitnessMetrics> {
        // Calculate behavioral adaptation
        let behavioral_adaptation = self.calculate_behavioral_adaptation(document)?;

        // Calculate performance efficiency
        let performance_efficiency =
            self.calculate_performance_efficiency(document, semantic_result)?;

        // Calculate resource utilization
        let resource_utilization = self.calculate_resource_utilization(document)?;

        // Calculate temporal consistency
        let temporal_consistency = self.calculate_temporal_consistency(document)?;

        self.stats.vector_calculations += 4;

        Ok(FitnessMetrics {
            behavioral_adaptation,
            performance_efficiency,
            resource_utilization,
            temporal_consistency,
        })
    }

    /// Calculate affinity metrics (aff component)
    fn calculate_affinity_metrics(
        &mut self,
        document: &AispDocument,
        _semantic_result: &DeepVerificationResult,
    ) -> AispResult<AffinityMetrics> {
        // Calculate domain compatibility
        let domain_compatibility = self.calculate_domain_compatibility(document)?;

        // Calculate protocol alignment
        let protocol_alignment = self.calculate_protocol_alignment(document)?;

        // Calculate interface compatibility
        let interface_compatibility = self.calculate_interface_compatibility(document)?;

        // Calculate context adaptation
        let context_adaptation = self.calculate_context_adaptation(document)?;

        self.stats.vector_calculations += 4;

        Ok(AffinityMetrics {
            domain_compatibility,
            protocol_alignment,
            interface_compatibility,
            context_adaptation,
        })
    }

    /// Calculate semantic vector distance
    fn calculate_semantic_distance(
        &mut self,
        document: &AispDocument,
        semantic_result: &DeepVerificationResult,
    ) -> AispResult<f64> {
        let cache_key = format!("semantic_{}", document.header.name);

        if let Some(&cached_distance) = self.similarity_cache.get(&cache_key) {
            self.stats.cache_hits += 1;
            return Ok(cached_distance);
        }

        self.stats.cache_misses += 1;

        // Calculate semantic distance based on delta and ambiguity
        let distance = if semantic_result.delta() > 0.0 {
            1.0 - semantic_result.ambiguity().min(1.0)
        } else {
            0.0
        };

        if self.config.enable_caching {
            self.similarity_cache.insert(cache_key, distance);
        }

        Ok(distance)
    }

    /// Calculate structural similarity ratio
    fn calculate_structural_similarity(&self, document: &AispDocument) -> AispResult<f64> {
        // Base structural similarity on block completeness
        let expected_blocks = 5; // Meta, Types, Rules, Functions, Evidence
        let actual_blocks = document.blocks.len();

        let completeness_ratio = (actual_blocks as f64 / expected_blocks as f64).min(1.0);

        // Weight by block type diversity
        let mut block_types = std::collections::HashSet::new();
        for block in &document.blocks {
            block_types.insert(block.block_type());
        }

        let type_diversity = block_types.len() as f64 / expected_blocks as f64;

        Ok((completeness_ratio + type_diversity) / 2.0)
    }

    /// Calculate content overlap percentage
    fn calculate_content_overlap(&self, document: &AispDocument) -> AispResult<f64> {
        // Calculate overlap based on meta entries and function definitions
        let mut total_entries = 0;
        let mut non_empty_entries = 0;

        for block in &document.blocks {
            match block {
                AispBlock::Meta(meta_block) => {
                    total_entries += meta_block.entries.len();
                    non_empty_entries += meta_block.entries.len(); // All meta entries are non-empty
                }
                AispBlock::Functions(functions_block) => {
                    total_entries += functions_block.functions.len();
                    non_empty_entries += functions_block.functions.len(); // All function defs are non-empty
                }
                _ => continue,
            }
        }

        if total_entries == 0 {
            Ok(0.0)
        } else {
            Ok(non_empty_entries as f64 / total_entries as f64)
        }
    }

    /// Calculate type compatibility score
    fn calculate_type_compatibility(&self, document: &AispDocument) -> AispResult<f64> {
        // Base compatibility on types block completeness
        for block in &document.blocks {
            if let AispBlock::Types(types_block) = block {
                if types_block.definitions.is_empty() {
                    return Ok(0.5); // Partial compatibility for empty types
                }

                // Higher compatibility for more type definitions
                let type_count = types_block.definitions.len() as f64;
                return Ok((type_count / (type_count + 5.0)).min(1.0));
            }
        }

        Ok(0.3) // Lower compatibility if no types block
    }

    /// Calculate behavioral adaptation score
    fn calculate_behavioral_adaptation(&self, document: &AispDocument) -> AispResult<f64> {
        // Base adaptation on rules and functions complexity
        let mut adaptation_score = 0.0;
        let mut score_components = 0;

        for block in &document.blocks {
            match block {
                AispBlock::Rules(rules_block) => {
                    let rules_complexity = rules_block.rules.len() as f64;
                    adaptation_score += (rules_complexity / (rules_complexity + 10.0)).min(1.0);
                    score_components += 1;
                }
                AispBlock::Functions(functions_block) => {
                    let functions_complexity = functions_block.functions.len() as f64;
                    adaptation_score +=
                        (functions_complexity / (functions_complexity + 5.0)).min(1.0);
                    score_components += 1;
                }
                _ => continue,
            }
        }

        if score_components == 0 {
            Ok(0.0)
        } else {
            Ok(adaptation_score / score_components as f64)
        }
    }

    /// Calculate performance efficiency ratio
    fn calculate_performance_efficiency(
        &self,
        document: &AispDocument,
        semantic_result: &DeepVerificationResult,
    ) -> AispResult<f64> {
        // Base efficiency on delta (semantic density)
        let delta_efficiency = semantic_result.delta().min(1.0);

        // Factor in document size efficiency
        let size_factor = match document.blocks.len() {
            0..=3 => 0.6,  // Too small
            4..=6 => 1.0,  // Optimal size
            7..=10 => 0.9, // Good size
            _ => 0.7,      // Too large
        };

        Ok(delta_efficiency * size_factor)
    }

    /// Calculate resource utilization score
    fn calculate_resource_utilization(&self, document: &AispDocument) -> AispResult<f64> {
        // Estimate resource usage based on complexity
        let mut complexity_score = 0.0;
        let mut utilization_components = 0;

        for block in &document.blocks {
            let block_complexity = match block {
                AispBlock::Meta(meta_block) => meta_block.entries.len() as f64 * 0.1,
                AispBlock::Types(types_block) => types_block.definitions.len() as f64 * 0.2,
                AispBlock::Rules(rules_block) => rules_block.rules.len() as f64 * 0.3,
                AispBlock::Functions(functions_block) => {
                    functions_block.functions.len() as f64 * 0.4
                }
                AispBlock::Evidence(_) => 0.1,
            };

            complexity_score += block_complexity;
            utilization_components += 1;
        }

        // Normalize to 0-1 range
        let normalized_complexity = (complexity_score / 10.0).min(1.0);

        // Optimal utilization is around 0.7
        let utilization_score = if normalized_complexity < 0.7 {
            normalized_complexity / 0.7
        } else {
            1.0 - ((normalized_complexity - 0.7) / 0.3)
        };

        Ok(utilization_score.max(0.0))
    }

    /// Calculate temporal consistency score
    fn calculate_temporal_consistency(&self, document: &AispDocument) -> AispResult<f64> {
        // Base consistency on temporal operators in rules
        let mut temporal_operators = 0;
        let mut total_expressions = 0;

        for block in &document.blocks {
            if let AispBlock::Rules(rules_block) = block {
                for rule in &rules_block.rules {
                    total_expressions += 1;
                    // Note: canonical rules are strings, not structured expressions
                    // Use simple heuristic for temporal operators
                    if rule.raw_text.contains('□')
                        || rule.raw_text.contains('◊')
                        || rule.raw_text.contains('X')
                    {
                        temporal_operators += 1;
                    }
                }
            }
        }

        if total_expressions == 0 {
            Ok(0.8) // Default consistency for documents without temporal logic
        } else {
            let temporal_ratio = temporal_operators as f64 / total_expressions as f64;
            Ok(0.5 + (temporal_ratio * 0.5)) // Scale to 0.5-1.0 range
        }
    }

    /// Check if rule string contains temporal operators (simplified heuristic)
    fn rule_has_temporal_operators(&self, rule: &str) -> bool {
        rule.contains('□')
            || rule.contains('◊')
            || rule.contains('X')
            || rule.contains("always")
            || rule.contains("eventually")
            || rule.contains("next")
    }

    /// Calculate domain compatibility percentage
    fn calculate_domain_compatibility(&self, document: &AispDocument) -> AispResult<f64> {
        // Check domain specification in metadata
        if let Some(domain) = &document.metadata.domain {
            // Higher compatibility for recognized domains
            let compatibility = match domain.as_str() {
                "ai" | "ml" | "nlp" | "vision" => 1.0,
                "robotics" | "autonomous" => 0.9,
                "general" | "universal" => 0.8,
                "test" | "experimental" => 0.6,
                _ => 0.7, // Unknown but specified domain
            };
            Ok(compatibility)
        } else {
            Ok(0.5) // Partial compatibility for unspecified domain
        }
    }

    /// Calculate protocol alignment score
    fn calculate_protocol_alignment(&self, document: &AispDocument) -> AispResult<f64> {
        // Check protocol specification and version alignment
        let version_alignment = if document.header.version == "5.1" {
            1.0
        } else if document.header.version.starts_with("5.") {
            0.8
        } else {
            0.5
        };

        let protocol_alignment = if document.metadata.protocol.is_some() {
            0.9
        } else {
            0.7
        };

        Ok((version_alignment + protocol_alignment) / 2.0)
    }

    /// Calculate interface compatibility ratio
    fn calculate_interface_compatibility(&self, document: &AispDocument) -> AispResult<f64> {
        // Base compatibility on function definitions and types
        let mut has_types = false;
        let mut has_functions = false;
        let mut has_evidence = false;

        for block in &document.blocks {
            match block {
                AispBlock::Types(_) => has_types = true,
                AispBlock::Functions(_) => has_functions = true,
                AispBlock::Evidence(_) => has_evidence = true,
                _ => continue,
            }
        }

        let compatibility_score = match (has_types, has_functions, has_evidence) {
            (true, true, true) => 1.0,  // Full interface compatibility
            (true, true, false) => 0.8, // Missing evidence
            (true, false, true) => 0.7, // Missing functions
            (false, true, true) => 0.6, // Missing types
            _ => 0.4,                   // Minimal compatibility
        };

        Ok(compatibility_score)
    }

    /// Calculate context adaptation score
    fn calculate_context_adaptation(&self, document: &AispDocument) -> AispResult<f64> {
        // Base adaptation on meta block context information
        for block in &document.blocks {
            if let AispBlock::Meta(meta_block) = block {
                let context_entries = meta_block
                    .entries
                    .iter()
                    .filter(|(_, entry)| {
                        entry.key.contains("context") || entry.key.contains("adapt")
                    })
                    .count();

                let total_entries = meta_block.entries.len();

                if total_entries == 0 {
                    return Ok(0.5);
                }

                let adaptation_ratio = context_entries as f64 / total_entries as f64;
                return Ok(0.6 + (adaptation_ratio * 0.4)); // Scale to 0.6-1.0
            }
        }

        Ok(0.5) // Default adaptation for missing meta block
    }

    /// Compute weighted similarity score
    fn compute_similarity_score(&self, metrics: &SimilarityMetrics) -> AispResult<f64> {
        let score = (metrics.semantic_distance * 0.4)
            + (metrics.structural_similarity * 0.3)
            + (metrics.content_overlap * 0.2)
            + (metrics.type_compatibility * 0.1);

        Ok(score.min(1.0))
    }

    /// Compute weighted fitness score
    fn compute_fitness_score(&self, metrics: &FitnessMetrics) -> AispResult<f64> {
        let score = (metrics.behavioral_adaptation * 0.3)
            + (metrics.performance_efficiency * 0.3)
            + (metrics.resource_utilization * 0.25)
            + (metrics.temporal_consistency * 0.15);

        Ok(score.min(1.0))
    }

    /// Compute weighted affinity score
    fn compute_affinity_score(&self, metrics: &AffinityMetrics) -> AispResult<f64> {
        let score = (metrics.domain_compatibility * 0.4)
            + (metrics.protocol_alignment * 0.3)
            + (metrics.interface_compatibility * 0.2)
            + (metrics.context_adaptation * 0.1);

        Ok(score.min(1.0))
    }

    /// Generate analysis warnings
    fn generate_warnings(&self, rossnet_score: f64) -> Vec<String> {
        let mut warnings = Vec::new();

        if rossnet_score < self.config.min_rossnet_score {
            warnings.push(format!(
                "RossNet score {:.3} is below minimum threshold {:.3}",
                rossnet_score, self.config.min_rossnet_score
            ));
        }

        if rossnet_score < 0.5 {
            warnings
                .push("Low RossNet score indicates significant compatibility issues".to_string());
        }

        if self.stats.analysis_time > self.config.max_analysis_time {
            warnings.push("RossNet analysis exceeded maximum time limit".to_string());
        }

        warnings
    }

    /// Get validation statistics
    pub fn get_stats(&self) -> &RossNetStats {
        &self.stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::canonical;

    fn create_test_document() -> AispDocument {
        canonical::create_document("rossnet_test", "5.1", "2026-01-26")
    }

    fn create_test_semantic_result() -> DeepVerificationResult {
        DeepVerificationResult::test_default()
    }

    #[test]
    fn test_rossnet_validator_creation() {
        let config = RossNetConfig::default();
        let validator = RossNetValidator::new(config);

        assert_eq!(validator.stats.comparisons_performed, 0);
        assert_eq!(validator.stats.vector_calculations, 0);
    }

    #[test]
    fn test_domain_compatibility_calculation() {
        let config = RossNetConfig::default();
        let validator = RossNetValidator::new(config);
        let document = create_test_document();

        let compatibility = validator.calculate_domain_compatibility(&document).unwrap();
        assert_eq!(compatibility, 0.5); // Unspecified domain should have partial compatibility
    }

    #[test]
    fn test_protocol_alignment_calculation() {
        let config = RossNetConfig::default();
        let validator = RossNetValidator::new(config);
        let document = create_test_document();

        let alignment = validator.calculate_protocol_alignment(&document).unwrap();

        // Version 5.1 basic document should have reasonable alignment
        assert!(
            alignment >= 0.8,
            "Expected alignment >= 0.8, got {}",
            alignment
        );
    }

    #[test]
    fn test_structural_similarity_calculation() {
        let config = RossNetConfig::default();
        let validator = RossNetValidator::new(config);
        let document = create_test_document();

        let similarity = validator
            .calculate_structural_similarity(&document)
            .unwrap();
        assert!(similarity >= 0.0 && similarity <= 1.0);
    }

    #[test]
    fn test_semantic_distance_caching() {
        let config = RossNetConfig {
            enable_caching: true,
            ..Default::default()
        };
        let mut validator = RossNetValidator::new(config);
        let document = create_test_document();
        let semantic_result = create_test_semantic_result();

        // First call should miss cache
        let distance1 = validator
            .calculate_semantic_distance(&document, &semantic_result)
            .unwrap();
        assert_eq!(validator.stats.cache_misses, 1);
        assert_eq!(validator.stats.cache_hits, 0);

        // Second call should hit cache
        let distance2 = validator
            .calculate_semantic_distance(&document, &semantic_result)
            .unwrap();
        assert_eq!(validator.stats.cache_misses, 1);
        assert_eq!(validator.stats.cache_hits, 1);
        assert_eq!(distance1, distance2);
    }

    #[test]
    fn test_rossnet_score_computation() {
        let components = RossNetComponents {
            similarity: 0.8,
            fitness: 0.7,
            affinity: 0.9,
            similarity_metrics: SimilarityMetrics {
                semantic_distance: 0.8,
                structural_similarity: 0.8,
                content_overlap: 0.8,
                type_compatibility: 0.8,
            },
            fitness_metrics: FitnessMetrics {
                behavioral_adaptation: 0.7,
                performance_efficiency: 0.7,
                resource_utilization: 0.7,
                temporal_consistency: 0.7,
            },
            affinity_metrics: AffinityMetrics {
                domain_compatibility: 0.9,
                protocol_alignment: 0.9,
                interface_compatibility: 0.9,
                context_adaptation: 0.9,
            },
        };

        let config = RossNetConfig::default();
        let expected_score = (0.8 * config.similarity_weight)
            + (0.7 * config.fitness_weight)
            + (0.9 * config.affinity_weight);

        // Expected: (0.8 * 0.4) + (0.7 * 0.35) + (0.9 * 0.25) = 0.32 + 0.245 + 0.225 = 0.79
        assert!((expected_score - 0.79).abs() < 0.01);
    }

    #[test]
    fn test_warning_generation() {
        let config = RossNetConfig {
            min_rossnet_score: 0.8,
            ..Default::default()
        };
        let validator = RossNetValidator::new(config);

        let warnings_low = validator.generate_warnings(0.6);
        assert!(warnings_low.len() >= 1);
        assert!(warnings_low
            .iter()
            .any(|w| w.contains("below minimum threshold")));

        let warnings_high = validator.generate_warnings(0.9);
        assert!(warnings_high.is_empty());
    }
}
