//! Multi-Layer Verification Pipeline Implementation
//!
//! This module implements a comprehensive verification pipeline following Single Responsibility
//! Principle with focused modules for different verification aspects.

pub mod adversarial_testing;
pub mod compliance_auditor;
pub mod core_types;
pub mod main_pipeline;
pub mod performance_monitor;
pub mod pipeline_orchestrator;
pub mod security_enforcer;

// Re-export all public items
pub use adversarial_testing::*;
pub use compliance_auditor::*;
pub use core_types::*;
pub use main_pipeline::*;
pub use performance_monitor::*;
pub use pipeline_orchestrator::*;
pub use security_enforcer::*;
