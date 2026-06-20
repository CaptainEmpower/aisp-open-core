//! Tests to validate our formal verification implementation improvements
//!
//! This test suite verifies that our implementation changes actually work.

use aisp_core::validator::AispValidator;

/// Test that our enumeration parser fix works
#[test]
fn test_enumeration_parsing_fix() {
    let validator = AispValidator::new();

    // Test space-separated enumeration (previously failed)
    let doc_spaces = r#"
𝔸5.1.EnumTest@2026-01-26
⟦Σ:Types⟧{
  GameState≜{Start Playing GameOver}
  Player≜{PlayerA PlayerB}
}
    "#;

    let result1 = validator.validate(doc_spaces);
    // Should not fail with parsing error
    assert!(
        !result1
            .error
            .as_ref()
            .is_some_and(|e| e.to_string().contains("Expected ',' or '}' in enumeration")),
        "Space-separated enumerations should parse without comma errors"
    );

    // Test comma-separated enumeration (should still work)
    let doc_commas = r#"
𝔸5.1.EnumTest2@2026-01-26
⟦Σ:Types⟧{
  Status≜{Active, Inactive, Pending}
}
    "#;

    let result2 = validator.validate(doc_commas);
    // Should not fail with parsing error
    assert!(
        !result2
            .error
            .as_ref()
            .is_some_and(|e| e.to_string().contains("Expected ',' or '}' in enumeration")),
        "Comma-separated enumerations should still parse correctly"
    );
}

/// Test that ambiguity calculation works with our new implementation
#[test]
fn test_ambiguity_calculation() {
    let validator = AispValidator::new();

    // Simple document to test ambiguity measurement
    let doc = r#"
𝔸5.1.Test@2026-01-26
⟦Σ:Types⟧{
  State≜{Start End}
}
    "#;

    let result = validator.validate(doc);

    // Test that ambiguity is calculated (should be a valid number)
    assert!(
        result.ambiguity >= 0.0,
        "Ambiguity should be non-negative: {}",
        result.ambiguity
    );
    assert!(
        result.ambiguity <= 1.0,
        "Ambiguity should not exceed 1.0: {}",
        result.ambiguity
    );
    assert!(!result.ambiguity.is_nan(), "Ambiguity should not be NaN");
    assert!(
        !result.ambiguity.is_infinite(),
        "Ambiguity should not be infinite"
    );
}

/// Test that formal verification system works without crashing
#[test]
fn test_formal_verification_system() {
    let validator = AispValidator::new();

    // Test with formal verification level
    let doc = r#"
𝔸5.1.Verification@2026-01-26
⟦Γ:Rules⟧{
  ∀x:Natural→x≥0
}
    "#;

    let result = validator.validate(doc);

    // Should not crash and should provide meaningful output
    assert!(
        result.tier.value() >= 0,
        "Quality tier should have valid value"
    );
    assert!(
        result.delta >= 0.0 && result.delta <= 1.0,
        "Delta should be in valid range: {}",
        result.delta
    );
}

/// Test validation result structure
#[test]
fn test_validation_result_structure() {
    let validator = AispValidator::new();

    let doc = r#"
𝔸5.1.Structure@2026-01-26
⟦Ω:Meta⟧{
  domain≜test
}
    "#;

    let result = validator.validate(doc);

    // Verify all fields are properly initialized
    assert!(
        result.document_size > 0,
        "Document size should be calculated"
    );
    assert!(!result.mode.is_empty(), "Mode should be set");

    // Tier should have meaningful value
    let tier_value = result.tier.value();
    assert!(
        (0..=5).contains(&tier_value),
        "Quality tier should be in valid range: {}",
        tier_value
    );

    // Check quality assessment function works
    if result.valid && result.ambiguity < 0.02 {
        assert!(
            result.is_acceptable(),
            "Valid document with low ambiguity should be acceptable"
        );
    }
}

/// Test with malformed document
#[test]
fn test_malformed_document_handling() {
    let validator = AispValidator::new();

    // Completely malformed document
    let bad_doc = "This is not AISP at all!";

    let result = validator.validate(bad_doc);

    // Should handle gracefully without panicking
    assert!(!result.valid, "Malformed document should be invalid");
    assert!(
        result.error.is_some(),
        "Malformed document should have error message"
    );

    if let Some(error) = &result.error {
        assert!(
            !error.to_string().is_empty(),
            "Error message should not be empty"
        );
    }
}

/// Test quality assessment improvements
#[test]
fn test_quality_assessment() {
    let validator = AispValidator::new();

    // Well-formed AISP document
    let well_formed = r#"
𝔸5.1.WellFormed@2026-01-26
γ≔⟨formal,verified⟩
⟦Ω:Meta⟧{
  ∀D∈AISP:Ambig(D)<0.02
}
⟦Σ:Types⟧{
  State≜{Initial Processing Final}
}
⟦Γ:Rules⟧{
  ∀s:State→Valid(s)
}
    "#;

    let result = validator.validate(well_formed);

    // Should achieve reasonable quality metrics
    assert!(
        result.delta > 0.0,
        "Well-formed document should have positive delta: {}",
        result.delta
    );
    assert!(
        result.tier.value() > 0,
        "Should achieve better than reject tier: {:?}",
        result.tier
    );

    // Pure density should be calculated
    assert!(
        result.pure_density >= 0.0,
        "Pure density should be non-negative: {}",
        result.pure_density
    );
}

/// Test that improvements don't break existing functionality
#[test]
fn test_backwards_compatibility() {
    let validator = AispValidator::new();

    // Test various AISP constructs
    let comprehensive_doc = r#"
𝔸5.1.Comprehensive@2026-01-26
γ≔⟨system,test⟩
ρ≔⟨protocol,example⟩

⟦Ω:Meta⟧{
  domain≜testing
  version≜1.0.0
}

⟦Σ:Types⟧{
  Status≜{Active Inactive}
  Counter≜ℕ
}

⟦Γ:Rules⟧{
  ∀s:Status→Valid(s)
  ∀n:Counter→n≥0
}

⟦Λ:Funcs⟧{
  increment≜λn.n+1
}

⟦Ε⟧⟨δ≜0.9;φ≜95⟩
    "#;

    let result = validator.validate(comprehensive_doc);

    // Should handle all constructs without major errors
    assert!(
        result.warnings.len() < 10,
        "Should have reasonable number of warnings: {}",
        result.warnings.len()
    );

    // Quality should be decent for comprehensive document
    assert!(
        result.delta > 0.3,
        "Comprehensive document should have decent delta: {}",
        result.delta
    );

    // Should complete validation in reasonable time
    if let Some(parse_time) = result.parse_time {
        assert!(
            parse_time.as_millis() < 5000,
            "Should complete in reasonable time: {}ms",
            parse_time.as_millis()
        );
    }
}
