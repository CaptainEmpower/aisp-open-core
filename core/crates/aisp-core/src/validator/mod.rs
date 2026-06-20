//! AISP Validation Module
//!
//! Provides comprehensive AISP document validation through a modular
//! architecture following SRP principles.
//!
//! This module is organized into focused sub-modules:
//! - `types`: Core configuration and result types
//! - `verification_methods`: Individual verification method implementations  
//! - `structural_validator`: Document structure and required block validation
//! - `engine`: Main validation orchestration engine

// Re-export public types and main API
pub use self::engine::AispValidator;
pub use self::structural_validator::{
    StructuralValidationConfig, StructuralValidationResult, StructuralValidator,
};
pub use self::types::{ValidationConfig, ValidationResult};
pub use self::verification_methods::VerificationMethods;

// Module declarations
pub mod engine;
pub mod structural_validator;
pub mod types;
pub mod verification_methods;

// Convenience re-exports for backward compatibility
pub use engine::AispValidator as Validator;

#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_module_integration() {
        // Test that all components work together
        let mut config = ValidationConfig::default();
        config.include_timing = true;
        config.strict_mode = false;

        let validator = AispValidator::with_config(config);

        let test_document = r#"
aisp_v: 5.1
name: integration_test
date: 2026-01-27

-- Functions --
test_func ≜ λx.x * 2
        "#
        .trim();

        let result = validator.validate(test_document);

        // Should complete without panicking
        assert!(result.document_size > 0);
        assert!(!result.tier_symbol.is_empty());
    }

    #[test]
    fn test_strict_mode_integration() {
        let mut config = ValidationConfig::default();
        config.strict_mode = true;
        config.strict_formal_verification = false; // Avoid Z3 dependencies in test

        let validator = AispValidator::with_config(config);

        let test_document = r#"
aisp_v: 5.1
name: strict_test
date: 2026-01-27

-- Functions --
ambiguous_func ≜ λx.undefined_operation(x)
        "#
        .trim();

        let result = validator.validate(test_document);

        // Should handle strict mode validation
        assert!(result.document_size > 0);
    }

    #[test]
    fn test_configuration_changes() {
        let mut validator = AispValidator::new();

        // Test initial configuration
        assert!(!validator.config().strict_mode);

        // Update configuration
        let mut new_config = ValidationConfig::default();
        new_config.strict_mode = true;
        new_config.z3_timeout = Duration::from_secs(60);

        validator.configure(new_config);

        assert!(validator.config().strict_mode);
        assert_eq!(validator.config().z3_timeout, Duration::from_secs(60));
    }

    #[test]
    fn test_verification_methods_integration() {
        let config = ValidationConfig::default();
        let _methods = VerificationMethods::new(config);

        // Test that verification methods can be created and used
        // This is primarily a compilation test
        assert!(true);
    }

    #[test]
    fn test_validation_result_properties() {
        let config = ValidationConfig::default();
        let validator = AispValidator::with_config(config);

        let simple_doc = r#"
aisp_v: 5.1
name: simple
date: 2026-01-27

-- Functions --  
id ≜ λx.x
        "#
        .trim();

        let result = validator.validate(simple_doc);

        // Test result properties
        assert!(!result.tier_symbol.is_empty());
        assert!(!result.tier_name.is_empty());
        assert!(result.tier_value <= 4);
        assert!(result.delta >= 0.0 && result.delta <= 1.0);
        assert!(result.ambiguity >= 0.0 && result.ambiguity <= 1.0);
    }
}
