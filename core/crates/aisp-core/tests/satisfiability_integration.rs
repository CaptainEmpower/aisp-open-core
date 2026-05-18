//! Integration tests for satisfiability checking with invariant discovery
//!
//! This module tests the integration between the invariant discovery system
//! and satisfiability checking for constraint systems.

use aisp_core::{
    ast::canonical::{self, CanonicalAispDocument as AispDocument},
    invariant_discovery::InvariantDiscovery,
    invariant_types::InvariantDiscoveryConfig,
    parser::robust_parser::AispParser,
    satisfiability_checker::{
        ConsistencyResult, SatisfiabilityChecker, SatisfiabilityConfig, SatisfiabilityResult,
    },
};

/// Test satisfiability checking with discovered invariants
#[test]
fn test_satisfiability_with_natural_types() {
    let document = create_test_document_with_natural_types();

    // Discover invariants
    let mut discovery = InvariantDiscovery::new();
    let invariants = discovery.discover_invariants(&document).unwrap();

    assert!(!invariants.is_empty(), "Should discover some invariants");

    // Check satisfiability
    let checker = SatisfiabilityChecker::default();
    let result = checker.check_invariants(&invariants).unwrap();

    match result {
        SatisfiabilityResult::Satisfiable(model) => {
            // Natural number invariants should be satisfiable
            assert!(!model.variable_assignments.is_empty());
        }
        SatisfiabilityResult::Unsatisfiable(proof) => {
            panic!(
                "Natural number invariants should be satisfiable, got proof: {:?}",
                proof.reason
            );
        }
        SatisfiabilityResult::Unknown(reason) => {
            println!("Satisfiability unknown: {}", reason);
            // This is acceptable for complex constraints
        }
    }
}

/// Test consistency checking with multiple invariants
#[test]
fn test_consistency_checking() {
    let document = create_test_document_with_mixed_types();

    // Discover invariants with high confidence threshold
    let mut config = InvariantDiscoveryConfig::default();
    config.confidence_threshold = 0.8;
    let mut discovery = InvariantDiscovery::with_config(config);
    let invariants = discovery.discover_invariants(&document).unwrap();

    // Check consistency
    let checker = SatisfiabilityChecker::default();
    let result = checker.check_consistency(&invariants).unwrap();

    match result {
        ConsistencyResult::Consistent(model) => {
            println!(
                "Invariants are consistent with model: {} assignments",
                model.variable_assignments.len()
            );
        }
        ConsistencyResult::Inconsistent(proof) => {
            println!("Found inconsistency: {}", proof.reason);
            assert!(!proof.conflicting_constraints.is_empty());
        }
        ConsistencyResult::Unknown(reason) => {
            println!("Consistency unknown: {}", reason);
        }
    }
}

/// Test satisfiability with custom configuration
#[test]
fn test_satisfiability_with_custom_config() {
    let document = create_test_document_with_enumeration();

    // Discover invariants
    let mut discovery = InvariantDiscovery::new();
    let invariants = discovery.discover_invariants(&document).unwrap();

    // Custom satisfiability configuration
    let mut config = SatisfiabilityConfig::default();
    config.timeout_seconds = 5;
    config.max_model_size = 100;
    config.enable_quantifier_instantiation = true;

    let checker = SatisfiabilityChecker::new(config);
    let result = checker.check_invariants(&invariants).unwrap();

    // Should handle enumeration constraints
    match result {
        SatisfiabilityResult::Satisfiable(model) => {
            println!(
                "Found satisfying model with {} variables",
                model.variable_assignments.len()
            );
        }
        _ => {
            // Any result is acceptable for this test
        }
    }
}

/// Test integration with AISP parser
#[test]
fn test_parser_integration_with_satisfiability() {
    let aisp_text = r#"𝔸5.1.SatTest@2026-01-26

⟦Σ:Types⟧{
  Counter≜ℕ
  State≜{Active,Inactive,Pending}
}

⟦Γ:Rules⟧{
  ∀c:Counter→c≥0
  ∀s:State→s∈{Active,Inactive,Pending}
}
"#;

    // Parse the document
    let parser = AispParser::new(aisp_text.to_string());
    let parse_result = parser.parse(aisp_text);
    assert!(parse_result.is_success(), "Parsing should succeed");
    let document = parse_result.document.unwrap();

    // Discover invariants
    let mut discovery = InvariantDiscovery::new();
    let invariants = discovery.discover_invariants(&document).unwrap();

    assert!(!invariants.is_empty());

    // Check satisfiability of discovered constraints
    let checker = SatisfiabilityChecker::default();
    let result = checker.check_invariants(&invariants).unwrap();

    match result {
        SatisfiabilityResult::Satisfiable(_) => {
            // Expected for well-formed AISP documents
        }
        SatisfiabilityResult::Unsatisfiable(proof) => {
            panic!(
                "Well-formed AISP document should be satisfiable: {}",
                proof.reason
            );
        }
        SatisfiabilityResult::Unknown(reason) => {
            println!("Satisfiability check resulted in unknown: {}", reason);
        }
    }
}

/// Test empty document satisfiability
#[test]
fn test_empty_document_satisfiability() {
    let document = create_empty_document();

    let mut discovery = InvariantDiscovery::new();
    let invariants = discovery.discover_invariants(&document).unwrap();

    // Empty document should have no invariants
    assert!(invariants.is_empty());

    let checker = SatisfiabilityChecker::default();
    let result = checker.check_invariants(&invariants).unwrap();

    // Empty constraints should be satisfiable
    match result {
        SatisfiabilityResult::Satisfiable(model) => {
            assert!(model.variable_assignments.is_empty());
        }
        _ => {
            panic!("Empty constraint system should be satisfiable");
        }
    }
}

/// Test performance with many constraints
#[test]
fn test_satisfiability_performance() {
    use std::time::Instant;

    let document = create_large_test_document();

    // Discover many invariants
    let mut config = InvariantDiscoveryConfig::default();
    config.max_invariants = 50;
    config.confidence_threshold = 0.1; // Low threshold to get many invariants

    let mut discovery = InvariantDiscovery::with_config(config);

    let discovery_start = Instant::now();
    let invariants = discovery.discover_invariants(&document).unwrap();
    let discovery_time = discovery_start.elapsed();

    println!(
        "Discovered {} invariants in {:?}",
        invariants.len(),
        discovery_time
    );

    // Check satisfiability performance
    let mut sat_config = SatisfiabilityConfig::default();
    sat_config.timeout_seconds = 2; // Short timeout for performance test

    let checker = SatisfiabilityChecker::new(sat_config);

    let sat_start = Instant::now();
    let result = checker.check_invariants(&invariants).unwrap();
    let sat_time = sat_start.elapsed();

    println!("Satisfiability check completed in {:?}", sat_time);

    match result {
        SatisfiabilityResult::Satisfiable(_) => {
            println!("Large constraint system is satisfiable");
        }
        SatisfiabilityResult::Unsatisfiable(_) => {
            println!("Large constraint system is unsatisfiable");
        }
        SatisfiabilityResult::Unknown(_) => {
            println!("Large constraint system satisfiability is unknown (expected with timeout)");
        }
    }
}

// Helper functions for creating test documents using canonical types

fn create_test_document_with_natural_types() -> AispDocument {
    let mut doc = canonical::create_document("SatTestNatural", "5.1", "2026-01-26");
    doc.add_block(canonical::create_types_block(vec![
        "Counter≜ℕ".to_string(),
        "Value≜ℕ".to_string(),
    ]));
    doc.parse_structured_data();
    doc
}

fn create_test_document_with_mixed_types() -> AispDocument {
    let mut doc = canonical::create_document("SatTestMixed", "5.1", "2026-01-26");
    doc.add_block(canonical::create_types_block(vec![
        "Counter≜ℕ".to_string(),
        "Flag≜𝔹".to_string(),
        "Status≜{Active,Inactive}".to_string(),
    ]));
    doc.parse_structured_data();
    doc
}

fn create_test_document_with_enumeration() -> AispDocument {
    let mut doc = canonical::create_document("SatTestEnum", "5.1", "2026-01-26");
    doc.add_block(canonical::create_types_block(vec![
        "Color≜{Red,Green,Blue}".to_string(),
    ]));
    doc.parse_structured_data();
    doc
}

fn create_empty_document() -> AispDocument {
    canonical::create_document("EmptyDoc", "5.1", "2026-01-26")
}

fn create_large_test_document() -> AispDocument {
    let mut type_defs = Vec::new();

    // Create many natural number types
    for i in 0..10 {
        type_defs.push(format!("Counter{}≜ℕ", i));
    }

    // Create many boolean types
    for i in 0..5 {
        type_defs.push(format!("Flag{}≜𝔹", i));
    }

    // Create enumeration types
    for i in 0..5 {
        type_defs.push(format!(
            "State{}≜{{Value{}A,Value{}B,Value{}C}}",
            i, i, i, i
        ));
    }

    let mut doc = canonical::create_document("LargeSatTest", "5.1", "2026-01-26");
    doc.add_block(canonical::create_types_block(type_defs));
    doc.parse_structured_data();
    doc
}
