//! Simple integration tests for AISP validator
//!
//! These tests verify the complete validation pipeline works end-to-end
//! with real AISP documents and produces expected results.

use aisp_core::{
    semantic::QualityTier,
    validator::{
        types::{ValidationConfig, ValidationResult},
        AispValidator,
    },
};

/// Helper to assert validation results
fn assert_valid_document(result: &ValidationResult, expected_tier: QualityTier) {
    assert!(
        result.valid,
        "Document should be valid but got error: {:?}",
        result.error
    );
    assert_eq!(
        result.tier, expected_tier,
        "Expected quality tier {:?} but got {:?}",
        expected_tier, result.tier
    );
    assert!(
        result.delta >= 0.5,
        "Delta should be reasonable: {}",
        result.delta
    );
}

fn assert_invalid_document(result: ValidationResult) {
    assert!(!result.valid, "Document should be invalid but was valid");
    assert!(
        result.error.is_some(),
        "Invalid document should have an error"
    );
}

#[test]
fn test_minimal_valid_document() {
    let document = r#"𝔸5.1.TestDoc@2026-01-25

⟦Ω:Meta⟧{
  domain≜test
  version≜"1.0.0"
}

⟦Ε⟧⟨δ≜0.8⟩"#;

    let validator = AispValidator::new();
    let result = validator.validate(document);

    assert_valid_document(&result, QualityTier::Silver);
}

#[test]
fn test_complete_document() {
    let document = r#"𝔸5.1.GameLogic@2026-01-25

γ≔⟨game,turn-based⟩
ρ≔⟨protocol,state-transition⟩

⟦Ω:Meta⟧{
  domain≜game_logic
  version≜"1.0.0"
  description≜"Turn-based game state management"
  ∀D∈AISP:Ambig(D)<0.02
}

⟦Σ:Types⟧{
  GameState≜{Start,Playing,GameOver}
  Player≜{PlayerA,PlayerB}
  Move≜ℕ
  Score≜ℕ
}

⟦Γ:Rules⟧{
  ∀s:GameState→Valid(s)
  ∀p:Player→HasTurn(p)⇒CanMove(p)
  ∀m:Move→ValidMove(m)⇒UpdateState(m)
  □(Playing→◊GameOver)
}

⟦Λ:Funcs⟧{
  nextState≜λ(s,m).TransitionTo(s,m)
  isValidMove≜λm.ValidMove(m)
  calculateScore≜λ(p,moves).Σ(moves)
}

⟦Ε⟧⟨δ≜0.85;φ≜100;τ≜◊⁺⟩"#;

    let validator = AispValidator::new();
    let result = validator.validate(document);

    assert_valid_document(&result, QualityTier::Platinum);
}

#[test]
fn test_document_with_syntax_errors() {
    let document = r#"𝔸5.1.ErrorTest@2026-01-25

⟦Ω:Meta⟧{
  domain≜test
  invalid_syntax_here!!!
}

⟦Ε⟧⟨δ≜invalid⟩"#;

    let validator = AispValidator::new();
    let result = validator.validate(document);

    assert_invalid_document(result);
}

#[test]
fn test_document_with_types() {
    let document = r#"𝔸5.1.TypeTest@2026-01-25

⟦Σ:Types⟧{
  State≜{A,B,C}
  Transition≜State→State
  Value≜ℕ
}

⟦Ω:Meta⟧{
  domain≜type_test
}

⟦Ε⟧⟨δ≜0.8⟩"#;

    let validator = AispValidator::new();
    let result = validator.validate(document);

    assert_valid_document(&result, QualityTier::Gold);
}

#[test]
fn test_document_with_temporal_logic() {
    let document = r#"𝔸5.1.TemporalTest@2026-01-25

⟦Σ:Types⟧{
  State≜{A,B,C}
}

⟦Γ:Rules⟧{
  ∀s:State→Valid(s)
  □(A→◊B)
  ◊□(C)
}

⟦Ω:Meta⟧{
  domain≜temporal_test
}

⟦Ε⟧⟨δ≜0.85;τ≜◊⁺⟩"#;

    let validator = AispValidator::new();
    let result = validator.validate(document);

    assert_valid_document(&result, QualityTier::Platinum);
}

#[test]
fn test_formal_verification_enabled() {
    let document = r#"𝔸5.1.FormalTest@2026-01-25

⟦Σ:Types⟧{
  Number≜ℕ
}

⟦Γ:Rules⟧{
  ∀x:Number→x≥0
}

⟦Ω:Meta⟧{
  domain≜formal_test
  version≜"1.0.0"
}

⟦Ε⟧⟨δ≜0.9⟩"#;

    let mut config = ValidationConfig::default();
    config.enable_formal_verification = true;

    let validator = AispValidator::with_config(config);
    let result = validator.validate(document);

    assert_valid_document(&result, QualityTier::Platinum);
    // Note: formal verification results would be in result.formal_verification_result
}

#[test]
fn test_validation_config_options() {
    let document = r#"𝔸5.1.ConfigTest@2026-01-25

⟦Ω:Meta⟧{
  domain≜config_test
}

⟦Ε⟧⟨δ≜0.8⟩"#;

    let mut config = ValidationConfig::default();
    config.strict_mode = true;
    config.include_timing = true;
    config.include_ast = true;
    config.include_symbol_stats = true;
    config.max_document_size = 1000;

    let validator = AispValidator::with_config(config);
    let result = validator.validate(document);

    assert_valid_document(&result, QualityTier::Silver);

    // Verify timing information is included when requested
    assert!(
        result.total_time.is_some(),
        "Timing information should be present"
    );
}

#[test]
fn test_large_document_limit() {
    // Create a document that exceeds size limit
    let large_content = "x≜ℕ\n".repeat(1000); // Make it large
    let document = format!(
        r#"𝔸5.1.LargeTest@2026-01-25

⟦Σ:Types⟧{{
  {}
}}

⟦Ω:Meta⟧{{
  domain≜large_test
}}

⟦Ε⟧⟨δ≜0.8⟩"#,
        large_content
    );

    let mut config = ValidationConfig::default();
    config.max_document_size = 100; // Very small limit

    let validator = AispValidator::with_config(config);
    let result = validator.validate(&document);

    assert_invalid_document(result);
}

#[test]
fn test_validation_performance() {
    let document = r#"𝔸5.1.PerfTest@2026-01-25

⟦Σ:Types⟧{
  State≜{A,B,C,D,E,F,G,H,I,J}
  Complex≜State→State→State
  Nested≜{a:Complex, b:Complex, c:Complex}
}

⟦Γ:Rules⟧{
  ∀s:State→Valid(s)
  ∀c:Complex→Consistent(c)
  ∀n:Nested→WellFormed(n)
  □(A→◊B)
  □(B→◊C)
  □(C→◊A)
}

⟦Λ:Funcs⟧{
  process≜λs:State.Transform(s)
  validate≜λc:Complex.Check(c)
  analyze≜λn:Nested.Evaluate(n)
}

⟦Ω:Meta⟧{
  domain≜performance_test
  version≜"1.0.0"
  description≜"Performance testing with complex types and rules"
}

⟦Ε⟧⟨δ≜0.88;φ≜150;τ≜◊⁺⟩"#;

    let mut config = ValidationConfig::default();
    config.include_timing = true;

    let validator = AispValidator::with_config(config);
    let start = std::time::Instant::now();
    let result = validator.validate(document);
    let duration = start.elapsed();

    assert_valid_document(&result, QualityTier::Platinum);

    // Validation should complete reasonably quickly
    assert!(
        duration.as_millis() < 5000,
        "Validation took too long: {}ms",
        duration.as_millis()
    );

    // Timing information should be available
    if let Some(total_time) = result.total_time {
        assert!(
            total_time.as_millis() < 5000,
            "Reported timing too high: {}ms",
            total_time.as_millis()
        );
    }
}

#[test]
fn test_unicode_symbols_handling() {
    let document = r#"𝔸5.1.UnicodeTest@2026-01-25

⟦Σ:Types⟧{
  Natural≜ℕ
  Integer≜ℤ
  Real≜ℝ
  Boolean≜𝔹
  String≜𝕊
}

⟦Γ:Rules⟧{
  ∀x:ℕ→x≥0
  ∃y:ℝ→y>0
  □(P→◊Q)
  ◊□(R)
}

⟦Ω:Meta⟧{
  domain≜unicode_test
  description≜"Testing Unicode symbol handling"
}

⟦Ε⟧⟨δ≜0.85⟩"#;

    let validator = AispValidator::new();
    let result = validator.validate(document);

    assert_valid_document(&result, QualityTier::Platinum);
}
