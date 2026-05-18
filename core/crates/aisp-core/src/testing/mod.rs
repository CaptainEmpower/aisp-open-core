// Testing module for AISP security hardening
// Includes adversarial testing framework and security validation tests

pub mod adversarial_framework;
pub mod security_validation_tests;

pub use adversarial_framework::{
    AdversarialTestSuite, AttackCategory, AttackResult, SecurityAssessmentReport,
    SecurityRecommendation,
};

pub use security_validation_tests::{
    ComplianceStatus, ParserSecurityTestSuite, SecurityComplianceReport, SecurityTestResults,
};
