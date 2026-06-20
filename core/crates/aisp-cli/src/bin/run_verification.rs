//! AISP Formal Verification Runner
//!
//! Demonstrates the complete formal verification workflow including
//! invariant discovery, satisfiability checking, and proof generation.

use aisp_core::{
    ast::canonical::{self, CanonicalAispDocument as AispDocument},
    formal_verification::{FormalVerifier, VerificationConfig, VerificationMethod},
    parser::AispParser,
};
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("AISP Formal Verification System");
    println!("===================================\n");

    // Test 1: Simple document verification
    println!("Test 1: Simple AISP Document Verification");
    run_simple_verification()?;

    // Test 2: Complex document with multiple types
    println!("\nTest 2: Complex Document with Multiple Types");
    run_complex_verification()?;

    // Test 3: Parser integration test
    println!("\nTest 3: Full Parser Integration Test");
    run_parser_integration_test()?;

    // Test 4: Performance analysis
    println!("\nTest 4: Performance Analysis");
    run_performance_analysis()?;

    // Test 5: Verification method comparison
    println!("\nTest 5: Verification Method Comparison");
    run_method_comparison()?;

    println!("\nAll verification tests completed successfully!");
    Ok(())
}

fn run_simple_verification() -> Result<(), Box<dyn std::error::Error>> {
    // Create a simple test document using canonical convenience functions
    let document = create_simple_test_document();

    println!("   Document: {}", document.header.name);
    println!("   Version: {}", document.header.version);

    // Create verifier with default configuration
    let mut verifier = FormalVerifier::new();

    // Run verification
    let start_time = Instant::now();
    let result = verifier.verify(&document)?;
    let verification_time = start_time.elapsed();

    // Display results
    println!("   Verification time: {:?}", verification_time);
    println!("   Status: {:?}", result.status);
    println!(
        "   Properties checked: {}",
        result.statistics.properties_checked
    );
    println!(
        "   Successful verifications: {}",
        result.statistics.successful_verifications
    );
    println!("   Proofs generated: {}", result.proofs.len());

    if !result.verified_invariants.is_empty() {
        println!(
            "   First verified invariant: {}",
            result.verified_invariants[0].invariant.name
        );
        println!(
            "   Proof method: {:?}",
            result.verified_invariants[0].verification_method
        );
        println!(
            "   Confidence: {:.2}%",
            result.verified_invariants[0].verification_confidence * 100.0
        );
    }

    Ok(())
}

fn run_complex_verification() -> Result<(), Box<dyn std::error::Error>> {
    // Create a complex test document
    let document = create_complex_test_document();

    println!("   Document: {}", document.header.name);
    println!("   Type blocks: {}", document.blocks.len());

    // Create verifier with custom configuration
    let config = VerificationConfig {
        total_timeout: Duration::from_secs(60),
        timeout_per_property: Duration::from_secs(10),
        methods: vec![
            VerificationMethod::DirectProof,
            VerificationMethod::SmtSolverVerification,
            VerificationMethod::AutomatedProof,
        ],
        parallel_verification: true,
        worker_threads: 2,
        enable_proof_generation: true,
        enable_model_generation: true,
        memory_limit: 1_000_000_000, // 1GB
        cache_config: Default::default(),
    };

    let mut verifier = FormalVerifier::with_config(config);

    // Run verification
    let result = verifier.verify(&document)?;

    // Display detailed results
    println!("   Status: {:?}", result.status);
    println!("   Statistics:");
    println!("      Total time: {:?}", result.statistics.total_time);
    println!(
        "      Properties checked: {}",
        result.statistics.properties_checked
    );
    println!(
        "      Successful verifications: {}",
        result.statistics.successful_verifications
    );
    println!(
        "      Failed verifications: {}",
        result.statistics.failed_verifications
    );
    println!(
        "      Peak memory usage: {} bytes",
        result.statistics.resource_usage.peak_memory
    );

    // Show method distribution
    println!("   Method distribution:");
    for (method, duration) in &result.statistics.time_per_method {
        println!("      {:?}: {:?}", method, duration);
    }

    // Show proof complexity analysis
    if !result.proofs.is_empty() {
        let avg_complexity = result
            .proofs
            .iter()
            .map(|p| p.complexity.steps as f64)
            .sum::<f64>()
            / result.proofs.len() as f64;
        println!("   Average proof complexity: {:.1} steps", avg_complexity);

        let simple_proofs = result
            .proofs
            .iter()
            .filter(|p| p.complexity.steps < 10)
            .count();
        let complex_proofs = result
            .proofs
            .iter()
            .filter(|p| p.complexity.steps >= 20)
            .count();
        println!(
            "   Proof complexity distribution: {} simple, {} complex",
            simple_proofs, complex_proofs
        );
    }

    Ok(())
}

fn run_parser_integration_test() -> Result<(), Box<dyn std::error::Error>> {
    let aisp_text = r#"𝔸5.1.VerificationDemo@2026-01-26

⟦Σ:Types⟧{
  Counter≜ℕ
  Status≜{Active,Inactive,Pending}
  Balance≜ℕ
  Flag≜𝔹
}

⟦Γ:Rules⟧{
  ∀c:Counter→c≥0
  ∀s:Status→s∈{Active,Inactive,Pending}
  ∀b:Balance→b≥0
}

⟦Λ:Funcs⟧{
  increment≜λx:ℕ.x+1
  reset≜λx:ℕ.0
}
"#;

    println!("   Parsing AISP document...");

    // Parse the document
    let parser = AispParser::new(aisp_text.to_string());
    let parse_result = parser.parse(aisp_text);

    if !parse_result.errors.is_empty() {
        return Err(format!("Parse errors: {:?}", parse_result.errors).into());
    }

    let document = parse_result.document.ok_or("Failed to parse document")?;

    println!("   Document parsed successfully");
    println!("   Name: {}", document.header.name);
    println!("   Date: {}", document.header.date);

    // Run full verification workflow
    println!("   Running formal verification...");

    let mut verifier = FormalVerifier::new();
    let result = verifier.verify(&document)?;

    println!("   Verification Results:");
    println!("      Status: {:?}", result.status);
    println!(
        "      Properties checked: {}",
        result.statistics.properties_checked
    );
    println!(
        "      Successful verifications: {}",
        result.statistics.successful_verifications
    );
    println!("      Generated: {} proofs", result.proofs.len());

    // Show individual proof details
    for (i, proof) in result.proofs.iter().enumerate().take(3) {
        println!(
            "   Proof {} ({}): {} steps, depth {}",
            i + 1,
            &proof.id[..8.min(proof.id.len())],
            proof.complexity.steps,
            proof.complexity.logical_depth
        );
    }

    if result.proofs.len() > 3 {
        println!("   ... and {} more proofs", result.proofs.len() - 3);
    }

    Ok(())
}

fn run_performance_analysis() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Running performance analysis...");

    // Create documents of varying complexity
    let simple_doc = create_simple_test_document();
    let complex_doc = create_complex_test_document();
    let large_doc = create_large_test_document();

    let documents = vec![
        ("Simple", simple_doc),
        ("Complex", complex_doc),
        ("Large", large_doc),
    ];

    for (name, document) in documents {
        println!("   Testing {} document:", name);

        let mut verifier = FormalVerifier::new();
        let start_time = Instant::now();

        match verifier.verify(&document) {
            Ok(result) => {
                let total_time = start_time.elapsed();
                println!("      Total time: {:?}", total_time);
                println!(
                    "      Properties: {}/{} verified",
                    result.statistics.successful_verifications,
                    result.statistics.properties_checked
                );
                println!(
                    "      Memory peak: {} KB",
                    result.statistics.resource_usage.peak_memory / 1024
                );

                if !result.proofs.is_empty() {
                    let avg_proof_time = result.statistics.performance.avg_proof_time;
                    println!("      Avg proof time: {:?}", avg_proof_time);
                }
            }
            Err(e) => {
                println!("      Failed: {}", e);
            }
        }
    }

    Ok(())
}

fn run_method_comparison() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Comparing verification methods...");

    let document = create_simple_test_document();

    let methods_to_test = vec![
        ("Direct Proof", vec![VerificationMethod::DirectProof]),
        (
            "SMT Verification",
            vec![VerificationMethod::SmtSolverVerification],
        ),
        ("Automated Proof", vec![VerificationMethod::AutomatedProof]),
        (
            "Hybrid Approach",
            vec![
                VerificationMethod::DirectProof,
                VerificationMethod::SmtSolverVerification,
                VerificationMethod::AutomatedProof,
            ],
        ),
    ];

    for (name, methods) in methods_to_test {
        println!("   Testing {}:", name);

        let config = VerificationConfig {
            methods,
            timeout_per_property: Duration::from_secs(5),
            total_timeout: Duration::from_secs(30),
            memory_limit: 500_000_000, // 500MB
            enable_proof_generation: true,
            enable_model_generation: false,
            parallel_verification: false,
            worker_threads: 1,
            cache_config: Default::default(),
        };

        let mut verifier = FormalVerifier::with_config(config);
        let start_time = Instant::now();

        match verifier.verify(&document) {
            Ok(result) => {
                let verification_time = start_time.elapsed();
                println!("      Time: {:?}", verification_time);
                println!(
                    "      Success rate: {}/{}",
                    result.statistics.successful_verifications,
                    result.statistics.properties_checked
                );

                if !result.proofs.is_empty() {
                    let avg_complexity = result
                        .proofs
                        .iter()
                        .map(|p| p.complexity.steps as f64)
                        .sum::<f64>()
                        / result.proofs.len() as f64;
                    println!("      Avg complexity: {:.1} steps", avg_complexity);
                }
            }
            Err(e) => {
                println!("      Failed: {}", e);
            }
        }
    }

    Ok(())
}

// Helper functions to create test documents using canonical module

fn create_simple_test_document() -> AispDocument {
    let mut doc = canonical::create_document("SimpleVerificationTest", "5.1", "2026-01-26");
    doc.add_block(canonical::create_types_block(vec!["Counter≜ℕ".to_string()]));
    doc.parse_structured_data();
    doc
}

fn create_complex_test_document() -> AispDocument {
    let mut doc = canonical::create_document("ComplexVerificationTest", "5.1", "2026-01-26");
    doc.add_block(canonical::create_types_block(vec![
        "Counter≜ℕ".to_string(),
        "Status≜{Active,Inactive,Pending}".to_string(),
        "Flag≜𝔹".to_string(),
    ]));
    doc.parse_structured_data();
    doc
}

fn create_large_test_document() -> AispDocument {
    let mut doc = canonical::create_document("LargeScaleVerificationTest", "5.1", "2026-01-26");

    let mut type_defs = Vec::new();
    for i in 0..10 {
        type_defs.push(format!("Counter{}≜ℕ", i));
        type_defs.push(format!("Flag{}≜𝔹", i));
        type_defs.push(format!(
            "Status{}≜{{State{}A,State{}B,State{}C}}",
            i, i, i, i
        ));
    }

    doc.add_block(canonical::create_types_block(type_defs));
    doc.parse_structured_data();
    doc
}
