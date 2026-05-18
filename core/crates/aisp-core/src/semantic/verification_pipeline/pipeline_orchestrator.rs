//! Pipeline Orchestrator Module
//!
//! Manages verification workflow, stage dependencies, and execution strategies
//! for the multi-layer verification pipeline.

use super::core_types::*;
use crate::ast::canonical::CanonicalAispDocument as AispDocument;
use crate::error::AispResult;
use std::collections::HashMap;

/// Pipeline orchestrator for managing verification workflow
pub struct PipelineOrchestrator {
    pub verification_stages: Vec<VerificationStage>,
    pub stage_dependencies: HashMap<VerificationStage, Vec<VerificationStage>>,
    pub execution_strategy: ExecutionStrategy,
    pub failure_handling: FailureHandlingStrategy,
    pub resource_manager: ResourceManager,
}

impl PipelineOrchestrator {
    /// Create new pipeline orchestrator with default configuration
    pub fn new() -> Self {
        let verification_stages = vec![
            VerificationStage::Initialize,
            VerificationStage::ParseValidation,
            VerificationStage::SemanticAnalysis,
            VerificationStage::BehavioralVerification,
            VerificationStage::AdversarialTesting,
            VerificationStage::CrossValidation,
            VerificationStage::SecurityEnforcement,
            VerificationStage::ComplianceAudit,
            VerificationStage::PerformanceOptimization,
            VerificationStage::FinalAssessment,
        ];

        let mut stage_dependencies = HashMap::new();

        // Define stage dependencies for proper execution order
        stage_dependencies.insert(
            VerificationStage::SemanticAnalysis,
            vec![VerificationStage::ParseValidation],
        );
        stage_dependencies.insert(
            VerificationStage::BehavioralVerification,
            vec![VerificationStage::ParseValidation],
        );
        stage_dependencies.insert(
            VerificationStage::CrossValidation,
            vec![
                VerificationStage::SemanticAnalysis,
                VerificationStage::BehavioralVerification,
            ],
        );
        stage_dependencies.insert(
            VerificationStage::SecurityEnforcement,
            vec![VerificationStage::CrossValidation],
        );
        stage_dependencies.insert(
            VerificationStage::ComplianceAudit,
            vec![VerificationStage::SecurityEnforcement],
        );

        Self {
            verification_stages,
            stage_dependencies,
            execution_strategy: ExecutionStrategy::Hybrid,
            failure_handling: FailureHandlingStrategy::RiskBasedDecision,
            resource_manager: ResourceManager {
                resource_pools: HashMap::new(),
            },
        }
    }

    /// Create performance-optimized orchestrator
    pub fn new_performance_optimized() -> Self {
        let mut orchestrator = Self::new();
        orchestrator.execution_strategy = ExecutionStrategy::Parallel;
        orchestrator.failure_handling = FailureHandlingStrategy::GracefulDegradation;
        orchestrator.optimize_resource_allocation();
        orchestrator
    }

    /// Initialize verification session with document context
    pub fn initialize_session(&mut self, document: &AispDocument) -> AispResult<String> {
        let session_id = self.generate_session_id(document);
        self.prepare_session_resources(&session_id)?;
        self.log_session_initialization(&session_id);
        Ok(session_id)
    }

    /// Generate unique session identifier
    fn generate_session_id(&self, document: &AispDocument) -> String {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        format!(
            "verification_session_{}_{}",
            document.header.name.chars().take(8).collect::<String>(),
            timestamp
        )
    }

    /// Prepare session-specific resources
    fn prepare_session_resources(&mut self, session_id: &str) -> AispResult<()> {
        // Initialize resource pools for the session
        self.resource_manager.resource_pools.insert(
            format!("{}_memory", session_id),
            1024 * 1024, // 1MB memory pool
        );
        self.resource_manager.resource_pools.insert(
            format!("{}_threads", session_id),
            std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4),
        );
        Ok(())
    }

    /// Log session initialization for audit trail
    fn log_session_initialization(&self, session_id: &str) {
        eprintln!("Verification session initialized: {}", session_id);
        eprintln!("Execution strategy: {:?}", self.execution_strategy);
        eprintln!("Failure handling: {:?}", self.failure_handling);
    }

    /// Optimize resource allocation for performance
    fn optimize_resource_allocation(&mut self) {
        // Increase thread pool for parallel execution
        if let Some(thread_count) = self.resource_manager.resource_pools.get_mut("threads") {
            *thread_count = (*thread_count * 2).min(16); // Cap at 16 threads
        }

        // Allocate larger memory pools for performance
        if let Some(memory_pool) = self.resource_manager.resource_pools.get_mut("memory") {
            *memory_pool = (*memory_pool * 4).min(16 * 1024 * 1024); // Cap at 16MB
        }
    }

    /// Check if stage can be executed based on dependencies
    pub fn can_execute_stage(
        &self,
        stage: &VerificationStage,
        completed_stages: &[VerificationStage],
    ) -> bool {
        if let Some(dependencies) = self.stage_dependencies.get(stage) {
            dependencies
                .iter()
                .all(|dep| completed_stages.contains(dep))
        } else {
            true // No dependencies
        }
    }

    /// Get next executable stages based on completion status
    pub fn get_next_executable_stages(
        &self,
        completed_stages: &[VerificationStage],
    ) -> Vec<VerificationStage> {
        self.verification_stages
            .iter()
            .filter(|stage| {
                !completed_stages.contains(stage) && self.can_execute_stage(stage, completed_stages)
            })
            .cloned()
            .collect()
    }

    /// Validate execution strategy compatibility
    pub fn validate_execution_strategy(&self) -> AispResult<()> {
        match self.execution_strategy {
            ExecutionStrategy::Parallel => {
                if self.has_circular_dependencies() {
                    return Err(crate::error::AispError::internal_error(
                        "Circular dependencies detected - incompatible with parallel execution",
                    ));
                }
            }
            ExecutionStrategy::Adaptive => {
                if self.resource_manager.resource_pools.is_empty() {
                    return Err(crate::error::AispError::internal_error(
                        "Adaptive strategy requires resource pools configuration",
                    ));
                }
            }
            _ => {} // Other strategies don't require special validation
        }
        Ok(())
    }

    /// Check for circular dependencies in stage graph
    fn has_circular_dependencies(&self) -> bool {
        // Simplified cycle detection - in production would use proper graph algorithms
        for (stage, deps) in &self.stage_dependencies {
            if self.has_transitive_dependency(stage, stage, &mut Vec::new()) {
                return true;
            }
        }
        false
    }

    /// Check for transitive dependencies (simplified implementation)
    fn has_transitive_dependency(
        &self,
        current: &VerificationStage,
        target: &VerificationStage,
        visited: &mut Vec<VerificationStage>,
    ) -> bool {
        if visited.contains(current) {
            return current == target;
        }

        visited.push(current.clone());

        if let Some(deps) = self.stage_dependencies.get(current) {
            for dep in deps {
                if dep == target || self.has_transitive_dependency(dep, target, visited) {
                    return true;
                }
            }
        }

        visited.pop();
        false
    }

    /// Finalize session and cleanup resources
    pub fn finalize_session(&mut self, session_id: &str) -> AispResult<()> {
        // Clean up session-specific resources
        let session_keys: Vec<_> = self
            .resource_manager
            .resource_pools
            .keys()
            .filter(|k| k.starts_with(session_id))
            .cloned()
            .collect();

        for key in session_keys {
            self.resource_manager.resource_pools.remove(&key);
        }

        eprintln!("Verification session finalized: {}", session_id);
        Ok(())
    }
}

impl Default for PipelineOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::canonical::{DocumentHeader, DocumentMetadata};

    fn create_test_document() -> AispDocument {
        crate::ast::canonical::create_document("test_doc", "5.1", "2026-01-27")
    }

    #[test]
    fn test_orchestrator_creation() {
        let orchestrator = PipelineOrchestrator::new();
        assert_eq!(orchestrator.verification_stages.len(), 10);
        assert_eq!(orchestrator.execution_strategy, ExecutionStrategy::Hybrid);
        assert_eq!(
            orchestrator.failure_handling,
            FailureHandlingStrategy::RiskBasedDecision
        );
    }

    #[test]
    fn test_performance_optimized_creation() {
        let orchestrator = PipelineOrchestrator::new_performance_optimized();
        assert_eq!(orchestrator.execution_strategy, ExecutionStrategy::Parallel);
        assert_eq!(
            orchestrator.failure_handling,
            FailureHandlingStrategy::GracefulDegradation
        );
    }

    #[test]
    fn test_session_initialization() {
        let mut orchestrator = PipelineOrchestrator::new();
        let document = create_test_document();

        let session_result = orchestrator.initialize_session(&document);
        assert!(session_result.is_ok());

        let session_id = session_result.unwrap();
        assert!(session_id.starts_with("verification_session_"));
        assert!(!session_id.is_empty());
    }

    #[test]
    fn test_stage_dependency_validation() {
        let orchestrator = PipelineOrchestrator::new();
        let completed_stages = vec![
            VerificationStage::Initialize,
            VerificationStage::ParseValidation,
        ];

        // SemanticAnalysis should be executable after ParseValidation
        assert!(
            orchestrator.can_execute_stage(&VerificationStage::SemanticAnalysis, &completed_stages)
        );

        // CrossValidation should NOT be executable without SemanticAnalysis
        assert!(
            !orchestrator.can_execute_stage(&VerificationStage::CrossValidation, &completed_stages)
        );
    }

    #[test]
    fn test_next_executable_stages() {
        let orchestrator = PipelineOrchestrator::new();
        let completed_stages = vec![
            VerificationStage::Initialize,
            VerificationStage::ParseValidation,
        ];

        let next_stages = orchestrator.get_next_executable_stages(&completed_stages);

        // Both SemanticAnalysis and BehavioralVerification should be available
        assert!(next_stages.contains(&VerificationStage::SemanticAnalysis));
        assert!(next_stages.contains(&VerificationStage::BehavioralVerification));
        assert!(!next_stages.contains(&VerificationStage::CrossValidation));
    }

    #[test]
    fn test_execution_strategy_validation() {
        let mut orchestrator = PipelineOrchestrator::new();

        // Default hybrid strategy should be valid
        assert!(orchestrator.validate_execution_strategy().is_ok());

        // Parallel strategy should be valid (no circular dependencies in default config)
        orchestrator.execution_strategy = ExecutionStrategy::Parallel;
        assert!(orchestrator.validate_execution_strategy().is_ok());
    }

    #[test]
    fn test_session_finalization() {
        let mut orchestrator = PipelineOrchestrator::new();
        let document = create_test_document();

        let session_id = orchestrator.initialize_session(&document).unwrap();

        // Should have session resources
        let initial_resource_count = orchestrator.resource_manager.resource_pools.len();
        assert!(initial_resource_count > 0);

        // Finalize session
        let finalize_result = orchestrator.finalize_session(&session_id);
        assert!(finalize_result.is_ok());

        // Session-specific resources should be cleaned up
        let remaining_resources = orchestrator
            .resource_manager
            .resource_pools
            .keys()
            .filter(|k| k.contains(&session_id))
            .count();
        assert_eq!(remaining_resources, 0);
    }

    #[test]
    fn test_resource_optimization() {
        let orchestrator = PipelineOrchestrator::new_performance_optimized();

        // Performance optimized should have resource configurations
        assert!(!orchestrator.resource_manager.resource_pools.is_empty());
    }
}
