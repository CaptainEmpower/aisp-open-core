//! Z3 Property Verification Types
//!
//! This module provides canonical type definitions for Z3-based property verification.
//! All types are re-exported from the canonical_types module to ensure consistency.

// Re-export all canonical types with legacy names for compatibility
pub use crate::z3_verification::canonical_types::{
    Z3CounterexampleModel as CounterexampleModel, Z3Diagnostic as SolverDiagnostic,
    Z3DiagnosticLevel as DiagnosticLevel, Z3FormalProof as FormalProof,
    Z3PropertyCategory as PropertyCategory, Z3PropertyResult as PropertyResult,
    Z3ResourceUsage as ResourceUsage, Z3TimingBreakdown as TimingBreakdown,
    Z3UnsatCore as UnsatCore, Z3VerificationConfig as AdvancedVerificationConfig,
    Z3VerificationResult as EnhancedVerificationResult,
    Z3VerificationStatistics as EnhancedVerificationStats,
    Z3VerificationStatus as VerificationStatus, Z3VerifiedProperty as VerifiedProperty,
};

use std::collections::HashMap;
use std::time::Duration;

/// Property complexity levels for categorization and metrics
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ComplexityLevel {
    Simple,
    Moderate,
    Complex,
    VeryComplex,
    Extreme,
}

impl ComplexityLevel {
    /// Get numeric complexity score
    pub fn score(&self) -> f64 {
        match self {
            ComplexityLevel::Simple => 1.0,
            ComplexityLevel::Moderate => 2.5,
            ComplexityLevel::Complex => 5.0,
            ComplexityLevel::VeryComplex => 8.0,
            ComplexityLevel::Extreme => 12.0,
        }
    }

    /// Categorize complexity from numeric score
    pub fn from_score(score: f64) -> Self {
        if score < 1.5 {
            ComplexityLevel::Simple
        } else if score < 3.5 {
            ComplexityLevel::Moderate
        } else if score < 6.5 {
            ComplexityLevel::Complex
        } else if score < 10.0 {
            ComplexityLevel::VeryComplex
        } else {
            ComplexityLevel::Extreme
        }
    }
}

/// Cache configuration for verification optimization
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum number of cached results
    pub max_size: usize,
    /// Cache eviction policy
    pub eviction_policy: EvictionPolicy,
    /// Target cache hit ratio
    pub target_hit_ratio: f64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_size: 1000,
            eviction_policy: EvictionPolicy::LRU,
            target_hit_ratio: 0.8,
        }
    }
}

/// Cache eviction policies
#[derive(Debug, Clone, PartialEq)]
pub enum EvictionPolicy {
    /// Least Recently Used
    LRU,
    /// Least Frequently Used
    LFU,
    /// First In, First Out
    FIFO,
    /// Random eviction
    Random,
    /// Adaptive policy based on access patterns
    Adaptive,
}

/// Verification optimization configuration
#[derive(Debug, Clone)]
pub struct OptimizationConfig {
    /// Enable verification caching
    pub enable_caching: bool,
    /// Cache configuration
    pub cache_config: CacheConfig,
    /// Enable incremental verification
    pub incremental: bool,
    /// Enable proof sharing between queries
    pub proof_sharing: bool,
    /// Enable query simplification
    pub query_simplification: bool,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            enable_caching: true,
            cache_config: CacheConfig::default(),
            incremental: true,
            proof_sharing: false, // Conservative default
            query_simplification: true,
        }
    }
}

/// Load balancing strategies for parallel verification
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub enum LoadBalancingStrategy {
    /// Round-robin assignment
    RoundRobin,
    /// Assign based on current load
    #[default]
    LoadBased,
    /// Assign based on property complexity
    ComplexityBased,
    /// Work-stealing approach
    WorkStealing,
}


/// Property verification context
#[derive(Debug, Clone)]
pub struct PropertyVerificationContext {
    /// Session identifier for tracking
    pub session_id: String,
    /// Active verification tasks
    pub active_verifications: HashMap<String, VerificationTask>,
    /// Shared verification state
    pub shared_state: SharedVerificationState,
    /// Context statistics
    pub context_stats: ContextStatistics,
}

/// Individual verification task
#[derive(Debug, Clone)]
pub struct VerificationTask {
    /// Task identifier
    pub task_id: String,
    /// Property being verified
    pub property: String,
    /// Task status
    pub status: TaskStatus,
    /// Start time
    pub start_time: std::time::SystemTime,
    /// Priority level
    pub priority: TaskPriority,
}

/// Task execution status
#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
    Cancelled,
}

/// Task priority levels
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Shared state across verification sessions
#[derive(Debug, Clone)]
pub struct SharedVerificationState {
    /// Shared lemma database
    pub lemma_database: LemmaDatabase,
    /// Shared counterexample database
    pub counterexample_database: CounterexampleDatabase,
    /// Global performance metrics
    pub performance_metrics: PerformanceMetrics,
}

/// Database of reusable lemmas
#[derive(Debug, Clone)]
pub struct LemmaDatabase {
    /// Lemma storage
    pub lemmas: HashMap<String, String>,
    /// Usage statistics
    pub usage_stats: HashMap<String, usize>,
    /// Effectiveness scores
    pub effectiveness_scores: HashMap<String, f64>,
}

/// Database of counterexamples for learning
#[derive(Debug, Clone)]
pub struct CounterexampleDatabase {
    /// Counterexample storage
    pub counterexamples: HashMap<String, String>,
    /// Pattern analysis results
    pub patterns: Vec<CounterexamplePattern>,
}

/// Counterexample pattern for analysis
#[derive(Debug, Clone)]
pub struct CounterexamplePattern {
    /// Pattern identifier
    pub id: String,
    /// Pattern description
    pub description: String,
    /// Frequency of occurrence
    pub frequency: usize,
    /// Related property categories
    pub categories: Vec<PropertyCategory>,
}

/// Performance metrics for optimization
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    /// Throughput metrics
    pub throughput: ThroughputMetrics,
    /// Latency metrics
    pub latency: LatencyMetrics,
    /// Resource efficiency metrics
    pub efficiency: EfficiencyMetrics,
}

/// Throughput measurement
#[derive(Debug, Clone)]
pub struct ThroughputMetrics {
    /// Properties verified per second
    pub properties_per_second: f64,
    /// Queries executed per second
    pub queries_per_second: f64,
    /// Peak throughput achieved
    pub peak_throughput: f64,
    /// Average throughput over session
    pub average_throughput: f64,
}

/// Latency measurement
#[derive(Debug, Clone)]
pub struct LatencyMetrics {
    /// Average verification latency
    pub average_latency: Duration,
    /// 95th percentile latency
    pub p95_latency: Duration,
    /// 99th percentile latency
    pub p99_latency: Duration,
    /// Maximum latency observed
    pub max_latency: Duration,
}

/// Resource efficiency metrics
#[derive(Debug, Clone)]
pub struct EfficiencyMetrics {
    /// CPU utilization percentage
    pub cpu_utilization: f64,
    /// Memory efficiency score
    pub memory_efficiency: f64,
    /// Cache hit ratio
    pub cache_hit_ratio: f64,
    /// Solver efficiency score
    pub solver_efficiency: f64,
}

/// Context-level statistics
#[derive(Debug, Clone)]
pub struct ContextStatistics {
    /// Total properties processed
    pub total_properties: usize,
    /// Overall success rate
    pub success_rate: f64,
    /// Average verification time
    pub avg_verification_time: Duration,
    /// Resource usage distribution
    pub resource_distribution: HashMap<String, f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complexity_level_scoring() {
        assert_eq!(ComplexityLevel::Simple.score(), 1.0);
        assert_eq!(ComplexityLevel::Extreme.score(), 12.0);

        assert_eq!(ComplexityLevel::from_score(0.5), ComplexityLevel::Simple);
        assert_eq!(ComplexityLevel::from_score(15.0), ComplexityLevel::Extreme);
    }

    #[test]
    fn test_complexity_ordering() {
        assert!(ComplexityLevel::Simple < ComplexityLevel::Complex);
        assert!(ComplexityLevel::Complex < ComplexityLevel::Extreme);

        let mut levels = [ComplexityLevel::Extreme,
            ComplexityLevel::Simple,
            ComplexityLevel::Complex];
        levels.sort();

        assert_eq!(levels[0], ComplexityLevel::Simple);
        assert_eq!(levels[2], ComplexityLevel::Extreme);
    }

    #[test]
    fn test_cache_config_defaults() {
        let config = CacheConfig::default();
        assert_eq!(config.max_size, 1000);
        assert_eq!(config.eviction_policy, EvictionPolicy::LRU);
        assert_eq!(config.target_hit_ratio, 0.8);
    }

    #[test]
    fn test_optimization_config_defaults() {
        let config = OptimizationConfig::default();
        assert!(config.enable_caching);
        assert!(config.incremental);
        assert!(!config.proof_sharing); // Conservative default
        assert!(config.query_simplification);
    }

    #[test]
    fn test_task_priority_ordering() {
        assert!(TaskPriority::Low < TaskPriority::High);
        assert!(TaskPriority::High < TaskPriority::Critical);
        assert!(TaskPriority::Normal < TaskPriority::Critical);
    }

    #[test]
    fn test_verification_task() {
        let task = VerificationTask {
            task_id: "test_task".to_string(),
            property: "test_property".to_string(),
            status: TaskStatus::Pending,
            start_time: std::time::SystemTime::now(),
            priority: TaskPriority::Normal,
        };

        assert_eq!(task.task_id, "test_task");
        assert_eq!(task.status, TaskStatus::Pending);
        assert_eq!(task.priority, TaskPriority::Normal);
    }
}
