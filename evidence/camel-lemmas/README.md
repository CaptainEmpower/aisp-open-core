# Known-true / known-false lemma experiment (camel-core)

**Question:** when the verifier is given a lemma whose truth value is already known,
does it return that value? In particular, does it *refute* a false lemma, rather than
returning `Unknown` or "verified"?

**Corpus** (`lemmas/`, one lemma per document, expected verdict in `⟦Ω⟧ expect≜`).
Each true form is a small arithmetic, set or propositional **abstraction** of a
property that a Kani harness in gents-camel (`core/rust/src`) proves over the real
Rust. The lemma is not that harness's statement, and Kani discharged none of these
formulas: the harnesses reason about the actual functions over machine integers,
while these lemmas state plain facts over ℕ, sets and booleans. The column below
says what each one abstracts.

| id | abstracts | true form | false mutant |
|----|-----------|-----------|--------------|
| 1 | `policy/leak_proofs.rs::a_charge_never_carries_the_tally_past_the_budget`: only the conclusion `spent+debit ≤ budget`, as `p⇒p` (no model of `leak_check_for`) | `≤` | `<` |
| 2 | `policy/leak_proofs.rs::an_exhausted_budget_stays_exhausted`: a debit that does not fit still does not fit once more has been spent | with `s≤t` | hypothesis dropped |
| 3 | `policy/leak_proofs.rs::a_debit_that_fits_is_never_refused`: one direction of the arithmetic behind it (the harness proves the biconditional over `leak_check_for`) | debit bounded | debit unbounded |
| 4 | `policy/leak_proofs.rs::a_run_of_calls_never_spends_past_the_budget`: the induction step, with the charge-or-refuse decision written out | with `s≤b` | hypothesis dropped |
| 5 | `taint/cap/proofs.rs::join_is_a_semilattice`: commutativity, with a cap's sources as a set and join as `∪` | `a∪b≡b∪a` | `a∪b≡a` |
| 6 | `taint/cap/proofs.rs::trust_is_the_meet_under_join`: "trusted" as "no sources" | `∧` | `∨` |
| 7 | `policy/engine/proofs.rs::deny_beats_confirm`: the hit branch only, with the decision given as a hypothesis (the harness proves `decide()` makes it) | `¬confirm` | `confirm` |

Controls:

- `C1`: a contradiction (must refute).
- `C2`: a tautology (must prove).
- `C3`: a false lemma over a `⟦Λ⟧`-defined symbol (must refute). It checks that a defined symbol does not stop a refutation. It cannot check that the definition is unfolded, because a false lemma is just as refutable with the symbol left uninterpreted.
- `C4`: the same symbol in a lemma that is true **only** once the definition is unfolded (must prove). This is the control that tells a verifier that unfolds definitions from one that does not.

Every expected verdict is checked against Z3 directly by `ground-truth.sh`. It holds a
hand-written SMT-LIB encoding of each lemma and requires the negation to be `unsat`
for `proved` and `sat` for `refuted`. So each expected answer is both correct and
decidable by the solver a fixed verifier would use. This experiment tests the AISP
verifier, not camel-core.

**Pass criterion:** every `F*` and `C1`/`C3` lemma is refuted, and every `T*` and
`C2`/`C4` lemma is proved. A result of `Unknown` counts as *not refuted*.

A verdict counts only if the lemma's rule decided it. Neither entry point reports a
per-rule result, so both runners use the same proxy. Each lemma is paired with the one
that shares its skeleton and differs only in the rule: `T_n` ↔ `F_n`, `C2` ↔ `C1` and
`C4` ↔ `C3`. A row agrees only if its verdict matches its expectation **and** differs
from its partner's. Output that does not depend on what the rule says gives both
members the same verdict and fails both rows. Examples are a heuristic score, a
hard-coded `Proven`, or properties derived from `⟦Σ⟧`. What counts as *proved* and
*refuted* for each entry point is defined in the header of `run.sh` and of
`driver/src/main.rs`, and both apply the pairing.

Run (each exits 1 on any disagreement):

- `./ground-truth.sh`: the expected verdicts against Z3 (needs `z3`; set `Z3` to its path). This one should pass.
- `./run.sh`: the CLI path (needs a Z3-enabled `aisp-cli`; set `AISP_CLI` to its path).
- `cargo run --release --manifest-path driver/Cargo.toml -- lemmas/*.aisp`: the library path (`FormalVerifier`).

Results: `RESULTS.md`.
