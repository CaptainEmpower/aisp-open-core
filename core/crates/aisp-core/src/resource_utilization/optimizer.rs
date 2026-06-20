//! Resource Optimization Engine
//!
//! Provides optimization recommendations and strategies for resource utilization.

use super::types::*;
use crate::error::AispResult;
use std::collections::HashMap;

/// Resource optimization engine
pub struct ResourceOptimizer {
    /// Optimization strategies
    strategies: Vec<OptimizationStrategy>,
    /// Configuration
    config: OptimizerConfig,
}

/// Configuration for resource optimizer
#[derive(Debug, Clone)]
pub struct OptimizerConfig {
    /// Maximum number of recommendations per resource
    pub max_recommendations_per_resource: usize,
    /// Minimum improvement threshold for recommendations
    pub min_improvement_threshold: f64,
    /// Enable aggressive optimizations
    pub enable_aggressive_optimizations: bool,
}

/// Optimization strategy definition
#[derive(Debug, Clone)]
pub struct OptimizationStrategy {
    /// Strategy name
    pub name: String,
    /// Applicable resource types
    pub applicable_resources: Vec<ResourceType>,
    /// Optimization approach
    pub approach: OptimizationType,
    /// Expected improvement range
    pub expected_improvement: (f64, f64),
    /// Implementation complexity
    pub complexity: ImplementationDifficulty,
    /// Prerequisites for applying this strategy
    pub prerequisites: Vec<String>,
}

impl ResourceOptimizer {
    /// Create new optimizer with default strategies
    pub fn new() -> Self {
        Self {
            strategies: Self::default_strategies(),
            config: OptimizerConfig::default(),
        }
    }

    /// Create optimizer with custom configuration
    pub fn with_config(config: OptimizerConfig) -> Self {
        Self {
            strategies: Self::default_strategies(),
            config,
        }
    }

    /// Generate optimization recommendations for analysis
    pub fn optimize(&self, analysis: &ResourceUtilizationAnalysis) -> AispResult<OptimizationPlan> {
        let mut plan = OptimizationPlan {
            total_recommendations: 0,
            estimated_total_improvement: 0.0,
            resource_optimizations: HashMap::new(),
            implementation_timeline: Vec::new(),
            cost_benefit_analysis: CostBenefitAnalysis::default(),
        };

        // Generate recommendations for each resource type
        for (resource_type, resource_analysis) in &analysis.resource_analysis {
            let recommendations = self.generate_resource_recommendations(
                resource_type,
                resource_analysis,
                &analysis.bottlenecks,
            )?;

            if !recommendations.is_empty() {
                plan.resource_optimizations
                    .insert(resource_type.clone(), recommendations);
            }
        }

        // Calculate total metrics
        plan.total_recommendations = plan
            .resource_optimizations
            .values()
            .map(|recs| recs.len())
            .sum();

        plan.estimated_total_improvement = plan
            .resource_optimizations
            .values()
            .flatten()
            .map(|rec| rec.estimated_improvement)
            .sum();

        // Generate implementation timeline
        plan.implementation_timeline = self.generate_implementation_timeline(&plan)?;

        // Perform cost-benefit analysis
        plan.cost_benefit_analysis = self.analyze_cost_benefit(&plan)?;

        Ok(plan)
    }

    /// Generate recommendations for specific resource
    fn generate_resource_recommendations(
        &self,
        resource_type: &ResourceType,
        analysis: &ResourceTypeAnalysis,
        bottlenecks: &[ResourceBottleneck],
    ) -> AispResult<Vec<OptimizationRecommendation>> {
        let mut recommendations = Vec::new();

        // Check if this resource has bottlenecks
        let has_bottleneck = bottlenecks
            .iter()
            .any(|b| &b.resource_type == resource_type);

        // Find applicable strategies
        for strategy in &self.strategies {
            if strategy.applicable_resources.contains(resource_type) {
                // Determine if strategy is applicable based on current state
                if self.is_strategy_applicable(strategy, analysis, has_bottleneck) {
                    let recommendation = self.create_recommendation_from_strategy(
                        strategy,
                        resource_type,
                        analysis,
                        has_bottleneck,
                    )?;
                    recommendations.push(recommendation);
                }
            }
        }

        // Sort by estimated improvement and limit count
        recommendations.sort_by(|a, b| {
            b.estimated_improvement
                .partial_cmp(&a.estimated_improvement)
                .unwrap()
        });
        recommendations.truncate(self.config.max_recommendations_per_resource);

        Ok(recommendations)
    }

    /// Check if optimization strategy is applicable
    fn is_strategy_applicable(
        &self,
        strategy: &OptimizationStrategy,
        analysis: &ResourceTypeAnalysis,
        has_bottleneck: bool,
    ) -> bool {
        match strategy.approach {
            OptimizationType::ScaleUp => has_bottleneck && analysis.current_utilization > 0.8,
            OptimizationType::ScaleDown => !has_bottleneck && analysis.current_utilization < 0.3,
            OptimizationType::LoadBalance => {
                analysis.current_utilization > 0.7
                    && matches!(analysis.trend, UtilizationTrend::Volatile(_))
            }
            OptimizationType::Caching => analysis.current_utilization > 0.6,
            OptimizationType::Compression => {
                matches!(
                    analysis.resource_type,
                    ResourceType::Memory | ResourceType::NetworkBandwidth
                )
            }
            OptimizationType::Parallelization => {
                matches!(analysis.resource_type, ResourceType::CPU)
                    && analysis.current_utilization > 0.7
            }
            _ => true, // Other optimizations are generally applicable
        }
    }

    /// Create recommendation from strategy
    fn create_recommendation_from_strategy(
        &self,
        strategy: &OptimizationStrategy,
        resource_type: &ResourceType,
        analysis: &ResourceTypeAnalysis,
        has_bottleneck: bool,
    ) -> AispResult<OptimizationRecommendation> {
        let priority = if has_bottleneck {
            match analysis.current_utilization {
                u if u > 0.9 => RecommendationPriority::Critical,
                u if u > 0.8 => RecommendationPriority::High,
                _ => RecommendationPriority::Medium,
            }
        } else {
            RecommendationPriority::Low
        };

        let improvement_factor = if has_bottleneck {
            strategy.expected_improvement.1 // Use upper bound for bottlenecks
        } else {
            (strategy.expected_improvement.0 + strategy.expected_improvement.1) / 2.0
            // Use average
        };

        Ok(OptimizationRecommendation {
            resource_type: resource_type.clone(),
            optimization_type: strategy.approach.clone(),
            estimated_improvement: improvement_factor,
            difficulty: strategy.complexity.clone(),
            description: format!(
                "{} for {:?}: {}",
                strategy.name,
                resource_type,
                self.get_strategy_description(&strategy.approach, resource_type)
            ),
            priority,
        })
    }

    /// Get description for optimization strategy
    fn get_strategy_description(
        &self,
        optimization_type: &OptimizationType,
        resource_type: &ResourceType,
    ) -> String {
        match optimization_type {
            OptimizationType::ScaleUp => format!(
                "Increase {:?} capacity to handle higher load",
                resource_type
            ),
            OptimizationType::ScaleDown => {
                format!("Reduce {:?} allocation to optimize costs", resource_type)
            }
            OptimizationType::LoadBalance => {
                format!("Distribute {:?} load more evenly", resource_type)
            }
            OptimizationType::Caching => {
                format!("Implement caching to reduce {:?} pressure", resource_type)
            }
            OptimizationType::Compression => {
                format!("Apply compression to optimize {:?} usage", resource_type)
            }
            OptimizationType::Parallelization => format!(
                "Use parallel processing to utilize {:?} more efficiently",
                resource_type
            ),
            OptimizationType::Algorithm => {
                format!("Optimize algorithms to reduce {:?} usage", resource_type)
            }
            OptimizationType::DataStructure => format!(
                "Optimize data structures for better {:?} utilization",
                resource_type
            ),
            OptimizationType::Custom(desc) => desc.clone(),
        }
    }

    /// Generate implementation timeline
    fn generate_implementation_timeline(
        &self,
        plan: &OptimizationPlan,
    ) -> AispResult<Vec<TimelineItem>> {
        let mut timeline = Vec::new();
        let mut current_week = 1;

        // Group recommendations by difficulty and priority
        let mut all_recommendations: Vec<&OptimizationRecommendation> =
            plan.resource_optimizations.values().flatten().collect();

        // Sort by priority first, then by difficulty
        all_recommendations.sort_by(|a, b| {
            b.priority
                .cmp(&a.priority)
                .then(a.difficulty.cmp(&b.difficulty))
        });

        for recommendation in all_recommendations {
            let duration = match recommendation.difficulty {
                ImplementationDifficulty::Trivial => 1,
                ImplementationDifficulty::Easy => 2,
                ImplementationDifficulty::Moderate => 4,
                ImplementationDifficulty::Hard => 8,
                ImplementationDifficulty::Expert => 12,
            };

            timeline.push(TimelineItem {
                week_start: current_week,
                week_end: current_week + duration - 1,
                description: recommendation.description.clone(),
                resource_type: recommendation.resource_type.clone(),
                priority: recommendation.priority.clone(),
                estimated_improvement: recommendation.estimated_improvement,
            });

            current_week += duration;
        }

        Ok(timeline)
    }

    /// Analyze cost-benefit of optimization plan
    fn analyze_cost_benefit(&self, plan: &OptimizationPlan) -> AispResult<CostBenefitAnalysis> {
        let mut total_cost = 0.0;
        let total_benefit = plan.estimated_total_improvement;

        for recommendations in plan.resource_optimizations.values() {
            for recommendation in recommendations {
                let cost = match recommendation.difficulty {
                    ImplementationDifficulty::Trivial => 1000.0,
                    ImplementationDifficulty::Easy => 5000.0,
                    ImplementationDifficulty::Moderate => 15000.0,
                    ImplementationDifficulty::Hard => 40000.0,
                    ImplementationDifficulty::Expert => 80000.0,
                };
                total_cost += cost;
            }
        }

        let roi = if total_cost > 0.0 {
            (total_benefit * 50000.0 - total_cost) / total_cost // Assume $50k value per improvement point
        } else {
            0.0
        };

        Ok(CostBenefitAnalysis {
            total_implementation_cost: total_cost,
            expected_annual_savings: total_benefit * 30000.0, // Assume $30k savings per improvement point
            return_on_investment: roi,
            payback_period_months: if total_benefit > 0.0 {
                total_cost / (total_benefit * 2500.0)
            } else {
                f64::INFINITY
            },
            risk_level: if total_cost > 100000.0 {
                RiskLevel::High
            } else if total_cost > 25000.0 {
                RiskLevel::Medium
            } else {
                RiskLevel::Low
            },
        })
    }

    /// Create default optimization strategies
    fn default_strategies() -> Vec<OptimizationStrategy> {
        vec![
            OptimizationStrategy {
                name: "Memory Caching".to_string(),
                applicable_resources: vec![ResourceType::Memory, ResourceType::DiskIO],
                approach: OptimizationType::Caching,
                expected_improvement: (0.15, 0.35),
                complexity: ImplementationDifficulty::Moderate,
                prerequisites: vec!["Identified cacheable data patterns".to_string()],
            },
            OptimizationStrategy {
                name: "CPU Parallelization".to_string(),
                applicable_resources: vec![ResourceType::CPU],
                approach: OptimizationType::Parallelization,
                expected_improvement: (0.25, 0.50),
                complexity: ImplementationDifficulty::Hard,
                prerequisites: vec![
                    "Parallelizable workload".to_string(),
                    "Multi-core availability".to_string(),
                ],
            },
            OptimizationStrategy {
                name: "Resource Scaling".to_string(),
                applicable_resources: vec![
                    ResourceType::Memory,
                    ResourceType::CPU,
                    ResourceType::NetworkBandwidth,
                ],
                approach: OptimizationType::ScaleUp,
                expected_improvement: (0.20, 0.40),
                complexity: ImplementationDifficulty::Easy,
                prerequisites: vec!["Scalable infrastructure".to_string()],
            },
            OptimizationStrategy {
                name: "Load Balancing".to_string(),
                applicable_resources: vec![ResourceType::CPU, ResourceType::NetworkBandwidth],
                approach: OptimizationType::LoadBalance,
                expected_improvement: (0.15, 0.30),
                complexity: ImplementationDifficulty::Moderate,
                prerequisites: vec!["Multiple processing units".to_string()],
            },
            OptimizationStrategy {
                name: "Data Compression".to_string(),
                applicable_resources: vec![
                    ResourceType::Memory,
                    ResourceType::NetworkBandwidth,
                    ResourceType::DiskIO,
                ],
                approach: OptimizationType::Compression,
                expected_improvement: (0.10, 0.25),
                complexity: ImplementationDifficulty::Easy,
                prerequisites: vec!["Compressible data types".to_string()],
            },
        ]
    }
}

/// Complete optimization plan
#[derive(Debug, Clone)]
pub struct OptimizationPlan {
    /// Total number of recommendations
    pub total_recommendations: usize,
    /// Estimated total improvement
    pub estimated_total_improvement: f64,
    /// Recommendations per resource type
    pub resource_optimizations: HashMap<ResourceType, Vec<OptimizationRecommendation>>,
    /// Implementation timeline
    pub implementation_timeline: Vec<TimelineItem>,
    /// Cost-benefit analysis
    pub cost_benefit_analysis: CostBenefitAnalysis,
}

/// Timeline item for implementation
#[derive(Debug, Clone)]
pub struct TimelineItem {
    /// Starting week
    pub week_start: usize,
    /// Ending week
    pub week_end: usize,
    /// Description of work
    pub description: String,
    /// Target resource type
    pub resource_type: ResourceType,
    /// Priority level
    pub priority: RecommendationPriority,
    /// Expected improvement
    pub estimated_improvement: f64,
}

/// Cost-benefit analysis
#[derive(Debug, Clone)]
pub struct CostBenefitAnalysis {
    /// Total implementation cost
    pub total_implementation_cost: f64,
    /// Expected annual savings
    pub expected_annual_savings: f64,
    /// Return on investment percentage
    pub return_on_investment: f64,
    /// Payback period in months
    pub payback_period_months: f64,
    /// Implementation risk level
    pub risk_level: RiskLevel,
}

/// Risk levels for optimization implementations
#[derive(Debug, Clone, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

impl Default for OptimizerConfig {
    fn default() -> Self {
        Self {
            max_recommendations_per_resource: 5,
            min_improvement_threshold: 0.05,
            enable_aggressive_optimizations: false,
        }
    }
}

impl Default for CostBenefitAnalysis {
    fn default() -> Self {
        Self {
            total_implementation_cost: 0.0,
            expected_annual_savings: 0.0,
            return_on_investment: 0.0,
            payback_period_months: f64::INFINITY,
            risk_level: RiskLevel::Low,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimizer_creation() {
        let optimizer = ResourceOptimizer::new();
        assert!(!optimizer.strategies.is_empty());
        assert_eq!(optimizer.config.max_recommendations_per_resource, 5);
        assert!(!optimizer.config.enable_aggressive_optimizations);
    }

    #[test]
    fn test_default_strategies() {
        let strategies = ResourceOptimizer::default_strategies();
        assert_eq!(strategies.len(), 5);

        let caching_strategy = strategies
            .iter()
            .find(|s| s.name == "Memory Caching")
            .unwrap();

        assert!(matches!(
            caching_strategy.approach,
            OptimizationType::Caching
        ));
        assert_eq!(
            caching_strategy.complexity,
            ImplementationDifficulty::Moderate
        );
        assert!(caching_strategy
            .applicable_resources
            .contains(&ResourceType::Memory));
    }

    #[test]
    fn test_strategy_applicability() {
        let optimizer = ResourceOptimizer::new();
        let strategy = &optimizer.strategies[0]; // Memory Caching strategy

        // High utilization memory should be applicable for caching
        let high_utilization_analysis = ResourceTypeAnalysis {
            resource_type: ResourceType::Memory,
            current_utilization: 0.8,
            peak_utilization: 0.9,
            average_utilization: 0.75,
            capacity: 1000.0,
            current_allocation: 800.0,
            trend: UtilizationTrend::Increasing(0.05),
            metrics: HashMap::new(),
        };

        assert!(optimizer.is_strategy_applicable(strategy, &high_utilization_analysis, true));

        // Low utilization should not be applicable for scale-up strategies
        let low_utilization_analysis = ResourceTypeAnalysis {
            resource_type: ResourceType::Memory,
            current_utilization: 0.2,
            peak_utilization: 0.3,
            average_utilization: 0.25,
            capacity: 1000.0,
            current_allocation: 200.0,
            trend: UtilizationTrend::Stable(0.02),
            metrics: HashMap::new(),
        };

        let scale_up_strategy = OptimizationStrategy {
            name: "Scale Up".to_string(),
            applicable_resources: vec![ResourceType::Memory],
            approach: OptimizationType::ScaleUp,
            expected_improvement: (0.2, 0.4),
            complexity: ImplementationDifficulty::Easy,
            prerequisites: vec![],
        };

        assert!(!optimizer.is_strategy_applicable(
            &scale_up_strategy,
            &low_utilization_analysis,
            false
        ));
    }

    #[test]
    fn test_cost_benefit_analysis() {
        let optimizer = ResourceOptimizer::new();

        let mut plan = OptimizationPlan {
            total_recommendations: 3,
            estimated_total_improvement: 0.6,
            resource_optimizations: HashMap::new(),
            implementation_timeline: Vec::new(),
            cost_benefit_analysis: CostBenefitAnalysis::default(),
        };

        // Add some test recommendations
        let recommendations = [OptimizationRecommendation {
                resource_type: ResourceType::Memory,
                optimization_type: OptimizationType::Caching,
                estimated_improvement: 0.2,
                difficulty: ImplementationDifficulty::Moderate,
                description: "Test recommendation".to_string(),
                priority: RecommendationPriority::High,
            },
            OptimizationRecommendation {
                resource_type: ResourceType::CPU,
                optimization_type: OptimizationType::Parallelization,
                estimated_improvement: 0.4,
                difficulty: ImplementationDifficulty::Hard,
                description: "Test recommendation 2".to_string(),
                priority: RecommendationPriority::Medium,
            }];

        plan.resource_optimizations
            .insert(ResourceType::Memory, vec![recommendations[0].clone()]);
        plan.resource_optimizations
            .insert(ResourceType::CPU, vec![recommendations[1].clone()]);

        let cost_benefit = optimizer.analyze_cost_benefit(&plan).unwrap();

        assert!(cost_benefit.total_implementation_cost > 0.0);
        assert!(cost_benefit.expected_annual_savings > 0.0);
        assert_ne!(cost_benefit.payback_period_months, f64::INFINITY);
    }

    #[test]
    fn test_timeline_generation() {
        let optimizer = ResourceOptimizer::new();

        let mut plan = OptimizationPlan {
            total_recommendations: 2,
            estimated_total_improvement: 0.4,
            resource_optimizations: HashMap::new(),
            implementation_timeline: Vec::new(),
            cost_benefit_analysis: CostBenefitAnalysis::default(),
        };

        let recommendations = vec![
            OptimizationRecommendation {
                resource_type: ResourceType::Memory,
                optimization_type: OptimizationType::Caching,
                estimated_improvement: 0.2,
                difficulty: ImplementationDifficulty::Easy, // 2 weeks
                description: "Easy optimization".to_string(),
                priority: RecommendationPriority::High,
            },
            OptimizationRecommendation {
                resource_type: ResourceType::CPU,
                optimization_type: OptimizationType::Parallelization,
                estimated_improvement: 0.2,
                difficulty: ImplementationDifficulty::Hard, // 8 weeks
                description: "Hard optimization".to_string(),
                priority: RecommendationPriority::Medium,
            },
        ];

        plan.resource_optimizations
            .insert(ResourceType::Memory, vec![recommendations[0].clone()]);
        plan.resource_optimizations
            .insert(ResourceType::CPU, vec![recommendations[1].clone()]);

        let timeline = optimizer.generate_implementation_timeline(&plan).unwrap();

        assert_eq!(timeline.len(), 2);
        // High priority, easy task should come first
        assert_eq!(timeline[0].week_start, 1);
        assert_eq!(timeline[0].week_end, 2);
        assert_eq!(timeline[1].week_start, 3);
        assert_eq!(timeline[1].week_end, 10);
    }
}
