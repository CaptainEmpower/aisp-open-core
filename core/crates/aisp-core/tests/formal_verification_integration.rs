//! Formal-verification integration tests for the AISP validator.
//!
//! Single entry point for the **formal-verification** path: exercises the
//! validator with `enable_formal_verification` toggled on/off across temporal,
//! mathematical, and concurrent documents (R-13). General validation-pipeline
//! behaviour (tiers, config, strict mode, performance) lives in
//! `validation_pipeline_integration.rs`.

use aisp_core::{
    semantic::QualityTier,
    validator::{
        types::{ValidationConfig, ValidationResult},
        AispValidator,
    },
};

/// Validate a document with formal verification toggled on/off (timing on).
fn test_formal_verification(document: &str, enable_formal: bool) -> ValidationResult {
    let mut config = ValidationConfig::default();
    config.enable_formal_verification = enable_formal;
    config.include_timing = true;

    let validator = AispValidator::with_config(config);
    validator.validate(document)
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

⟦Λ:Funcs⟧{
  double≜λx.2*x
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

    assert!(result.valid, "Document should be valid: {:?}", result.error);
    assert_eq!(result.tier, QualityTier::Platinum);
    assert!(
        result.formal_verification.is_some(),
        "formal_verification should be populated when enable_formal_verification = true"
    );
}

#[test]
fn test_basic_formal_verification() {
    let document = r#"𝔸5.1.BasicFormal@2026-01-25

⟦Σ:Types⟧{
  Number≜ℕ
  Positive≜{x:Number | x>0}
}

⟦Γ:Rules⟧{
  ∀x:Number→x≥0
  ∀p:Positive→p>0
  ∃x:Number→x=0
}

⟦Λ:Funcs⟧{
  double≜λx:Number.2*x
  isPositive≜λx:Number.x>0
}

⟦Ω:Meta⟧{
  domain≜basic_formal
  version≜"1.0.0"
  ∀f:Functions→Deterministic(f)
}

⟦Ε⟧⟨δ≜0.9;φ≜100⟩"#;

    let result = test_formal_verification(document, true);

    assert!(result.valid, "Document should be valid: {:?}", result.error);
    assert_eq!(result.tier, QualityTier::Platinum);
    assert!(result.delta >= 0.85);

    // Note: formal verification results are in result.formal_verification
    // The exact structure depends on the implementation
}

#[test]
fn test_temporal_formal_verification() {
    let document = r#"𝔸5.1.TemporalFormal@2026-01-25

⟦Σ:Types⟧{
  State≜{S0,S1,S2}
  Transition≜State→State
}

⟦Γ:Rules⟧{
  □(S0→◊S1)
  □(S1→◊S2)
  ◊□(S2)
  □◊(S0)
}

⟦Λ:Funcs⟧{
  next≜λs:State.NextState(s)
  reachable≜λs:State.CanReach(s)
}

⟦Ω:Meta⟧{
  domain≜temporal_formal
  version≜"1.0.0"
  ∀s:State→Reachable(s)
  ∀t:Transition→Deterministic(t)
}

⟦Ε⟧⟨δ≜0.9;φ≜120;τ≜◊⁺⟩"#;

    let result = test_formal_verification(document, true);

    assert!(
        result.valid,
        "Temporal document should be valid: {:?}",
        result.error
    );
    assert_eq!(result.tier, QualityTier::Platinum);
    assert!(result.delta >= 0.85);
}

#[test]
fn test_mathematical_formal_verification() {
    let document = r#"𝔸5.1.MathFormal@2026-01-25

⟦Σ:Types⟧{
  Natural≜ℕ
  Even≜{x:Natural | x%2=0}
  Odd≜{x:Natural | x%2=1}
  Prime≜{p:Natural | p>1 ∧ ∀x:Natural→(x|p ⇒ x=1 ∨ x=p)}
}

⟦Γ:Rules⟧{
  ∀x:Natural→(Even(x) ∨ Odd(x))
  ∀x:Natural→¬(Even(x) ∧ Odd(x))
  ∀x:Even→∀y:Odd→Even(x+y+1)
  ∃p:Prime→p>2 ∧ Odd(p)
}

⟦Λ:Funcs⟧{
  double≜λx:Natural.2*x
  successor≜λx:Natural.x+1
  isPrime≜λp:Natural.CheckPrimality(p)
  gcd≜λ(a:Natural,b:Natural).GreatestCommonDivisor(a,b)
}

⟦Ω:Meta⟧{
  domain≜mathematical_formal
  version≜"2.0.0"
  description≜"Mathematical property verification"
  ∀f:Functions→Mathematical(f)
  ∀p:Properties→Provable(p)
}

⟦Ε⟧⟨δ≜0.95;φ≜200⟩"#;

    let result = test_formal_verification(document, true);

    assert!(
        result.valid,
        "Mathematical document should be valid: {:?}",
        result.error
    );
    assert_eq!(result.tier, QualityTier::Platinum);
    assert!(result.delta >= 0.9);
}

#[test]
fn test_concurrent_formal_verification() {
    let document = r#"𝔸5.1.ConcurrentFormal@2026-01-25

⟦Σ:Types⟧{
  ProcessState≜{Idle,Running,Blocked,Terminated}
  Resource≜{Available,Locked}
  Lock≜{Acquired,Released}
}

⟦Γ:Rules⟧{
  □(Running→◊(Blocked∨Terminated))
  □¬(Acquired∧Available)
  □(Locked→◊Available)
  □◊(Idle→Running)
}

⟦Λ:Funcs⟧{
  acquire≜λr:Resource.Lock(r)
  release≜λr:Resource.Unlock(r)
  schedule≜λp:ProcessState.NextSchedule(p)
  terminate≜λp:ProcessState.Cleanup(p)
}

⟦Ω:Meta⟧{
  domain≜concurrent_formal
  version≜"1.0.0"
  description≜"Concurrent system verification"
  ∀p:Process→WellFormed(p)
  ∀r:Resource→Accessible(r)
  ∀synchronization:Correct(synchronization)
}

⟦Ε⟧⟨δ≜0.88;φ≜150;τ≜◊⁺⟩"#;

    let result = test_formal_verification(document, true);

    assert!(
        result.valid,
        "Concurrent document should be valid: {:?}",
        result.error
    );
    assert_eq!(result.tier, QualityTier::Platinum);
    assert!(result.delta >= 0.85);
}

#[test]
fn test_formal_verification_disabled() {
    let document = r#"𝔸5.1.NoFormal@2026-01-25

⟦Σ:Types⟧{
  State≜{A,B,C}
}

⟦Γ:Rules⟧{
  ∀s:State→Valid(s)
  □(A→◊B)
}

⟦Ω:Meta⟧{
  domain≜no_formal
  version≜"1.0.0"
}

⟦Ε⟧⟨δ≜0.8⟩"#;

    let result = test_formal_verification(document, false);

    assert!(
        result.valid,
        "Document should be valid without formal verification"
    );
    assert!(
        result.formal_verification.is_none(),
        "No formal verification should be performed"
    );
}

#[test]
fn test_formal_verification_with_timing() {
    let document = r#"𝔸5.1.TimingFormal@2026-01-25

⟦Σ:Types⟧{
  Integer≜ℤ
  Boolean≜𝔹
  Array≜Integer[10]
}

⟦Γ:Rules⟧{
  ∀x:Integer→(x>0 ⇒ x*x>0)
  ∀a:Array→∀i:ℕ→(i<10 ⇒ a[i]∈Integer)
  ∀b:Boolean→(b ∨ ¬b)
  ∀x:Integer→∀y:Integer→(x<y ⇒ x+1≤y)
}

⟦Λ:Funcs⟧{
  square≜λx:Integer.x*x
  arrayGet≜λ(a:Array,i:ℕ).a[i]
  negate≜λb:Boolean.¬b
  compare≜λ(x:Integer,y:Integer).x<y
}

⟦Ω:Meta⟧{
  domain≜timing_formal
  version≜"1.0.0"
  description≜"Timing analysis for formal verification"
  ∀formula:WellFormed(formula)
  ∀encoding:Correct(encoding)
}

⟦Ε⟧⟨δ≜0.92;φ≜180⟩"#;

    let start = std::time::Instant::now();
    let result = test_formal_verification(document, true);
    let duration = start.elapsed();

    assert!(result.valid, "Document should be valid");
    assert_eq!(result.tier, QualityTier::Platinum);

    // Formal verification should complete in reasonable time
    assert!(
        duration.as_millis() < 10000,
        "Formal verification took too long: {}ms",
        duration.as_millis()
    );

    // Should have timing information
    assert!(
        result.total_time.is_some(),
        "Should include timing information"
    );
}

#[test]
fn test_formal_verification_integration_with_main_validator() {
    let document = r#"𝔸5.1.Integration@2026-01-25

⟦Σ:Types⟧{
  ProcessState≜{Ready,Running,Complete}
}

⟦Γ:Rules⟧{
  □(Ready→◊Running)
  □(Running→◊Complete)
}

⟦Ω:Meta⟧{
  domain≜integration_test
  version≜"1.0.0"
}

⟦Ε⟧⟨δ≜0.9⟩"#;

    // Test both with and without formal verification
    let normal_result = test_formal_verification(document, false);
    let formal_result = test_formal_verification(document, true);

    // Both should be valid
    assert!(normal_result.valid, "Normal validation should succeed");
    assert!(formal_result.valid, "Formal validation should succeed");

    // Quality should be similar
    assert_eq!(normal_result.tier, formal_result.tier);

    // Formal result should have formal verification data
    assert!(
        normal_result.formal_verification.is_none(),
        "Normal should not have formal verification"
    );
    assert!(
        formal_result.formal_verification.is_some(),
        "Formal validation should populate formal_verification when enabled"
    );
}

#[test]
fn test_comprehensive_formal_validation() {
    let document = r#"𝔸5.1.ComprehensiveFormal@2026-01-25

⟦Ω:Meta⟧{
  domain≜comprehensive_formal
  version≜"3.0.0"
  description≜"Complete formal verification test"
  author≜"Formal Verification Team"
  ∀D∈AISP:Verified(D)
  ∀P∈Properties:Provable(P)
  ∀F∈Functions:Correct(F)
}

⟦Σ:Types⟧{
  State≜{Initial,Processing,Validated,Complete}
  Quality≜{Low,Medium,High,Excellent}
  Metric≜{precision:ℝ, recall:ℝ, accuracy:ℝ}
  Result≜{state:State, quality:Quality, metrics:Metric}
}

⟦Γ:Rules⟧{
  # Temporal properties
  □(Initial→◊Processing)
  □(Processing→◊Validated)
  □(Validated→◊Complete)
  ◊□(Complete)

  # Quality constraints
  ∀m:Metric→(m.precision≥0 ∧ m.precision≤1)
  ∀m:Metric→(m.recall≥0 ∧ m.recall≤1)
  ∀m:Metric→(m.accuracy≥0 ∧ m.accuracy≤1)
  ∀r:Result→(r.quality=Excellent ⇒ r.metrics.accuracy>0.95)
}

⟦Λ:Funcs⟧{
  process≜λs:State.NextState(s)
  validate≜λs:State.CheckValidation(s)
  assess≜λr:Result.EvaluateQuality(r)
  measure≜λr:Result.CalculateMetrics(r)
}

⟦Ε⟧⟨δ≜0.98;φ≜250;τ≜◊⁺;ψ≜□◊;ξ≜0.99⟩"#;

    let mut config = ValidationConfig::default();
    config.enable_formal_verification = true;
    config.include_timing = true;
    config.include_ast = true;
    config.strict_mode = true;

    let validator = AispValidator::with_config(config);
    let result = validator.validate(document);

    assert!(
        result.valid,
        "Comprehensive document should be valid: {:?}",
        result.error
    );
    assert_eq!(result.tier, QualityTier::Platinum);
    assert!(result.delta >= 0.95);
    assert!(result.total_time.is_some(), "Should include timing");

    // Should have comprehensive analysis
    if let Some(semantic_analysis) = &result.semantic_analysis {
        assert!(
            !semantic_analysis.warnings().is_empty() || result.warnings.is_empty(),
            "Should have analysis results"
        );
    }
}
