# Results: 2026-11-05, revised 2026-11-06

**Verdict: the verifier fails the experiment.** It reports *verified* for every lemma,
including all 7 false mutants and the contradiction `∀x∈ℕ:x<x`. It refutes none of
them, and none comes back `Unknown` either. Its "proofs" of the true lemmas are not
evidence: the output is the same with the lemma's rule deleted, so the rule was never
examined.

Conditions: aisp-open-core `main` @ `e412f3e`, `aisp-cli` release build with
`z3-verification` (links `libz3.so.4`, Z3 4.8.12), Ubuntu 24.04, x86-64.

The 2026-11-06 revision follows the review on #32. Its changes:

- The runners now judge each verdict and share one proxy for "the rule decided it".
  Each lemma has a partner that is byte-identical outside `⟦Γ⟧` and expects the
  opposite verdict, and a verdict counts only if it differs from the partner's. The
  expected verdicts moved out of the documents into `lemmas/MANIFEST.tsv`, and
  `corpus.sh` and the driver refuse a corpus that breaks these rules. What counts as
  proved and refuted differs per entry point; see the README.
- T4/F4 (non-linear, undecidable by Z3) are replaced by a linear pair.
- C4 is added.
- Every expected verdict is checked against Z3 directly.
- The citations are corrected.
- The driver treats a parse error as no verdict. The finding behind it: no lemma's
  `⟦Γ⟧` block survives parsing.

The verdict is unchanged.

## 0. Ground truth: Z3 on each lemma's negation (`./ground-truth.sh`)

Every expected verdict is correct and decidable. This is the run that should, and does, pass.

| lemma | expected | z3 on the negation | agrees |
|---|---|---|---|
| C1-contradiction | refuted | sat | yes |
| C2-tautology | proved | unsat | yes |
| C3-uses-lambda | refuted | sat | yes |
| C4-needs-lambda | proved | unsat | yes |
| F1-charge-sound | refuted | sat | yes |
| F2-exhausted-stays | refuted | sat | yes |
| F3-fits-charged | refuted | sat | yes |
| F4-run-stays-in-budget | refuted | sat | yes |
| F5-join-commutes | refuted | sat | yes |
| F6-trust-meet | refuted | sat | yes |
| F7-deny-beats-confirm | refuted | sat | yes |
| T1-charge-sound | proved | unsat | yes |
| T2-exhausted-stays | proved | unsat | yes |
| T3-fits-charged | proved | unsat | yes |
| T4-run-stays-in-budget | proved | unsat | yes |
| T5-join-commutes | proved | unsat | yes |
| T6-trust-meet | proved | unsat | yes |
| T7-deny-beats-confirm | proved | unsat | yes |

## 1. CLI: `aisp-cli -l formal --formal-verification -f json` (`./run.sh`)

| lemma | expected | valid | formal status | proven/checked | verdict | partner verdict | agrees |
|---|---|---|---|---|---|---|---|
| C1-contradiction | refuted | true | AllVerified | 2/2 | proved | proved | **no** |
| C2-tautology | proved | true | AllVerified | 2/2 | proved | proved | **no** |
| C3-uses-lambda | refuted | true | AllVerified | 2/2 | proved | proved | **no** |
| C4-needs-lambda | proved | true | AllVerified | 2/2 | proved | proved | **no** |
| F1-charge-sound | refuted | true | AllVerified | 2/2 | proved | proved | **no** |
| F2-exhausted-stays | refuted | true | AllVerified | 2/2 | proved | proved | **no** |
| F3-fits-charged | refuted | true | AllVerified | 2/2 | proved | proved | **no** |
| F4-run-stays-in-budget | refuted | true | AllVerified | 2/2 | proved | proved | **no** |
| F5-join-commutes | refuted | true | AllVerified | 2/2 | proved | proved | **no** |
| F6-trust-meet | refuted | true | AllVerified | 2/2 | proved | proved | **no** |
| F7-deny-beats-confirm | refuted | true | AllVerified | 2/2 | proved | proved | **no** |
| T1-charge-sound | proved | true | AllVerified | 2/2 | proved | proved | **no** |
| T2-exhausted-stays | proved | true | AllVerified | 2/2 | proved | proved | **no** |
| T3-fits-charged | proved | true | AllVerified | 2/2 | proved | proved | **no** |
| T4-run-stays-in-budget | proved | true | AllVerified | 2/2 | proved | proved | **no** |
| T5-join-commutes | proved | true | AllVerified | 2/2 | proved | proved | **no** |
| T6-trust-meet | proved | true | AllVerified | 2/2 | proved | proved | **no** |
| T7-deny-beats-confirm | proved | true | AllVerified | 2/2 | proved | proved | **no** |

Every document reports `valid: true`, tier Platinum, `AllVerified 2/2`. Every lemma and
its partner therefore get the same verdict, so no row can show that the rule decided
anything. Nothing the CLI reports depends on the lemma. With the `⟦Γ⟧` block deleted,
the output is also unchanged.

## 2. Library: `FormalVerifier::verify` (`driver/`)

| lemma | expected | status | checked/ok/failed | verdict | partner verdict | agrees |
|---|---|---|---|---|---|---|
| C1-contradiction | refuted | PARSE-ERRORS(3): kept Meta,Functions,Evidence | - | none | none | **no** |
| C2-tautology | proved | PARSE-ERRORS(3): kept Meta,Functions,Evidence | - | none | none | **no** |
| C3-uses-lambda | refuted | PARSE-ERRORS(3): kept Meta,Functions,Evidence | - | none | none | **no** |
| C4-needs-lambda | proved | PARSE-ERRORS(3): kept Meta,Functions,Evidence | - | none | none | **no** |
| F1-charge-sound | refuted | PARSE-ERRORS(3): kept Meta,Functions,Evidence | - | none | none | **no** |
| F2-exhausted-stays | refuted | PARSE-ERRORS(3): kept Meta,Functions,Evidence | - | none | none | **no** |
| F3-fits-charged | refuted | PARSE-ERRORS(3): kept Meta,Functions,Evidence | - | none | none | **no** |
| F4-run-stays-in-budget | refuted | PARSE-ERRORS(3): kept Meta,Functions,Evidence | - | none | none | **no** |
| F5-join-commutes | refuted | PARSE-ERRORS(3): kept Meta,Functions,Evidence | - | none | none | **no** |
| F6-trust-meet | refuted | PARSE-ERRORS(3): kept Meta,Functions,Evidence | - | none | none | **no** |
| F7-deny-beats-confirm | refuted | PARSE-ERRORS(3): kept Meta,Functions,Evidence | - | none | none | **no** |
| T1-charge-sound | proved | PARSE-ERRORS(3): kept Meta,Functions,Evidence | - | none | none | **no** |
| T2-exhausted-stays | proved | PARSE-ERRORS(3): kept Meta,Functions,Evidence | - | none | none | **no** |
| T3-fits-charged | proved | PARSE-ERRORS(3): kept Meta,Functions,Evidence | - | none | none | **no** |
| T4-run-stays-in-budget | proved | PARSE-ERRORS(3): kept Meta,Functions,Evidence | - | none | none | **no** |
| T5-join-commutes | proved | PARSE-ERRORS(3): kept Meta,Functions,Evidence | - | none | none | **no** |
| T6-trust-meet | proved | PARSE-ERRORS(3): kept Meta,Functions,Evidence | - | none | none | **no** |
| T7-deny-beats-confirm | proved | PARSE-ERRORS(3): kept Meta,Functions,Evidence | - | none | none | **no** |

None of the lemmas parses cleanly. The parser reports three errors per document and
keeps only the `⟦Ω⟧`, `⟦Λ⟧` and `⟦Ε⟧` blocks: it drops `⟦Σ⟧` (it rejects `𝒫(ℕ)`) and
`⟦Γ⟧` (it rejects the quantified rule; see "What would make it pass"). Run on the
remnant, `FormalVerifier` reports `Verified` with **zero** properties checked. The
driver therefore treats any parse error as no verdict, rather than judging a document
that no longer contains the lemma.

## Why: read from the source, not inferred from the output

All paths are relative to `core/crates/`.

0. **The rule never reaches either verifier.** The pest grammar rejects every
   quantified rule, so the parser drops the whole `⟦Γ⟧` block and carries on (see the
   first item under "What would make it pass"). The CLI reports such a document as
   `valid: true` with `errors: []`.
1. **CLI `AllVerified` is a confidence threshold.** `aisp-cli/src/main.rs:409-415`
   sets it when the `DeepVerifier`'s `overall_confidence > 0.9`. The two "properties"
   are `TypeSystem` and `LogicConsistency`, each passed when a heuristic score exceeds
   0.8 (`aisp-core/src/semantic/deep_verifier/mod.rs:243-262`). Neither one is the
   lemma.
2. **`FormalVerifier` never reads `⟦Γ⟧`.** `aisp-core/src/invariant_analyzer.rs:45-48`
   discovers invariants from `⟦Σ⟧` type definitions only. The function-invariant pass,
   `invariant_analyzer.rs::analyze_function_invariants` (lines 104-109), is an empty
   stub. The repo's own examples confirm this: their checked properties are
   type-derived (`∀x:Counter. WellFormed(x)`). Even a true one, `∀x:Counter. x≥0`,
   fails with `ProofGenerationFailed: Proof not found` (`run_verification` Test 3), so
   that path is also wrong in the other direction.
3. **The one path that does read `⟦Γ⟧` hard-codes the answer.**
   `aisp-core/src/z3_integration.rs:531-552` (`verify_custom_assertions`) turns every
   rule into a property with `status: PropertyStatus::Proven, // Simplified` and never
   calls the solver.

The `Unknown`-returning property verifier (`aisp-core/src/z3_verification/properties/`)
is honest about what falls outside its fragment. But neither document entry point
routes `⟦Γ⟧` rules to it, so it plays no part in either result.

## What this means for gents-camel

ADR-0194 §4 left the solver vehicle open: Z3 directly, or AISP through this repository.
On this evidence, **AISP is not a viable vehicle today**. Its verifier cannot tell a
true `⟦Γ⟧` rule from a false one, so adopting it would produce exactly the green that
discharged nothing which ADR-0194's consequence 2 exists to refuse. The `ground-truth.sh`
encodings above are what using Z3 directly would look like for these lemmas.

## What would make it pass

Routing `⟦Γ⟧` to the solver is necessary, but not sufficient on its own. The rules do
not reach a solver intact today, for five reasons:

- **Quantified rules do not parse:** the pest grammar rejects every quantified rule
  tried. `∀x∈ℕ:x≤x`, multi- and nested-variable binders and `∀a,b:a∪b≡b∪a` fail
  with `expected logical_expr`. `∀x:x≤x` fails with `expected set_type_expr`, and
  `∀p:ℋ.id(p)≡SHA256(𝒩(p))` from `evidence/rosetta-stone/spec.aisp` fails with
  `expected quant_type`. Only an unquantified comparison such as `x≤y` gets through.
  The parser then drops the whole `⟦Γ⟧` block ("Failed to recover RulesBlock") and
  returns the rest of the document. The CLI shows none of these errors, and
  `FormalVerifier` verifies whatever blocks are left. The repository's own examples
  lose `⟦Γ⟧` too, for this reason or another. `spec.aisp` loses it on its quantified
  rules. `docs/examples/reference-platinum.aisp` has no quantifier in `⟦Γ⟧`, and
  loses it because the rule grammar also rejects its `name≜"string"` lines. The
  corpus's `⟦Σ⟧` block is dropped the same way, because `𝒫(ℕ)` is not a
  `type_expression`.
- **Binders:** `aisp-core/src/parser/content/logic_content.rs::parse_quantifier_binding`
  keeps everything before `∈` as one variable name, so `∀b,s,d∈ℕ:` binds a single
  variable called `"b,s,d"`. A `LogicalRule` holds a single quantifier, so nesting the
  binders instead would not help. The multi-variable form is the reference's own
  (`∀A,B:` in `docs/examples/reference.md`), so the corpus keeps it.
- **Arithmetic:** `aisp-core/src/ast/canonical/types.rs::BinaryOperator` has no
  addition, subtraction or power. `s+d` and `b-s` therefore become
  `LogicalExpression::Raw`.
- **Quantifiers and operands are dropped:**
  `aisp-core/src/formula_converter.rs::extract_quantifiers` returns `vec![]`. In
  `formula_converter.rs::convert_logical_expression_structure`, `∧`, `∨` and `⇒` keep
  their structure. `=` becomes `ArithmeticEqual(left, right)`, and every other
  operator becomes an uninterpreted atom named after the operator (`LessEqual`,
  `LessThan`, `GreaterThan`, `Equivalence` for `≡`, `Union`, …). In both cases the
  arguments are the literal variables `left` and `right`, whatever the operands were,
  and `Raw` becomes the atom `unknown`. Every occurrence of an operator is therefore
  the same proposition, and a solver would decide shapes rather than arithmetic.
  F2 `(s+d>b)⇒(t+d>b)` becomes `GreaterThan(left,right)⇒GreaterThan(left,right)`, so
  it would be **proved** although it is false. T2 becomes `(GreaterThan∧LessEqual)⇒
  GreaterThan` and would be proved, right for the wrong reason. F1
  `LessEqual⇒LessThan` would be "refuted" only because the two atom names differ.
- **A second rule path:** `aisp-core/src/ast/canonical/blocks.rs::parse_logical_rule`
  wraps every rule as `Raw`, with no quantifier.

A fix needs all of the following:

1. The grammar accepts quantified rules. A block that fails to parse makes the
   document fail verification, instead of being dropped.
2. The rule parser binds several variables per quantifier and parses arithmetic.
3. The converter keeps quantifiers, operands and `⟦Λ⟧` definitions. A rule over a
   defined symbol is checked with the definition unfolded (C4), or is `Unknown`.
4. `⟦Γ⟧` is routed to the `properties` verifier, which asserts the negation: `unsat`
   means proved, `sat` means refuted with a model, and anything out of fragment is
   `Unknown`.
5. `Verified` is never reported when zero properties were checked.

Then re-run this corpus as the acceptance test. `run.sh` and the driver should both
exit 0 and agree with `ground-truth.sh` row for row. Today they exit 1 on every row.
Tracked in #33 (CLI verdict) and #34 (`FormalVerifier`).
