//! Performance Verification Module
//!
//! Comprehensive performance constraint verification system split into
//! focused, SRP-compliant modules under 300 LOC each.

pub mod degradation;
pub mod qos;
pub mod resources;
pub mod sla;
pub mod throughput;
pub mod timing;
pub mod types;
pub mod verifier;

#[cfg(test)]
mod integration_test;

// Re-export key types for convenience
pub use degradation::PerformanceDegradationAnalysis;
pub use qos::QoSAnalysis;
pub use resources::ResourceBoundAnalysis;
pub use sla::SLACompliance;
pub use throughput::ThroughputAnalysis;
pub use timing::TimingConstraintAnalysis;
pub use types::*;
pub use verifier::PerformanceConstraintVerifier;
