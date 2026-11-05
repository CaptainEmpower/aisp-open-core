# Results: 2026-11-05

**Verdict: the verifier fails the experiment.** It returns *verified* for every lemma,
including all 7 false mutants, the contradiction `∀x∈ℕ:x<x`, and the
definition-dependent false lemma. No false lemma was refuted, and none came back
`Unknown` either. The true lemmas also "pass", but that is not evidence, because the
same output is produced whatever the lemma says.

Conditions: aisp-open-core `main` @ `e412f3e`, `aisp-cli` release build with
`z3-verification` (links `libz3.so.4`, Z3 4.8.12), Ubuntu 24.04, x86-64.

## 1. CLI: `aisp-cli -l formal --formal-verification -f json` (`./run.sh`)

| lemma | expected | valid | formal status | proven/checked | agrees |
|---|---|---|---|---|---|
| C1-contradiction | refuted | true | AllVerified | 2/2 | **no** |
| C2-tautology | proved | true | AllVerified | 2/2 | yes |
| C3-uses-lambda | refuted | true | AllVerified | 2/2 | **no** |
| F1-charge-sound | refuted | true | AllVerified | 2/2 | **no** |
| F2-exhausted-stays | refuted | true | AllVerified | 2/2 | **no** |
| F3-fits-charged | refuted | true | AllVerified | 2/2 | **no** |
| F4-bits-monotone | refuted | true | AllVerified | 2/2 | **no** |
| F5-join-commutes | refuted | true | AllVerified | 2/2 | **no** |
| F6-trust-meet | refuted | true | AllVerified | 2/2 | **no** |
| F7-deny-beats-confirm | refuted | true | AllVerified | 2/2 | **no** |
| T1-charge-sound | proved | true | AllVerified | 2/2 | yes |
| T2-exhausted-stays | proved | true | AllVerified | 2/2 | yes |
| T3-fits-charged | proved | true | AllVerified | 2/2 | yes |
| T4-bits-monotone | proved | true | AllVerified | 2/2 | yes |
| T5-join-commutes | proved | true | AllVerified | 2/2 | yes |
| T6-trust-meet | proved | true | AllVerified | 2/2 | yes |
| T7-deny-beats-confirm | proved | true | AllVerified | 2/2 | yes |

Every document reports `valid: true` at tier Platinum (δ=1.0).

## 2. Library: `FormalVerifier::verify` (`driver/`)

| lemma | status | properties |
|---|---|---|
| C1-contradiction | Verified | checked=0 ok=0 failed=0 |
| C2-tautology | Verified | checked=0 ok=0 failed=0 |
| C3-uses-lambda | Verified | checked=0 ok=0 failed=0 |
| F1-charge-sound | Verified | checked=0 ok=0 failed=0 |
| F2-exhausted-stays | Verified | checked=0 ok=0 failed=0 |
| F3-fits-charged | Verified | checked=0 ok=0 failed=0 |
| F4-bits-monotone | Verified | checked=0 ok=0 failed=0 |
| F5-join-commutes | Verified | checked=0 ok=0 failed=0 |
| F6-trust-meet | Verified | checked=0 ok=0 failed=0 |
| F7-deny-beats-confirm | Verified | checked=0 ok=0 failed=0 |
| T1-charge-sound | Verified | checked=0 ok=0 failed=0 |
| T2-exhausted-stays | Verified | checked=0 ok=0 failed=0 |
| T3-fits-charged | Verified | checked=0 ok=0 failed=0 |
| T4-bits-monotone | Verified | checked=0 ok=0 failed=0 |
| T5-join-commutes | Verified | checked=0 ok=0 failed=0 |
| T6-trust-meet | Verified | checked=0 ok=0 failed=0 |
| T7-deny-beats-confirm | Verified | checked=0 ok=0 failed=0 |

`Verified` with **zero** properties checked: the status is "no failures", which
holds trivially when nothing was examined.

## Why: read from the source, not inferred from the output

All paths are relative to `core/crates/`.

1. **CLI `AllVerified` is a confidence threshold.** `aisp-cli/src/main.rs:408` sets
   it when the `DeepVerifier`'s `overall_confidence > 0.9`. The two "properties" are
   `TypeSystem` and `LogicConsistency`, each passed when a heuristic score exceeds 0.8
   (`aisp-core/src/semantic/deep_verifier/mod.rs:243-262`). Neither one is the lemma.
2. **`FormalVerifier` never reads `⟦Γ⟧`.** `aisp-core/src/invariant_analyzer.rs:45-48`
   discovers invariants from `⟦Σ⟧` type definitions only. The function-invariant pass
   is an empty stub (`:103-107`). The repo's own examples confirm this: their checked
   properties are type-derived (`∀x:Counter. WellFormed(x)`). Even a true one,
   `∀x:Counter. x≥0`, fails with `ProofGenerationFailed: Proof not found`
   (`run_verification` Test 3), so that path is unsound in the other direction too.
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
discharged nothing which ADR-0194's consequence 2 exists to refuse.

## What would make it pass

Route `⟦Γ⟧` rules through `formula_converter` / `smt_formula_converter` into the
`properties` verifier, and assert the negation. `unsat` means proved, `sat` means
refuted with a model, and anything out of fragment is `Unknown`. Never report
`Verified` when zero properties were checked. Then re-run this corpus: it is the
acceptance test, and it should fail before that fix and pass after it.
