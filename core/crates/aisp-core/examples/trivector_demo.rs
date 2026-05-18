//! Demonstration of tri-vector validation functionality
//!
//! This example shows how the tri-vector validation system works
//! and validates AISP documents with proper tri-vector signal definitions.

use aisp_core::{
    tri_vector_validation::{TriVectorValidationConfig, TriVectorValidator},
    validator::{AispValidator, ValidationConfig},
};

/// Example AISP document with tri-vector signal definition  
const EXAMPLE_DOCUMENT: &str = r#"𝔸5.1.TriVectorDemo@2026-01-26

γ≔⟨demo,trivector,validation⟩
ρ≔⟨signal,orthogonality⟩

⟦Ω:Meta⟧{
  domain≜trivector_demo
  description≜"Tri-vector signal validation demo"
  ∀D∈AISP:Ambig(D)<0.02
}

⟦Σ:Types⟧{
  Signal≜V_H⊕V_L⊕V_S
  V_H≜ℝ⁷⁶⁸
  V_L≜ℝ⁵¹²
  V_S≜ℝ²⁵⁶
  Vector≜ℝⁿ
}

⟦Γ:Rules⟧{
  ∀s:Signal→Valid(s)
  V_H∩V_S≡∅
  V_L∩V_S≡∅
  ∀v1∈V_H,v2∈V_S:⟨v1,v2⟩≡0
  ∀v1∈V_L,v2∈V_S:⟨v1,v2⟩≡0
}

⟦Λ:Funcs⟧{
  decompose≜λs:Signal.⟨project_H(s),project_L(s),project_S(s)⟩
  project_H≜λs.π_H(s)
  project_L≜λs.π_L(s) 
  project_S≜λs.π_S(s)
  orthogonal≜λ(v1,v2).⟨v1,v2⟩≡0
}

⟦Ε⟧⟨δ≜0.75;τ≜◊⁺⁺⟩"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("AISP Tri-Vector Validation Demo");
    println!("================================\n");

    // Create validator with tri-vector validation enabled
    let mut config = ValidationConfig::default();
    config.enable_trivector_validation = true;
    config.strict_mode = true;
    config.include_timing = true;

    let validator = AispValidator::with_config(config);

    println!("Validating AISP document with tri-vector signal...");
    let result = validator.validate(EXAMPLE_DOCUMENT);

    println!("\n=== Validation Results ===");
    println!("Valid: {}", result.valid);
    println!("Tier: {} ({})", result.tier_symbol, result.tier_name);
    println!("Delta: {:.3}", result.delta);
    println!("Ambiguity: {:.3}", result.ambiguity);

    if let Some(parse_time) = result.parse_time {
        println!("Parse time: {:?}", parse_time);
    }

    if let Some(semantic_time) = result.semantic_time {
        println!("Semantic analysis time: {:?}", semantic_time);
    }

    // Show tri-vector validation results
    if let Some(ref trivector_result) = result.trivector_validation {
        println!("\n=== Tri-Vector Validation ===");
        println!("Tri-vector valid: {}", trivector_result.valid);

        if let Some(ref signal) = trivector_result.signal {
            println!("Signal detected:");
            println!(
                "  - Semantic space (V_H): {} dimensions",
                signal.semantic.dimension
            );
            println!(
                "  - Structural space (V_L): {} dimensions",
                signal.structural.dimension
            );
            println!(
                "  - Safety space (V_S): {} dimensions",
                signal.safety.dimension
            );
        }

        println!("\nOrthogonality verification:");
        for (constraint, orthogonality) in &trivector_result.orthogonality_results {
            println!(
                "  - {}: {:?} (confidence: {:.2})",
                constraint, orthogonality.orthogonality_type, orthogonality.confidence
            );
        }

        println!("\nSafety isolation:");
        println!(
            "  - Isolated: {}",
            trivector_result.safety_isolation.isolated
        );
        println!(
            "  - Violations: {}",
            trivector_result.safety_isolation.violations.len()
        );

        if !trivector_result.proof_certificates.is_empty() {
            println!("\nProof certificates generated:");
            for cert in &trivector_result.proof_certificates {
                println!("  - {}: {}", cert.id, cert.property);
            }
        }

        if !trivector_result.errors.is_empty() {
            println!("\nTri-vector errors:");
            for error in &trivector_result.errors {
                println!("  - {}", error);
            }
        }
    } else {
        println!("\n=== Tri-Vector Validation ===");
        println!("No tri-vector validation results (may be disabled or failed)");
    }

    // Show warnings if any
    if !result.warnings.is_empty() {
        println!("\n=== Warnings ===");
        for warning in &result.warnings {
            println!("  - {}", warning.message);
        }
    }

    // Show error if validation failed
    if let Some(error) = result.error {
        println!("\n=== Error ===");
        println!("{}", error);
        return Err(error.into());
    }

    println!("\n=== Direct Tri-Vector Validator Demo ===");

    // Demonstrate direct usage of tri-vector validator
    let trivector_config = TriVectorValidationConfig {
        require_formal_proofs: true,
        orthogonality_tolerance: 1e-10,
        verify_safety_isolation: true,
        z3_timeout_ms: 30000,
        max_dimension: 2048,
    };

    let _trivector_validator = TriVectorValidator::with_config(trivector_config.clone());
    println!("Created tri-vector validator with config:");
    println!(
        "  - Require formal proofs: {}",
        trivector_config.require_formal_proofs
    );
    println!(
        "  - Orthogonality tolerance: {}",
        trivector_config.orthogonality_tolerance
    );
    println!(
        "  - Verify safety isolation: {}",
        trivector_config.verify_safety_isolation
    );
    println!("  - Z3 timeout: {}ms", trivector_config.z3_timeout_ms);
    println!("  - Max dimension: {}", trivector_config.max_dimension);

    println!("\n✅ Tri-vector validation system successfully integrated!");
    println!("\nThe formal verification system now includes:");
    println!("1. 🔬 Mathematical orthogonality verification (V_H ⊥ V_S, V_L ⊥ V_S)");
    println!("2. 🛡️  Safety isolation guarantees");
    println!("3. 📐 Vector space dimension validation (768+512+256)");
    println!("4. 📜 Formal proof certificate generation");
    println!("5. 🔍 Comprehensive tri-vector property checking");

    Ok(())
}
