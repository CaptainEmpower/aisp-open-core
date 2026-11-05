# Known-true / known-false lemma experiment (camel-core)

**Question:** when the verifier is given a lemma whose truth value is already known,
does it return that value? In particular, does it *refute* a false lemma, rather than
returning `Unknown` or "verified"?

**Corpus** (`lemmas/`, one lemma per document, expected verdict in `⟦Ω⟧ expect≜`):

| id | source in gents-camel `core/rust/src` | true form | false mutant |
|----|------|-----------|--------------|
| 1 | `policy/leak_proofs.rs::a_charge_never_carries_the_tally_past_the_budget` | `≤` | `<` |
| 2 | `policy/leak_proofs.rs::an_exhausted_budget_stays_exhausted` | with `s≤t` | hypothesis dropped |
| 3 | `policy/leak_proofs.rs::a_debit_that_fits_is_never_refused` | debit bounded | debit unbounded |
| 4 | `policy/leak_proofs.rs::bits_for_cardinality_is_monotone` | `≤` | `<` |
| 5 | `taint/cap/proofs.rs::join_is_a_semilattice` (commutativity) | `a∪b≡b∪a` | `a∪b≡a` |
| 6 | `taint/cap/proofs.rs::trust_is_the_meet_under_join` | `∧` | `∨` |
| 7 | `policy/engine/proofs.rs::deny_beats_confirm` | `¬confirm` | `confirm` |

Controls: `C1` contradiction (must refute), `C2` tautology (must prove), and `C3` a false
lemma that can only be refuted by unfolding a `⟦Λ⟧` definition.

Every true form is already discharged by Kani over the real Rust. This experiment
tests the AISP verifier, not camel-core.

**Pass criterion:** every `F*` and `C1`/`C3` lemma is refuted, and every `T*` and `C2`
lemma is proved. A result of `Unknown` counts as *not refuted*.

Run: `./run.sh` for the CLI (needs a Z3-enabled `aisp-cli`; set `AISP_CLI` to its path). For the library verifier: `cargo run --release --manifest-path driver/Cargo.toml -- lemmas/*.aisp`.
Results: `RESULTS.md`.
