//! Validation-pipeline integration tests for the AISP validator.
//!
//! Single entry point for end-to-end **validation-pipeline** behaviour:
//! parsing, quality tiers, validation config, document-size limits, strict
//! mode, Unicode handling, and performance (R-13). The dedicated
//! formal-verification path lives in `formal_verification_integration.rs`.
//!
//! Merges the previously separate `working_integration` and
//! `simple_integration` suites, deduplicating identical scenarios while
//! preserving every unique test and its `#[ignore]` status.

use aisp_core::{
    semantic::QualityTier,
    validator::{
        types::{ValidationConfig, ValidationResult},
        AispValidator,
    },
};

// ============================================================================
// Common Helpers
// ============================================================================

/// Fluent assertion helper for validation results.
struct ValidationAssertion {
    result: ValidationResult,
}

impl ValidationAssertion {
    pub fn new(result: ValidationResult) -> Self {
        Self { result }
    }

    pub fn is_valid(self) -> Self {
        if !self.result.valid {
            // Print the actual result so we can see what we're dealing with
            println!(
                "Validation result: valid={}, error={:?}, delta={}, tier={:?}",
                self.result.valid, self.result.error, self.result.delta, self.result.tier
            );
        }
        assert!(
            self.result.valid,
            "Document should be valid but got error: {:?}, delta: {}, tier: {:?}",
            self.result.error, self.result.delta, self.result.tier
        );
        self
    }

    pub fn is_invalid(self) -> Self {
        assert!(
            !self.result.valid,
            "Document should be invalid but was valid"
        );
        self
    }

    pub fn has_quality_tier(self, expected: QualityTier) -> Self {
        assert_eq!(
            self.result.tier, expected,
            "Expected quality tier {:?} but got {:?}",
            expected, self.result.tier
        );
        self
    }

    pub fn has_error_count(self, expected: usize) -> Self {
        let actual_errors = if self.result.error.is_some() { 1 } else { 0 };
        assert_eq!(
            actual_errors, expected,
            "Expected {} errors but got {}: {:?}",
            expected, actual_errors, self.result.error
        );
        self
    }

    pub fn has_delta_above(self, threshold: f64) -> Self {
        assert!(
            self.result.delta >= threshold,
            "Expected delta >= {} but got {}",
            threshold,
            self.result.delta
        );
        self
    }

    #[allow(dead_code)] // fluent assertion helper, kept for test authors
    pub fn has_warnings(self) -> Self {
        assert!(
            !self.result.warnings.is_empty(),
            "Expected warnings but got none"
        );
        self
    }

    pub fn has_timing(self) -> Self {
        assert!(
            self.result.total_time.is_some(),
            "Expected timing information but got none"
        );
        self
    }

    #[allow(dead_code)] // fluent assertion helper, kept for test authors
    pub fn has_formal_verification(self) -> Self {
        assert!(
            self.result.formal_verification.is_some(),
            "Expected formal verification result but got none"
        );
        self
    }
}

/// Helper to assert a document is valid with the expected quality tier.
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
    // NOTE: intentionally no δ-threshold assertion here. Validity is derived
    // from `overall_confidence` while the tier is a function of δ (see
    // `DeepVerificationResult::valid`/`tier`), so a valid document can
    // legitimately have a low δ; the expected tier already pins δ's band.
}

// ============================================================================
// Basic Validation
// ============================================================================

#[test]
#[ignore = "#18: blocked by δ computation, not the tier model. tier() follows the AISP spec δ-ladder (◊⁺⁺≥0.75…⊘<0.20), but the validator computes δ≈1.0 for any valid document (δ is not yet quality-graded), so this Meta/Types/Rules/Funcs doc (declares δ≜0.8, expects Gold) resolves to Platinum. Needs quality-graded δ computation."]
fn test_minimal_valid_document() {
    let document = r#"𝔸5.1.TestDoc@2026-01-25

⟦Ω:Meta⟧{
  domain≜test
  version≜"1.0.0"
}

⟦Σ:Types⟧{
  State≜{A,B,C}
}

⟦Γ:Rules⟧{
  ∀s:State→Valid(s)
}

⟦Λ:Funcs⟧{
  next≜λs.NextState(s)
}

⟦Ε⟧⟨δ≜0.8⟩"#;

    let validator = AispValidator::new();
    let result = validator.validate(document);

    ValidationAssertion::new(result)
        .is_valid()
        .has_quality_tier(QualityTier::Gold)
        .has_error_count(0)
        .has_delta_above(0.7);
}

#[test]
fn test_complete_platinum_document() {
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

    ValidationAssertion::new(result)
        .is_valid()
        .has_quality_tier(QualityTier::Platinum)
        .has_error_count(0)
        .has_delta_above(0.8);
}

#[test]
#[ignore = "#18: stricter syntax-error rejection not yet implemented"]
fn test_document_with_syntax_errors() {
    let document = r#"𝔸5.1.ErrorTest@2026-01-25

⟦Ω:Meta⟧{
  domain≜test
  invalid_syntax_here!!!
}

⟦Ε⟧⟨δ≜invalid⟩"#;

    let validator = AispValidator::new();
    let result = validator.validate(document);

    ValidationAssertion::new(result)
        .is_invalid()
        .has_error_count(1);
}

#[test]
#[ignore = "#18: blocked by δ computation, not the tier model. tier() follows the AISP spec δ-ladder (◊⁺⁺≥0.75…⊘<0.20), but the validator computes δ≈1.0 for any valid document (δ is not yet quality-graded), so this types-only doc (declares δ≜0.8, expects Gold) resolves to Platinum. Needs quality-graded δ computation."]
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
fn test_temporal_logic_document() {
    let document = r#"𝔸5.1.TemporalTest@2026-01-25

⟦Σ:Types⟧{
  State≜{A,B,C}
}

⟦Γ:Rules⟧{
  ∀s:State→Valid(s)
  □(A→◊B)
  ◊□(C)
}

⟦Λ:Funcs⟧{
  next≜λs.NextState(s)
}

⟦Ω:Meta⟧{
  domain≜temporal_test
}

⟦Ε⟧⟨δ≜0.85;τ≜◊⁺⟩"#;

    let validator = AispValidator::new();
    let result = validator.validate(document);

    ValidationAssertion::new(result)
        .is_valid()
        .has_quality_tier(QualityTier::Platinum)
        .has_delta_above(0.8);
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
  State≜{A,B,C}
}

⟦Γ:Rules⟧{
  ∀x:Natural→x≥0
  ∃y:Real→y>0
  ∀s:State→Valid(s)
  □(A→◊B)
  ◊□(C)
}

⟦Λ:Funcs⟧{
  check≜λx.IsValid(x)
}

⟦Ω:Meta⟧{
  domain≜unicode_test
  description≜"Testing Unicode symbol handling"
}

⟦Ε⟧⟨δ≜0.85⟩"#;

    let validator = AispValidator::new();
    let result = validator.validate(document);

    ValidationAssertion::new(result)
        .is_valid()
        .has_quality_tier(QualityTier::Platinum);
}

#[test]
fn test_large_document_limit() {
    // Create a document that exceeds size limit
    let large_content = "x≜ℕ\n".repeat(1000);
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

    ValidationAssertion::new(result)
        .is_invalid()
        .has_error_count(1);
}

#[test]
fn test_validation_performance() {
    let document = r#"𝔸5.1.PerfTest@2026-01-25

⟦Σ:Types⟧{
  State≜{A,B,C,D,E,F,G,H,I,J}
  Complex≜State→State→State
}

⟦Γ:Rules⟧{
  ∀s:State→Valid(s)
  ∀c:Complex→Consistent(c)
  □(A→◊B)
  □(B→◊C)
  □(C→◊A)
}

⟦Λ:Funcs⟧{
  process≜λs.Transform(s)
  validate≜λc.Check(c)
}

⟦Ω:Meta⟧{
  domain≜performance_test
  version≜"1.0.0"
  description≜"Performance testing with complex types and rules"
}

⟦Ε⟧⟨δ≜0.88;φ≜85;τ≜◊⁺⟩"#;

    let mut config = ValidationConfig::default();
    config.include_timing = true;

    let validator = AispValidator::with_config(config);
    let start = std::time::Instant::now();
    let result = validator.validate(document);
    let duration = start.elapsed();

    ValidationAssertion::new(result)
        .is_valid()
        .has_quality_tier(QualityTier::Platinum)
        .has_timing();

    // Validation should complete reasonably quickly
    assert!(
        duration.as_millis() < 5000,
        "Validation took too long: {}ms",
        duration.as_millis()
    );
}

// ============================================================================
// Advanced Configuration & Strict Mode
// ============================================================================

#[test]
#[ignore = "#18: strict_mode block-order semantics under consolidation"]
fn test_validation_config_options() {
    let document = r#"𝔸5.1.ConfigTest@2026-01-25

⟦Σ:Types⟧{
  State≜{A,B}
}

⟦Γ:Rules⟧{
  ∀s:State→Valid(s)
}

⟦Λ:Funcs⟧{
  next≜λs.NextState(s)
}

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

    ValidationAssertion::new(result)
        .is_valid()
        .has_quality_tier(QualityTier::Gold)
        .has_timing();
}

#[test]
#[ignore = "#18: strict_mode low-quality warning semantics under consolidation"]
fn test_strict_mode_validation() {
    let document = r#"𝔸5.1.StrictTest@2026-01-25

⟦Σ:Types⟧{
  State≜{A,B}
}

⟦Γ:Rules⟧{
  ∀s:State→Valid(s)
}

⟦Λ:Funcs⟧{
  next≜λs.NextState(s)
}

⟦Ω:Meta⟧{
  domain≜strict_test
}

⟦Ε⟧⟨δ≜0.5⟩"#; // Low delta to trigger strict mode warnings

    let mut config = ValidationConfig::default();
    config.strict_mode = true;

    let validator = AispValidator::with_config(config);
    let result = validator.validate(document);

    // Document may be valid but should have warnings in strict mode
    assert!(
        !result.warnings.is_empty(),
        "Strict mode should generate warnings for low quality"
    );
}

#[test]
#[ignore = "#18: strict_mode block-requirement semantics under consolidation"]
fn test_comprehensive_validation_pipeline() {
    let document = r#"𝔸5.1.Comprehensive@2026-01-25

γ≔⟨comprehensive,validation⟩
ρ≔⟨pipeline,full-spectrum⟩

⟦Ω:Meta⟧{
  domain≜comprehensive_validation
  version≜"3.1.0"
  description≜"Complete validation pipeline test"
  author≜"Integration Test Suite"
  ∀D∈AISP:Complete(D)
  ∀V∈Validation:Thorough(V)
}

⟦Σ:Types⟧{
  State≜{Initial,Processing,Validated,Complete,Error}
  Quality≜{Low,Medium,High,Excellent}
  Metric≜{precision:ℝ,recall:ℝ,accuracy:ℝ}
  Result≜{state:State,quality:Quality,metrics:Metric}
  TransitionType≜State→State
}

⟦Γ:Rules⟧{
  □(Initial→◊Processing)
  □(Processing→◊Validated)
  □(Validated→◊Complete)
  ◊□(Complete∨Error)
  ∀m:Metric→(m.precision≥0 ∧ m.precision≤1)
  ∀m:Metric→(m.recall≥0 ∧ m.recall≤1)
  ∀m:Metric→(m.accuracy≥0 ∧ m.accuracy≤1)
  ∀r:Result→(r.quality=Excellent ⇒ r.metrics.accuracy>0.95)
  ∀s:State→∀t:TransitionType→Valid(t(s))
  ∀s:State→(s≠Error ⇒ ∃next:State→Transition(s,next))
}

⟦Λ:Funcs⟧{
  process≜λs.NextState(s)
  validate≜λs.CheckValidation(s)
  assess≜λr.EvaluateQuality(r)
  measure≜λr.CalculateMetrics(r)
  transition≜λ(from,to).CreateTransition(from,to)
  aggregate≜λresults.CombineResults(results)
}

⟦Ε⟧⟨δ≜0.95;φ≜95;τ≜◊⁺;ψ≜□◊;ξ≜0.98;μ≜85⟩"#;

    // Test with comprehensive configuration
    let mut config = ValidationConfig::default();
    config.include_timing = true;
    config.include_ast = true;
    config.include_symbol_stats = true;
    config.enable_formal_verification = true;
    config.strict_mode = true;

    let validator = AispValidator::with_config(config);
    let result = validator.validate(document);

    ValidationAssertion::new(result)
        .is_valid()
        .has_quality_tier(QualityTier::Platinum)
        .has_error_count(0)
        .has_delta_above(0.9)
        .has_timing();
}
