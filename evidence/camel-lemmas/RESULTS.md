# Results: 2026-11-05, revised 2026-11-06

**Verdict: the verifier fails the experiment.** It reports *verified* for every lemma,
including all 7 false mutants and the contradiction `∀x∈ℕ:x<x`. It refutes none of
them, and none comes back `Unknown` either. Its "proofs" of the true lemmas are not
evidence: the output is the same with the lemma's rule deleted, so the rule was never
examined.

Conditions: aisp-open-core `main` @ `e412f3e`, `aisp-cli` release build with
`z3-verification` (links `libz3.so.4`, Z3 4.8.12), Ubuntu 24.04, x86-64.

The 2026-11-06 revision follows the review on #32. Its changes:

- The runners now judge each verdict, and count it only if the rule was examined.
- T4/F4 (non-linear, undecidable by Z3) are replaced by a linear pair.
- C4 is added.
- Every expected verdict is checked against Z3 directly.
- The citations are corrected.

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

| lemma | expected | valid | formal status | proven/checked | rule examined | agrees |
|---|---|---|---|---|---|---|
| C1-contradiction | refuted | true | AllVerified | 2/2 | no | **no** |
| C2-tautology | proved | true | AllVerified | 2/2 | no | **no** |
| C3-uses-lambda | refuted | true | AllVerified | 2/2 | no | **no** |
| C4-needs-lambda | proved | true | AllVerified | 2/2 | no | **no** |
| F1-charge-sound | refuted | true | AllVerified | 2/2 | no | **no** |
| F2-exhausted-stays | refuted | true | AllVerified | 2/2 | no | **no** |
| F3-fits-charged | refuted | true | AllVerified | 2/2 | no | **no** |
| F4-run-stays-in-budget | refuted | true | AllVerified | 2/2 | no | **no** |
| F5-join-commutes | refuted | true | AllVerified | 2/2 | no | **no** |
| F6-trust-meet | refuted | true | AllVerified | 2/2 | no | **no** |
| F7-deny-beats-confirm | refuted | true | AllVerified | 2/2 | no | **no** |
| T1-charge-sound | proved | true | AllVerified | 2/2 | no | **no** |
| T2-exhausted-stays | proved | true | AllVerified | 2/2 | no | **no** |
| T3-fits-charged | proved | true | AllVerified | 2/2 | no | **no** |
| T4-run-stays-in-budget | proved | true | AllVerified | 2/2 | no | **no** |
| T5-join-commutes | proved | true | AllVerified | 2/2 | no | **no** |
| T6-trust-meet | proved | true | AllVerified | 2/2 | no | **no** |
| T7-deny-beats-confirm | proved | true | AllVerified | 2/2 | no | **no** |

Every document reports `valid: true`, tier Platinum, `AllVerified 2/2`. The "rule
examined" column compares that with the same document with its `⟦Γ⟧` block removed,
which gives an identical result for all 18. Nothing the CLI reports depends on the
lemma.

## 2. Library: `FormalVerifier::verify` (`driver/`)

| lemma | expected | status | checked/ok/failed | verdict | agrees |
|---|---|---|---|---|---|
| C1-contradiction | refuted | Verified (nothing checked) | 0/0/0 | none | **no** |
| C2-tautology | proved | Verified (nothing checked) | 0/0/0 | none | **no** |
| C3-uses-lambda | refuted | Verified (nothing checked) | 0/0/0 | none | **no** |
| C4-needs-lambda | proved | Verified (nothing checked) | 0/0/0 | none | **no** |
| F1-charge-sound | refuted | Verified (nothing checked) | 0/0/0 | none | **no** |
| F2-exhausted-stays | refuted | Verified (nothing checked) | 0/0/0 | none | **no** |
| F3-fits-charged | refuted | Verified (nothing checked) | 0/0/0 | none | **no** |
| F4-run-stays-in-budget | refuted | Verified (nothing checked) | 0/0/0 | none | **no** |
| F5-join-commutes | refuted | Verified (nothing checked) | 0/0/0 | none | **no** |
| F6-trust-meet | refuted | Verified (nothing checked) | 0/0/0 | none | **no** |
| F7-deny-beats-confirm | refuted | Verified (nothing checked) | 0/0/0 | none | **no** |
| T1-charge-sound | proved | Verified (nothing checked) | 0/0/0 | none | **no** |
| T2-exhausted-stays | proved | Verified (nothing checked) | 0/0/0 | none | **no** |
| T3-fits-charged | proved | Verified (nothing checked) | 0/0/0 | none | **no** |
| T4-run-stays-in-budget | proved | Verified (nothing checked) | 0/0/0 | none | **no** |
| T5-join-commutes | proved | Verified (nothing checked) | 0/0/0 | none | **no** |
| T6-trust-meet | proved | Verified (nothing checked) | 0/0/0 | none | **no** |
| T7-deny-beats-confirm | proved | Verified (nothing checked) | 0/0/0 | none | **no** |

The library reports `Verified` with **zero** properties checked. The status means "no
failures", which holds trivially when nothing was examined.

## Why: read from the source, not inferred from the output

All paths are relative to `core/crates/`.

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
not reach a solver intact today, for four reasons:

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
  their structure, but every other operator becomes an uninterpreted atom named after
  the operator (`LessEqual`, `LessThan`, `Equivalence`, `Union`, …). Its arguments are
  the literal variables `left` and `right`, whatever the operands were, and `Raw`
  becomes the atom `unknown`. A solver given that formula would see shapes, not
  arithmetic: T1 `LessEqual(left,right)⇒LessEqual(left,right)` would still be proved as
  a propositional tautology, while T2's `≤`/`>` atoms become unrelated propositions
  and lose the arithmetic that makes T2 true. So any verdict it returned would follow
  from the formula's shape, not from the lemma.
- **A second rule path:** `aisp-core/src/ast/canonical/blocks.rs::parse_logical_rule`
  wraps every rule as `Raw`, with no quantifier.

A fix needs all of the following:

1. The rule parser binds several variables per quantifier and parses arithmetic.
2. The converter keeps quantifiers, operands and `⟦Λ⟧` definitions. A rule over a
   defined symbol is checked with the definition unfolded (C4), or is `Unknown`.
3. `⟦Γ⟧` is routed to the `properties` verifier, which asserts the negation: `unsat`
   means proved, `sat` means refuted with a model, and anything out of fragment is
   `Unknown`.
4. `Verified` is never reported when zero properties were checked.

Then re-run this corpus as the acceptance test. `run.sh` and the driver should both
exit 0 and agree with `ground-truth.sh` row for row. Today they exit 1 on every row.
Tracked in #33 (CLI verdict) and #34 (`FormalVerifier`).
