//! Concurrent Behavior Types
//!
//! Core type definitions for concurrent behavior verification.

use crate::protocol_state_machine::{ProtocolStateMachine, StateTransition, TransitionTrigger};
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

/// Comprehensive concurrent behavior analysis result
#[derive(Debug, Clone)]
pub struct ConcurrentBehaviorAnalysis {
    /// Identified concurrent processes
    pub concurrent_processes: Vec<ConcurrentProcess>,
    /// Race condition analysis
    pub race_condition_analysis: RaceConditionAnalysis,
    /// Deadlock detection results
    pub deadlock_analysis: DeadlockAnalysis,
    /// Synchronization mechanism verification
    pub synchronization_analysis: SynchronizationAnalysis,
    /// Resource contention analysis
    pub resource_contention: ResourceContentionAnalysis,
    /// Message passing verification
    pub message_passing_analysis: MessagePassingAnalysis,
    /// Performance impact assessment
    pub performance_impact: ConcurrencyPerformanceImpact,
    /// Verification warnings
    pub warnings: Vec<String>,
}

/// Concurrent process representation
#[derive(Debug, Clone)]
pub struct ConcurrentProcess {
    /// Process identifier
    pub id: String,
    /// Process name
    pub name: String,
    /// Process state machine
    pub state_machine: ProtocolStateMachine,
    /// Shared resources accessed
    pub shared_resources: HashSet<String>,
    /// Communication channels
    pub channels: Vec<CommunicationChannel>,
    /// Synchronization primitives used
    pub synchronization_primitives: Vec<SynchronizationPrimitive>,
    /// Process priority
    pub priority: ProcessPriority,
    /// Process type
    pub process_type: ProcessType,
}

/// Communication channel between processes
#[derive(Debug, Clone)]
pub struct CommunicationChannel {
    /// Channel identifier
    pub id: String,
    /// Channel type
    pub channel_type: ChannelType,
    /// Sender process
    pub sender: String,
    /// Receiver process
    pub receiver: String,
    /// Message types transmitted
    pub message_types: Vec<String>,
    /// Buffer capacity (for buffered channels)
    pub buffer_capacity: Option<usize>,
    /// Reliability guarantees
    pub reliability: ChannelReliability,
}

/// Types of communication channels
#[derive(Debug, Clone, PartialEq)]
pub enum ChannelType {
    /// Synchronous message passing
    Synchronous,
    /// Asynchronous message passing
    Asynchronous,
    /// Shared memory
    SharedMemory,
    /// Publish-subscribe
    PublishSubscribe,
    /// Request-response
    RequestResponse,
    /// Broadcast
    Broadcast,
}

/// Channel reliability levels
#[derive(Debug, Clone, PartialEq)]
pub enum ChannelReliability {
    /// At most once delivery
    AtMostOnce,
    /// At least once delivery
    AtLeastOnce,
    /// Exactly once delivery
    ExactlyOnce,
    /// Best effort delivery
    BestEffort,
}

/// Synchronization primitive
#[derive(Debug, Clone)]
pub struct SynchronizationPrimitive {
    /// Primitive identifier
    pub id: String,
    /// Primitive type
    pub primitive_type: SynchronizationPrimitiveType,
    /// Processes that use this primitive
    pub users: HashSet<String>,
    /// Configuration parameters
    pub parameters: HashMap<String, String>,
}

/// Types of synchronization primitives
#[derive(Debug, Clone, PartialEq)]
pub enum SynchronizationPrimitiveType {
    Mutex,
    Semaphore,
    ConditionVariable,
    Barrier,
    ReadWriteLock,
    SpinLock,
    AtomicOperation,
    MessageQueue,
}

/// Process priority levels
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum ProcessPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Process types
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessType {
    Producer,
    Consumer,
    ProducerConsumer,
    Worker,
    Coordinator,
    Monitor,
    Service,
}

/// Race condition analysis results
#[derive(Debug, Clone)]
pub struct RaceConditionAnalysis {
    /// Detected race conditions
    pub race_conditions: Vec<RaceCondition>,
    /// Shared resource analysis
    pub shared_resource_analysis: HashMap<String, SharedResourceAnalysis>,
    /// Memory access patterns
    pub memory_access_patterns: Vec<MemoryAccessPattern>,
    /// Confidence level
    pub confidence_level: f64,
}

/// Individual race condition
#[derive(Debug, Clone)]
pub struct RaceCondition {
    /// Race condition identifier
    pub id: String,
    /// Affected processes
    pub processes: Vec<String>,
    /// Shared resource involved
    pub shared_resource: String,
    /// Severity level
    pub severity: RaceSeverity,
    /// Description
    pub description: String,
    /// Suggested mitigation
    pub mitigation: String,
}

/// Race condition severity levels
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum RaceSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Analysis of shared resource usage
#[derive(Debug, Clone)]
pub struct SharedResourceAnalysis {
    /// Resource identifier
    pub resource_id: String,
    /// Accessing processes
    pub accessing_processes: HashSet<String>,
    /// Access patterns
    pub access_patterns: Vec<AccessPattern>,
    /// Contention level
    pub contention_level: ContentionLevel,
    /// Protection mechanisms
    pub protection_mechanisms: Vec<String>,
}

/// Memory access pattern
#[derive(Debug, Clone)]
pub struct MemoryAccessPattern {
    /// Pattern identifier
    pub id: String,
    /// Process performing access
    pub process: String,
    /// Memory location
    pub memory_location: String,
    /// Access type
    pub access_type: AccessType,
    /// Access frequency
    pub frequency: AccessFrequency,
    /// Temporal ordering constraints
    pub ordering_constraints: Vec<OrderingConstraint>,
}

/// Types of memory access
#[derive(Debug, Clone, PartialEq)]
pub enum AccessType {
    Read,
    Write,
    ReadWrite,
    Atomic,
    Volatile,
}

/// Access pattern details
#[derive(Debug, Clone)]
pub struct AccessPattern {
    /// Pattern type
    pub pattern_type: AccessPatternType,
    /// Frequency of access
    pub frequency: AccessFrequency,
    /// Timing constraints
    pub timing: Option<TimingConstraints>,
    /// Dependencies on other accesses
    pub dependencies: Vec<String>,
}

/// Types of access patterns
#[derive(Debug, Clone, PartialEq)]
pub enum AccessPatternType {
    Sequential,
    Concurrent,
    Periodic,
    Sporadic,
    Random,
    Burst,
}

/// Access frequency levels
#[derive(Debug, Clone, PartialEq)]
pub enum AccessFrequency {
    Rare,
    Occasional,
    Frequent,
    Continuous,
    Periodic(Duration),
}

/// Resource contention levels
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum ContentionLevel {
    None,
    Low,
    Moderate,
    High,
    Critical,
}

/// Timing constraints
#[derive(Debug, Clone)]
pub struct TimingConstraints {
    /// Maximum access duration
    pub max_duration: Duration,
    /// Required response time
    pub response_time: Duration,
    /// Deadline constraints
    pub deadline: Option<Instant>,
    /// Periodicity requirements
    pub period: Option<Duration>,
}

/// Ordering constraint between operations
#[derive(Debug, Clone)]
pub struct OrderingConstraint {
    /// Constraint type
    pub constraint_type: OrderingType,
    /// First operation
    pub first_operation: String,
    /// Second operation
    pub second_operation: String,
    /// Strength of constraint
    pub strength: ConstraintStrength,
}

/// Types of ordering constraints
#[derive(Debug, Clone, PartialEq)]
pub enum OrderingType {
    HappensBefore,
    Sequential,
    Causal,
    Synchronous,
    Mutual,
}

/// Strength of ordering constraints
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintStrength {
    Weak,
    Strong,
    Strict,
}

/// Deadlock analysis results
#[derive(Debug, Clone)]
pub struct DeadlockAnalysis {
    /// Detected deadlocks
    pub deadlocks: Vec<Deadlock>,
    /// Potential deadlock scenarios
    pub potential_deadlocks: Vec<PotentialDeadlock>,
    /// Resource dependency graph
    pub dependency_graph: ResourceDependencyGraph,
    /// Analysis confidence
    pub confidence: f64,
}

/// Individual deadlock
#[derive(Debug, Clone)]
pub struct Deadlock {
    /// Deadlock identifier
    pub id: String,
    /// Involved processes
    pub processes: Vec<String>,
    /// Involved resources
    pub resources: Vec<String>,
    /// Deadlock type
    pub deadlock_type: DeadlockType,
    /// Resolution strategies
    pub resolution_strategies: Vec<String>,
}

/// Potential deadlock scenario
#[derive(Debug, Clone)]
pub struct PotentialDeadlock {
    /// Scenario identifier
    pub id: String,
    /// Probability of occurrence
    pub probability: f64,
    /// Triggering conditions
    pub triggers: Vec<String>,
    /// Prevention strategies
    pub prevention: Vec<String>,
}

/// Types of deadlocks
#[derive(Debug, Clone, PartialEq)]
pub enum DeadlockType {
    ResourceDeadlock,
    CommunicationDeadlock,
    WaitForDeadlock,
    LivelockVariant,
}

/// Resource dependency graph for deadlock detection
#[derive(Debug, Clone)]
pub struct ResourceDependencyGraph {
    /// Graph nodes (resources)
    pub nodes: HashSet<String>,
    /// Graph edges (dependencies)
    pub edges: Vec<ResourceDependency>,
    /// Cycles detected
    pub cycles: Vec<Vec<String>>,
}

/// Resource dependency edge
#[derive(Debug, Clone)]
pub struct ResourceDependency {
    /// Source resource
    pub source: String,
    /// Target resource
    pub target: String,
    /// Process holding source
    pub holding_process: String,
    /// Process waiting for target
    pub waiting_process: String,
}

/// Synchronization analysis results
#[derive(Debug, Clone)]
pub struct SynchronizationAnalysis {
    /// Primitive effectiveness analysis
    pub primitive_analysis: HashMap<String, PrimitiveAnalysis>,
    /// Synchronization patterns
    pub patterns: Vec<SynchronizationPattern>,
    /// Performance overhead
    pub performance_overhead: f64,
    /// Correctness assessment
    pub correctness: SynchronizationCorrectness,
}

/// Analysis of synchronization primitive
#[derive(Debug, Clone)]
pub struct PrimitiveAnalysis {
    /// Primitive identifier
    pub primitive_id: String,
    /// Usage effectiveness
    pub effectiveness: f64,
    /// Performance impact
    pub performance_impact: f64,
    /// Contention analysis
    pub contention: ContentionAnalysis,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Resource contention analysis
#[derive(Debug, Clone)]
pub struct ResourceContentionAnalysis {
    /// Overall contention level
    pub overall_contention: ContentionLevel,
    /// Resource-specific contention
    pub resource_contention: HashMap<String, ResourceContention>,
    /// Hotspot analysis
    pub hotspots: Vec<ContentionHotspot>,
    /// Mitigation recommendations
    pub mitigations: Vec<String>,
}

impl Default for ConcurrentBehaviorAnalysis {
    fn default() -> Self {
        Self {
            concurrent_processes: Vec::new(),
            race_condition_analysis: RaceConditionAnalysis::default(),
            deadlock_analysis: DeadlockAnalysis::default(),
            synchronization_analysis: SynchronizationAnalysis::default(),
            resource_contention: ResourceContentionAnalysis::default(),
            message_passing_analysis: MessagePassingAnalysis::default(),
            performance_impact: ConcurrencyPerformanceImpact::default(),
            warnings: Vec::new(),
        }
    }
}

// Continued type definitions for remaining structs...
#[derive(Debug, Clone)]
pub struct SynchronizationPattern {
    pub pattern_type: String,
    pub effectiveness: f64,
    pub usage_frequency: f64,
}

#[derive(Debug, Clone)]
pub struct SynchronizationCorrectness {
    pub overall_score: f64,
    pub issues: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ContentionAnalysis {
    pub level: ContentionLevel,
    pub wait_times: Vec<Duration>,
    pub throughput_impact: f64,
}

#[derive(Debug, Clone)]
pub struct ResourceContention {
    pub resource_id: String,
    pub contention_level: ContentionLevel,
    pub competing_processes: Vec<String>,
    pub average_wait_time: Duration,
}

#[derive(Debug, Clone)]
pub struct ContentionHotspot {
    pub resource: String,
    pub intensity: f64,
    pub peak_times: Vec<Instant>,
}

#[derive(Debug, Clone)]
pub struct MessagePassingAnalysis {
    pub channels: Vec<ChannelAnalysis>,
    pub message_patterns: Vec<MessagePattern>,
    pub reliability_assessment: f64,
    pub performance_metrics: MessagePerformanceMetrics,
}

#[derive(Debug, Clone)]
pub struct ChannelAnalysis {
    pub channel_id: String,
    pub throughput: f64,
    pub latency: Duration,
    pub reliability: f64,
    pub buffer_utilization: f64,
}

#[derive(Debug, Clone)]
pub struct MessagePattern {
    pub pattern_type: String,
    pub frequency: f64,
    pub efficiency: f64,
}

#[derive(Debug, Clone)]
pub struct MessagePerformanceMetrics {
    pub average_latency: Duration,
    pub throughput: f64,
    pub message_loss_rate: f64,
    pub queue_depths: HashMap<String, f64>,
}

#[derive(Debug, Clone)]
pub struct ConcurrencyPerformanceImpact {
    pub overall_performance_score: f64,
    pub concurrency_overhead: f64,
    pub scalability_factor: f64,
    pub efficiency_metrics: HashMap<String, f64>,
}

// Default implementations
impl Default for RaceConditionAnalysis {
    fn default() -> Self {
        Self {
            race_conditions: Vec::new(),
            shared_resource_analysis: HashMap::new(),
            memory_access_patterns: Vec::new(),
            confidence_level: 0.0,
        }
    }
}

impl Default for DeadlockAnalysis {
    fn default() -> Self {
        Self {
            deadlocks: Vec::new(),
            potential_deadlocks: Vec::new(),
            dependency_graph: ResourceDependencyGraph {
                nodes: HashSet::new(),
                edges: Vec::new(),
                cycles: Vec::new(),
            },
            confidence: 0.0,
        }
    }
}

impl Default for SynchronizationAnalysis {
    fn default() -> Self {
        Self {
            primitive_analysis: HashMap::new(),
            patterns: Vec::new(),
            performance_overhead: 0.0,
            correctness: SynchronizationCorrectness {
                overall_score: 0.0,
                issues: Vec::new(),
            },
        }
    }
}

impl Default for ResourceContentionAnalysis {
    fn default() -> Self {
        Self {
            overall_contention: ContentionLevel::None,
            resource_contention: HashMap::new(),
            hotspots: Vec::new(),
            mitigations: Vec::new(),
        }
    }
}

impl Default for MessagePassingAnalysis {
    fn default() -> Self {
        Self {
            channels: Vec::new(),
            message_patterns: Vec::new(),
            reliability_assessment: 0.0,
            performance_metrics: MessagePerformanceMetrics {
                average_latency: Duration::from_secs(0),
                throughput: 0.0,
                message_loss_rate: 0.0,
                queue_depths: HashMap::new(),
            },
        }
    }
}

impl Default for ConcurrencyPerformanceImpact {
    fn default() -> Self {
        Self {
            overall_performance_score: 0.0,
            concurrency_overhead: 0.0,
            scalability_factor: 1.0,
            efficiency_metrics: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_priority_ordering() {
        assert!(ProcessPriority::Low < ProcessPriority::Critical);
        assert!(ProcessPriority::Normal < ProcessPriority::High);
        assert_eq!(
            ProcessPriority::High.cmp(&ProcessPriority::High),
            std::cmp::Ordering::Equal
        );
    }

    #[test]
    fn test_race_severity_ordering() {
        assert!(RaceSeverity::Low < RaceSeverity::Critical);
        assert!(RaceSeverity::Medium < RaceSeverity::High);
    }

    #[test]
    fn test_contention_level_ordering() {
        assert!(ContentionLevel::None < ContentionLevel::Critical);
        assert!(ContentionLevel::Low < ContentionLevel::Moderate);
    }

    #[test]
    fn test_default_implementations() {
        let analysis = ConcurrentBehaviorAnalysis::default();
        assert!(analysis.concurrent_processes.is_empty());
        assert_eq!(analysis.race_condition_analysis.confidence_level, 0.0);

        let race_analysis = RaceConditionAnalysis::default();
        assert!(race_analysis.race_conditions.is_empty());
        assert_eq!(race_analysis.confidence_level, 0.0);
    }

    #[test]
    fn test_channel_types() {
        let channel = CommunicationChannel {
            id: "test_channel".to_string(),
            channel_type: ChannelType::Asynchronous,
            sender: "producer".to_string(),
            receiver: "consumer".to_string(),
            message_types: vec!["data".to_string()],
            buffer_capacity: Some(100),
            reliability: ChannelReliability::AtLeastOnce,
        };

        assert_eq!(channel.channel_type, ChannelType::Asynchronous);
        assert_eq!(channel.reliability, ChannelReliability::AtLeastOnce);
        assert_eq!(channel.buffer_capacity, Some(100));
    }
}
