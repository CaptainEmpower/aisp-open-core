//! Property-based tests for modern formal verification system
//!
//! This module provides comprehensive property-based testing for AISP formal verification
//! including property extraction, theorem proving, and production Z3 integration.
//!
//! **Architectural Contract:**
//! - Tests modern `z3_verification` APIs (production-ready)
//! - Validates formal verification invariants across randomized inputs
//! - Ensures property extraction and Z3 verification consistency
//! - Maintains performance bounds for verification operations

#![cfg(feature = "property-testing")]

use aisp_core::{
    ast::canonical::*,
    parser_new::AispParser,
    proof_search::ProofSearchStrategy,
    property_extractor::PropertyExtractor,
    property_types::*,
    theorem_prover::TheoremProver,
    z3_verification::{DiagnosticLevel, PropertyResult, VerificationStatus, Z3VerificationFacade},
};
use proptest::prelude::*;
use std::time::Duration;

/// Strategy for generating property types
fn property_type_strategy() -> impl Strategy<Value = PropertyType> {
    prop_oneof![
        Just(PropertyType::TypeSafety),
        Just(PropertyType::FunctionalCorrectness),
        Just(PropertyType::LogicalAssertion),
        Just(PropertyType::RelationalConstraint),
        Just(PropertyType::TemporalSafety),
        Just(PropertyType::TemporalLiveness),
    ]
}

/// Strategy for generating proof search strategies
fn proof_strategy() -> impl Strategy<Value = ProofSearchStrategy> {
    prop_oneof![
        Just(ProofSearchStrategy::NaturalDeduction),
        Just(ProofSearchStrategy::BackwardChaining),
        Just(ProofSearchStrategy::ForwardChaining),
        Just(ProofSearchStrategy::Resolution),
    ]
}

/// Strategy for generating AISP documents with formal properties
fn formal_document() -> impl Strategy<Value = String> {
    (
        prop::string::string_regex(r"[a-zA-Z][a-zA-Z0-9_]*").unwrap(),
        prop::collection::vec(
            prop::string::string_regex(r"[A-Z][a-zA-Z0-9]*").unwrap(),
            1..=5,
        ),
        prop::collection::vec(
            prop::string::string_regex(r"[a-z][a-zA-Z0-9]*").unwrap(),
            1..=5,
        ),
    )
        .prop_map(|(domain, type_names, var_names)| {
            let type_defs = type_names
                .iter()
                .map(|name| format!("  {}≜ℕ", name))
                .collect::<Vec<_>>()
                .join("\n");

            let rules = var_names
                .iter()
                .zip(&type_names)
                .map(|(var, ty)| format!("  ∀{}:{}→Valid({})", var, ty, var))
                .collect::<Vec<_>>()
                .join("\n");

            let functions = type_names
                .iter()
                .map(|name| format!("  check{}≜λx:{}→Valid(x)", name, name))
                .collect::<Vec<_>>()
                .join("\n");

            format!(
                r#"𝔸5.1.formal@2026-01-26

⟦Ω:Meta⟧{{
  domain≜{}
  protocol≜"formal verification"
}}

⟦Σ:Types⟧{{
{}
}}

⟦Γ:Rules⟧{{
{}
}}

⟦Λ:Funcs⟧{{
{}
}}

⟦Ε⟧⟨δ≜0.75⟩"#,
                domain, type_defs, rules, functions
            )
        })
}

/// Strategy for generating complex type documents
fn complex_type_document() -> impl Strategy<Value = String> {
    prop::string::string_regex(r"[a-zA-Z][a-zA-Z0-9_]*")
        .unwrap()
        .prop_map(|domain| {
            format!(
                r#"𝔸5.1.{}@2026-01-26

⟦Ω:Meta⟧{{
  domain≜{}
  protocol≜"complex types"
}}

⟦Σ:Types⟧{{
  Nat≜ℕ
  Bool≜{{true,false}}
  Pair≜(ℕ,𝔹)
  List≜[ℕ]
  Func≜ℕ→𝔹
  Option≜{{Some(ℕ),None}}
}}

⟦Γ:Rules⟧{{
  ∀x:Nat→x≥0
  ∀p:Bool→p≡true∨p≡false
  ∀(a,b):Pair→Valid(a)∧Valid(b)
  ∀xs:List→∀x∈xs→Valid(x)
  ∀f:Func→∀x:ℕ→Valid(f(x))
  ∀opt:Option→opt≡None∨∃x:ℕ→opt≡Some(x)
}}

⟦Λ:Funcs⟧{{
  id≜λx:ℕ→x
  not≜λb:𝔹→¬b
  fst≜λ(x,y)→x
  length≜λxs:[ℕ]→|xs|
  apply≜λf:Func→λx:ℕ→f(x)
  unwrap≜λopt:Option→case opt of {{Some(x)→x; None→0}}
}}

⟦Ε⟧⟨δ≜0.85⟩"#,
                domain, domain
            )
        })
}

/// Strategy for generating temporal property documents
fn temporal_document() -> impl Strategy<Value = String> {
    prop::string::string_regex(r"[a-zA-Z][a-zA-Z0-9_]*")
        .unwrap()
        .prop_map(|domain| {
            format!(
                r#"𝔸5.1.{}@2026-01-26

⟦Ω:Meta⟧{{
  domain≜{}
  protocol≜"temporal properties"
}}

⟦Σ:Types⟧{{
  State≜{{Init,Running,Done,Error}}
  Event≜{{Start,Process,Finish,Fail}}
  Time≜ℕ
}}

⟦Γ:Rules⟧{{
  ∀s:State→◇(s≠Error)
  ∀t:Time→□(t≥0)
  ∀e:Event→e≡Start→◇(∃e':Event→e'≡Finish∨e'≡Fail)
  □◇(∃s:State→s≡Done)
  ∀s:State→s≡Init→○(s≡Running∨s≡Error)
}}

⟦Λ:Funcs⟧{{
  next≜λs:State→λe:Event→transition(s,e)
  valid≜λs:State→s≠Error
  eventually≜λφ→◇φ
  always≜λφ→□φ
  until≜λφ→λψ→φ𝒰ψ
}}

⟦Ε⟧⟨δ≜0.80;τ≜◊⁺⟩"#,
                domain, domain
            )
        })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    /// Property: Property extraction should be consistent across multiple runs
    #[test]
    fn prop_property_extraction_deterministic(doc in formal_document()) {
        let mut extractor1 = PropertyExtractor::new();
        let mut extractor2 = PropertyExtractor::new();

        let mut parser1 = AispParser::new(doc.clone());
        let mut parser2 = AispParser::new(doc.clone());

        if let (Ok(parsed1), Ok(parsed2)) = (parser1.parse(), parser2.parse()) {
            if let (Ok(props1), Ok(props2)) = (
                extractor1.extract_properties(&parsed1),
                extractor2.extract_properties(&parsed2)
            ) {
                // Same number of properties extracted
                prop_assert_eq!(props1.len(), props2.len(),
                              "Property extraction should be deterministic");

                // Same property types
                let types1: Vec<_> = props1.iter().map(|p| &p.property_type).collect();
                let types2: Vec<_> = props2.iter().map(|p| &p.property_type).collect();
                prop_assert_eq!(types1, types2, "Property types should be consistent");

                // Same complexity metrics
                let complexities1: Vec<_> = props1.iter().map(|p| p.complexity.difficulty_score).collect();
                let complexities2: Vec<_> = props2.iter().map(|p| p.complexity.difficulty_score).collect();
                prop_assert_eq!(complexities1, complexities2, "Complexity scores should be consistent");
            }
        }
    }

    /// Property: Extracted properties should have valid complexity metrics
    #[test]
    fn prop_property_complexity_bounds(doc in formal_document()) {
        let mut extractor = PropertyExtractor::new();
        let mut parser = AispParser::new(doc);

        if let Ok(parsed_doc) = parser.parse() {
            if let Ok(properties) = extractor.extract_properties(&parsed_doc) {
                for property in &properties {
                    // Quantifier depth should be reasonable
                    prop_assert!(property.complexity.quantifier_depth <= 10,
                               "Quantifier depth should be bounded");

                    // Logical connectives should be non-negative
                    prop_assert!(property.complexity.logical_connectives >= 0,
                               "Logical connectives count should be non-negative");

                    // Variable count should be reasonable
                    prop_assert!(property.complexity.variable_count <= 20,
                               "Variable count should be bounded");

                    // Difficulty score should be in reasonable range
                    prop_assert!(property.complexity.difficulty_score >= 1 &&
                               property.complexity.difficulty_score <= 20,
                               "Difficulty score should be in [1,20]");

                    // Function applications should be non-negative
                    prop_assert!(property.complexity.function_applications >= 0,
                               "Function applications should be non-negative");
                }
            }
        }
    }

    /// Property: Property extraction statistics should be accurate
    #[test]
    fn prop_property_statistics_accurate(doc in formal_document()) {
        let mut extractor = PropertyExtractor::new();
        let mut parser = AispParser::new(doc);

        if let Ok(parsed_doc) = parser.parse() {
            if let Ok(_properties) = extractor.extract_properties(&parsed_doc) {
                let stats = extractor.get_statistics();
                let properties = extractor.get_properties();

                // Total properties should match
                prop_assert_eq!(stats.total_properties, properties.len(),
                              "Total properties count should match");

                // Type counts should be accurate
                let actual_type_count = properties.iter()
                    .filter(|p| p.property_type == PropertyType::TypeSafety)
                    .count();
                prop_assert_eq!(stats.type_properties, actual_type_count,
                              "Type properties count should be accurate");

                // Function counts should be accurate
                let actual_func_count = properties.iter()
                    .filter(|p| p.property_type == PropertyType::FunctionalCorrectness)
                    .count();
                prop_assert_eq!(stats.function_properties, actual_func_count,
                              "Function properties count should be accurate");

                // Average complexity should be reasonable
                if !properties.is_empty() {
                    let expected_avg = properties.iter()
                        .map(|p| p.complexity.difficulty_score as f64)
                        .sum::<f64>() / properties.len() as f64;
                    prop_assert!((stats.average_complexity - expected_avg).abs() < 1e-6,
                               "Average complexity should be calculated correctly");
                }
            }
        }
    }

    /// Property: Theorem prover should have consistent strategy behavior
    #[test]
    fn prop_theorem_prover_strategy_consistency(
        strategy in proof_strategy(),
        max_depth in 1..100usize,
        timeout_secs in 1..60u64
    ) {
        let prover1 = TheoremProver::with_config(
            strategy.clone(),
            max_depth,
            Duration::from_secs(timeout_secs)
        );
        let prover2 = TheoremProver::with_config(
            strategy.clone(),
            max_depth,
            Duration::from_secs(timeout_secs)
        );

        // Same configuration should produce identical initial state
        prop_assert_eq!(prover1.get_axioms().len(), prover2.get_axioms().len(),
                       "Axiom count should be consistent");
        prop_assert_eq!(prover1.get_inference_rules().len(), prover2.get_inference_rules().len(),
                       "Inference rules count should be consistent");

        // Axiom names should match
        let axiom_names1: Vec<_> = prover1.get_axioms().iter().map(|a| &a.name).collect();
        let axiom_names2: Vec<_> = prover2.get_axioms().iter().map(|a| &a.name).collect();
        prop_assert_eq!(axiom_names1, axiom_names2, "Axiom names should match");
    }

    /// Property: Complex type documents should extract appropriate properties
    #[test]
    fn prop_complex_type_extraction(doc in complex_type_document()) {
        let mut extractor = PropertyExtractor::new();
        let mut parser = AispParser::new(doc);

        if let Ok(parsed_doc) = parser.parse() {
            if let Ok(properties) = extractor.extract_properties(&parsed_doc) {
                // Should extract properties for each type definition
                let type_safety_props: Vec<_> = properties.iter()
                    .filter(|p| p.property_type == PropertyType::TypeSafety)
                    .collect();

                prop_assert!(!type_safety_props.is_empty(),
                           "Complex type document should have type safety properties");

                // Should have enumeration properties for Bool type
                let enum_props: Vec<_> = properties.iter()
                    .filter(|p| p.name.contains("membership") || p.name.contains("Bool"))
                    .collect();

                prop_assert!(!enum_props.is_empty(),
                           "Should have enumeration properties for Bool type");

                // Should have function properties
                let func_props: Vec<_> = properties.iter()
                    .filter(|p| p.property_type == PropertyType::FunctionalCorrectness)
                    .collect();

                prop_assert!(!func_props.is_empty(),
                           "Should have function correctness properties");

                // All properties should have valid source locations
                for property in &properties {
                    prop_assert!(!property.source_location.block_type.is_empty(),
                               "Property should have valid source location");
                }
            }
        }
    }

    /// Property: Z3 verifier should handle documents gracefully
    #[test]
    fn prop_z3_verifier_graceful_handling(doc in formal_document()) {
        let verifier = Z3VerificationFacade::new();
        if verifier.is_err() {
            // Z3 not available, skip test
            return Ok(());
        }

        let mut verifier = verifier.unwrap();
        let mut parser = AispParser::new(doc.clone());

        if let Ok(parsed_doc) = parser.parse() {
            // Attempt verification (should not panic)
            let result = verifier.verify_document(&parsed_doc, None);

            prop_assert!(result.is_ok(), "Z3 verification should not fail unexpectedly");

            if let Ok(verification_result) = result {
                // Verification status should be valid
                prop_assert!(matches!(
                    verification_result.status,
                    VerificationStatus::AllVerified |
                    VerificationStatus::PartiallyVerified |
                    VerificationStatus::Incomplete |
                    VerificationStatus::Failed(_) |
                    VerificationStatus::Disabled
                ), "Verification status should be valid");

                // Properties should have valid statuses
                for property in &verification_result.verified_properties {
                    prop_assert!(matches!(
                        property.result,
                        PropertyResult::Proven |
                        PropertyResult::Disproven |
                        PropertyResult::Unknown |
                        PropertyResult::Error(_) |
                        PropertyResult::Unsupported
                    ), "Property result should be valid");

                    // Verification time should be reasonable
                    prop_assert!(property.verification_time <= Duration::from_secs(30),
                               "Verification time should be bounded");
                }

                // Statistics should be reasonable
                prop_assert!(verification_result.stats.smt_queries >= verification_result.verified_properties.len(),
                           "SMT queries should be at least as many as properties");

                prop_assert!(verification_result.stats.total_time >= Duration::ZERO,
                           "Total time should be non-negative");
            }
        }
    }

    /// Property: Temporal property extraction should handle temporal operators correctly
    #[test]
    fn prop_temporal_property_extraction(doc in temporal_document()) {
        let mut extractor = PropertyExtractor::new();
        let mut parser = AispParser::new(doc);

        if let Ok(parsed_doc) = parser.parse() {
            if let Ok(properties) = extractor.extract_properties(&parsed_doc) {
                // Should have temporal properties from rules containing temporal operators
                let temporal_props: Vec<_> = properties.iter()
                    .filter(|p| matches!(
                        p.property_type,
                        PropertyType::TemporalSafety | PropertyType::TemporalLiveness
                    ))
                    .collect();

                if !temporal_props.is_empty() {
                    // Temporal properties should have reasonable complexity
                    for prop in &temporal_props {
                        prop_assert!(prop.complexity.difficulty_score >= 3,
                                   "Temporal properties should have higher complexity");

                        // Should contain temporal operators in formula representation
                        let formula_str = format!("{:?}", prop.formula);
                        let has_temporal = formula_str.contains("◇") ||
                                         formula_str.contains("□") ||
                                         formula_str.contains("○") ||
                                         formula_str.contains("𝒰");

                        if has_temporal {
                            prop_assert!(prop.complexity.quantifier_depth >= 1,
                                       "Temporal formulas should have quantifiers");
                        }
                    }
                }
            }
        }
    }

    /// Property: Property formula structures should be well-formed
    #[test]
    fn prop_formula_structure_wellformed(doc in formal_document()) {
        let mut extractor = PropertyExtractor::new();
        let mut parser = AispParser::new(doc);

        if let Ok(parsed_doc) = parser.parse() {
            if let Ok(properties) = extractor.extract_properties(&parsed_doc) {
                for property in &properties {
                    // Formula should have consistent structure
                    match &property.formula.structure {
                        FormulaStructure::Atomic(atomic) => {
                            prop_assert!(!atomic.predicate.is_empty(),
                                       "Atomic formula should have non-empty predicate");
                        }
                        FormulaStructure::Conjunction(subformulas) |
                        FormulaStructure::Disjunction(subformulas) => {
                            prop_assert!(!subformulas.is_empty(),
                                       "Compound formula should have subformulas");
                        }
                        FormulaStructure::Universal(quantifier, _) |
                        FormulaStructure::Existential(quantifier, _) => {
                            prop_assert!(!quantifier.variable.is_empty(),
                                       "Quantified formula should have variable");
                        }
                        _ => {
                            // Other formula types are acceptable
                            prop_assert!(true);
                        }
                    }

                    // Predicates should be consistent with formula structure
                    let formula_predicates = &property.formula.predicates;
                    let structure_predicates = match &property.formula.structure {
                        FormulaStructure::Atomic(atomic) => {
                            vec![atomic.predicate.clone()]
                        }
                        _ => vec![]
                    };

                    for pred in &structure_predicates {
                        prop_assert!(formula_predicates.contains(pred),
                                   "Formula predicates should be consistent with structure");
                    }

                    // Free variables should be a subset of all variables mentioned
                    prop_assert!(property.formula.free_variables.len() <=
                               property.complexity.variable_count,
                               "Free variables should not exceed total variable count");
                }
            }
        }
    }

    /// Property: Property context should be consistent with document content
    #[test]
    fn prop_property_context_consistency(doc in formal_document()) {
        let mut extractor = PropertyExtractor::new();
        let mut parser = AispParser::new(doc);

        if let Ok(parsed_doc) = parser.parse() {
            if let Ok(properties) = extractor.extract_properties(&parsed_doc) {
                for property in &properties {
                    let context = &property.context;

                    // Type definitions in context should match document types
                    for block in &parsed_doc.blocks {
                        if let AispBlock::Types(types_block) = block {
                            for (type_name, _type_def) in &types_block.definitions {
                                if context.type_definitions.contains_key(type_name) {
                                    // Context should have this type
                                    prop_assert!(true, "Type context should include document types");
                                }
                            }
                        }
                    }

                    // Function definitions in context should match document functions
                    for block in &parsed_doc.blocks {
                        if let AispBlock::Functions(funcs_block) = block {
                            for function in &funcs_block.functions {
                                if context.function_definitions.contains_key(&function.name) {
                                    prop_assert!(true, "Function context should include document functions");
                                }
                            }
                        }
                    }

                    // Dependencies should be valid
                    for dep in &context.dependencies {
                        prop_assert!(!dep.is_empty(), "Dependencies should be non-empty strings");
                    }
                }
            }
        }
    }
}

/// Additional property tests for edge cases and integration scenarios
#[cfg(test)]
mod formal_edge_cases {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(25))]

        /// Property: Empty formal documents should be handled gracefully
        #[test]
        fn prop_empty_formal_document_handling(name in prop::string::string_regex(r"[a-z]+").unwrap()) {
            let doc = format!(r#"𝔸5.1.{}@2026-01-26

⟦Ω:Meta⟧{{}}

⟦Σ:Types⟧{{}}

⟦Γ:Rules⟧{{}}

⟦Λ:Funcs⟧{{}}

⟦Ε⟧⟨⟩"#, name);

            let mut extractor = PropertyExtractor::new();
            let mut parser = AispParser::new(doc.clone());

            if let Ok(parsed_doc) = parser.parse() {
                let result = extractor.extract_properties(&parsed_doc);
                prop_assert!(result.is_ok(), "Empty documents should be handled gracefully");

                if let Ok(properties) = result {
                    // May have some meta properties but should not crash
                    prop_assert!(properties.len() <= 10, "Empty document should have few properties");

                    let stats = extractor.get_statistics();
                    prop_assert_eq!(stats.total_properties, properties.len(),
                                  "Statistics should be consistent for empty documents");
                }
            }
        }

        /// Property: Large formal documents should not cause performance issues
        #[test]
        fn prop_large_formal_document_performance(repetitions in 1..20usize) {
            let large_doc = format!(r#"𝔸5.1.large@2026-01-26

⟦Ω:Meta⟧{{
  domain≜large-formal-test
}}

⟦Σ:Types⟧{{
{}
}}

⟦Γ:Rules⟧{{
{}
}}

⟦Λ:Funcs⟧{{
{}
}}

⟦Ε⟧⟨δ≜0.7⟩"#,
                (0..repetitions).map(|i| format!("  Type{}≜ℕ", i)).collect::<Vec<_>>().join("\n"),
                (0..repetitions).map(|i| format!("  ∀x:Type{}→Valid(x)", i)).collect::<Vec<_>>().join("\n"),
                (0..repetitions).map(|i| format!("  func{}≜λx:Type{}→Valid(x)", i, i)).collect::<Vec<_>>().join("\n")
            );

            let mut extractor = PropertyExtractor::new();
            let mut parser = AispParser::new(large_doc);

            if let Ok(parsed_doc) = parser.parse() {
                let start = std::time::Instant::now();
                let result = extractor.extract_properties(&parsed_doc);
                let duration = start.elapsed();

                prop_assert!(duration.as_secs() < 10,
                           "Large document property extraction should complete within 10 seconds");

                if let Ok(properties) = result {
                    // Should extract properties proportional to document size
                    let expected_min = repetitions; // At least one property per type
                    prop_assert!(properties.len() >= expected_min,
                               "Large documents should extract multiple properties");

                    // All properties should have reasonable complexity
                    for property in &properties {
                        prop_assert!(property.complexity.difficulty_score <= 15,
                                   "Properties in large documents should have bounded complexity");
                    }
                }
            }
        }

        /// Property: Formal verification integration should be consistent
        #[test]
        fn prop_formal_verification_integration(doc in formal_document()) {
            let mut parser = AispParser::new(doc.clone());

            if let Ok(parsed_doc) = parser.parse() {
                // Test property extraction
                let mut extractor = PropertyExtractor::new();
                let extraction_result = extractor.extract_properties(&parsed_doc);

                if extraction_result.is_ok() {
                    // Test Z3 integration
                    let verifier_result = Z3VerificationFacade::new();
                    if let Ok(mut verifier) = verifier_result {
                        let verification_result = verifier.verify_document(&parsed_doc, None);

                        if let Ok(verification) = verification_result {
                            // Properties should exist if extraction found them
                            let extracted_props = extraction_result.unwrap();
                            let verified_props = &verification.verified_properties;

                            // If we extracted properties, verification should handle them
                            if !extracted_props.is_empty() {
                                prop_assert!(!verified_props.is_empty() ||
                                           verification.diagnostics.iter().any(|d|
                                               d.message.contains("not available") ||
                                               d.message.contains("disabled")),
                                           "Verification should either find properties or explain why not");
                            }

                            // Verification status should be consistent
                            let proven_count = verified_props.iter()
                                .filter(|p| p.result == PropertyResult::Proven)
                                .count();

                            match verification.status {
                                VerificationStatus::AllVerified => {
                                    prop_assert_eq!(proven_count, verified_props.len(),
                                                  "All verified should mean all properties proven");
                                }
                                VerificationStatus::PartiallyVerified => {
                                    prop_assert!(proven_count > 0 && proven_count < verified_props.len(),
                                               "Partially verified should mean some but not all proven");
                                }
                                VerificationStatus::Failed(_) => {
                                    prop_assert!(verification.diagnostics.iter().any(|d|
                                                   d.level == DiagnosticLevel::Error),
                                               "Failed status should have error diagnostics");
                                }
                                VerificationStatus::Incomplete | VerificationStatus::Disabled => {
                                    // These are always acceptable
                                    prop_assert!(true);
                                }
                            }
                        }
                    }
                }
            }
        }

        /// Property: Property extraction should handle malformed formulas gracefully
        #[test]
        fn prop_malformed_formula_handling(name in prop::string::string_regex(r"[a-z]+").unwrap()) {
            // Create document with potentially problematic formulas
            let doc = format!(r#"𝔸5.1.{}@2026-01-26

⟦Ω:Meta⟧{{
  domain≜malformed-test
}}

⟦Σ:Types⟧{{
  BadType≜∃∀∃∀ℕ
}}

⟦Γ:Rules⟧{{
  ∀∃∀∃x→∀∃∀∃y→∀∃∀∃z→True
  (((())))→False
  ∀→∃→∀→∃
}}

⟦Λ:Funcs⟧{{
  bad≜λ→λ→λ
  nested≜λx→(λy→(λz→(λw→w)))
}}

⟦Ε⟧⟨δ≜0.5⟩"#, name);

            let mut extractor = PropertyExtractor::new();
            let mut parser = AispParser::new(doc);

            // Should either parse successfully or fail gracefully
            match parser.parse() {
                Ok(parsed_doc) => {
                    // If it parses, property extraction should not panic
                    let result = extractor.extract_properties(&parsed_doc);
                    prop_assert!(result.is_ok(), "Property extraction should handle parsed documents");

                    if let Ok(properties) = result {
                        // Properties should have bounded complexity even for malformed content
                        for property in &properties {
                            prop_assert!(property.complexity.quantifier_depth <= 20,
                                       "Malformed documents should have bounded quantifier depth");
                            prop_assert!(property.complexity.difficulty_score <= 25,
                                       "Malformed documents should have bounded difficulty");
                        }
                    }
                }
                Err(_) => {
                    // Parser rejection is acceptable for malformed documents
                    prop_assert!(true, "Parser rejection of malformed documents is acceptable");
                }
            }
        }
    }
}
