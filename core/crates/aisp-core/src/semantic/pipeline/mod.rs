//! Multi-Layer Verification Pipeline Module
//!
//! Implements ADR-023: Deep Verification Architecture for Semantic Security
//! Split into focused SRP modules for maintainability

pub mod compliance_auditor;
pub mod core_pipeline;
pub mod orchestrator;
pub mod performance_monitor;
pub mod security_enforcer;
pub mod types;

// Re-export main types for compatibility
pub use core_pipeline::MultiLayerVerificationPipeline;
pub use types::*;
