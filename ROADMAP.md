# AISP Open Core — Roadmap

This roadmap follows the June 2026 architecture and scientific review of the repository.
It is organized around one principle: **every claim the project makes must be backed by
runnable, reproducible evidence in this repository.**

Work items carry stable IDs (`R-xx`) so commits and PRs can reference them
(e.g. `Fixes R-03`). When GitHub Issues are enabled for this repository, each open
item should be filed as an issue 1:1 and linked back here.

---

## Current state (review summary, 2026-06)

**Working today**

- Pest-based, Unicode-aware parser with format detection and a canonical AST
  (`core/crates/aisp-core/src/parser/`)
- Structural validator with tiered heuristic scoring
- 1,044 passing unit tests
- npm/WASM validator package (`tools/validator/`)
- Substantial ADR trail (30+ records)

**Broken or unsubstantiated today**

- Building requires system Z3 headers (`z3-sys` fails with `z3.h not found`
  otherwise) — this is intentional (proving is on by default) but undocumented
- End-to-end validation suite fails 8/11 tests — the validator rejects its own fixtures
- npm validator test harness cannot locate its tests
- `reference_validator` "verification" returns hardcoded `true` results
  (acknowledged in `docs/research/soundness_completeness_analysis.md`)
- Headline numbers (40–65% NL ambiguity, <2% AISP ambiguity, 97× pipeline
  improvement, +22% SWE-bench) have no supporting artifacts or citations in the repo
- Committed `.rlib` binaries; merge-conflict markers committed in
  `docs/examples/reference.md`; `WORKSPACE.md` describes a nonexistent layout
- No CI

---

## Phase 0 — Make the repository trustworthy

Small, mechanical, high-leverage. Each item is a focused PR.

| ID | Item | Acceptance criteria | Status |
|----|------|---------------------|--------|
| R-01 | Repo hygiene: remove committed `.rlib` binaries, resolve committed merge-conflict markers in `docs/examples/reference.md`, fix `WORKSPACE.md` drift | `git ls-files` contains no `.rlib`; no `<<<<<<<` markers in tracked files; WORKSPACE.md commands run as written | PR #3 |
| R-02 | Z3 build prerequisite: Z3 proving **stays in default features** (maintainer decision, 2026-06-12 — proving is core; PR #5 closed); document the system Z3 requirement (`libz3-dev` / `brew install z3`) prominently | README/WORKSPACE document the prerequisite; CI provisions Z3 on every job; a checkout with Z3 installed builds and tests green | Planned |
| R-03 | Fix npm validator test harness (`npm test` cannot resolve `test/`) | `npm test` discovers and runs tests with non-zero pass count | PR #4 |
| R-04 | Add CI: Rust (Z3 provisioned on all jobs), fmt/clippy, npm validator | Workflow green on default branch; runs on all PRs | Planned |
| R-05 | Fix end-to-end validation suite (8/11 failing at `end_to_end_validation.rs:94`) by fixing the actual cause, not the assertions | All 11 e2e tests pass with the root cause documented in the PR | Planned |

## Phase 1 — Honest claims

The README and reference docs must not state what the repository cannot demonstrate.

| ID | Item | Acceptance criteria | Status |
|----|------|---------------------|--------|
| R-06 | Rewrite README/reference claims: present 40–65%, <2%, 97×, and +22% SWE-bench as hypotheses under validation or remove them; remove "mathematically proven" language; clarify the Harvard badge refers to an ALM capstone | No quantitative claim in README/docs without a linked artifact (data, harness, or citation) | Planned |
| R-07 | Reconcile contradictory research docs: `VERIFICATION_SUCCESS_REPORT.md` ("MATHEMATICALLY PROVEN") vs `soundness_completeness_analysis.md` ("DO NOT RELY ON CURRENT VERIFICATION RESULTS") — keep the accurate one, retract or annotate the other | A single, consistent statement of verification status in `docs/research/` | Planned |
| R-08 | Replace hardcoded results in `reference_validator/feature_verification.rs` with real checks or explicit `NotImplemented` status | No verification path returns `smt_verified: true` from a constant | Planned |

## Phase 2 — Engineering consolidation

| ID | Item | Acceptance criteria | Status |
|----|------|---------------------|--------|
| R-09 | Consolidate duplicate pipelines: `semantic/pipeline/` vs `semantic/verification_pipeline/` | One pipeline, with the other removed or merged; callers updated | Planned |
| R-10 | Isolate research modules (`anti_drift`, `ghost_intent_search`, `pocket_architecture`, temporal stubs, etc.) into an `experimental` module or feature flag, clearly marked unsupported | Core build/test does not depend on experimental modules; README distinguishes supported vs experimental surface | Planned |
| R-11 | Replace `panic!()` in parser/validator paths with `Result`-based errors | No `panic!` reachable from public parse/validate APIs; fuzz/property tests confirm | Planned |
| R-12 | Consolidate the 15 per-module `types.rs` files into a coherent shared type hierarchy; reduce the 337 build warnings | Warning count ratcheted down in CI; shared error/result types | Planned |
| R-13 | Deduplicate the five overlapping `formal_verification_*` test files into one suite | Single formal-verification test entry point; no redundant suites | In progress — ten suites gated behind nonexistent `*-deprecated` features removed; `property_testing_formal` restored to current APIs (13 tests passing). Remaining: consolidate the known-failing integration suites |

## Phase 3 — Formal verification done right

Scoped to what is actually decidable, with soundness before breadth.
The phased plan in `docs/research/soundness_completeness_analysis.md` is the reference.

| ID | Item | Acceptance criteria | Status |
|----|------|---------------------|--------|
| R-14 | Define the decidable fragment of AISP that will be verified (syntax, basic types, bounded arithmetic); document exclusions (self-reference, recursive optimization) | Written fragment definition in `docs/research/` | Planned |
| R-15 | Implement real SMT verification for the fragment: properties proven as **validity** (UNSAT of negation), never satisfiability presented as proof | Each verified property has a Z3 query checked into the repo with expected UNSAT result; CI runs them under the z3 feature | Planned |
| R-16 | Resource bounds: timeouts and memory limits on all solver calls; `Unknown` is a first-class result | No unbounded solver invocation; timeout paths tested | Planned |

## Phase 4 — Empirical validation (science)

The path from "interesting hypothesis" to "peer-reviewed result".

| ID | Item | Acceptance criteria | Status |
|----|------|---------------------|--------|
| R-17 | Operationalize "ambiguity" as a measurable quantity: cross-model output agreement across k models × n samples per spec, NL vs AISP | Published harness + raw data in repo; replaces the unsourced 40–65% → <2% claim | Planned |
| R-18 | Preregistered benchmark: AISP vs natural language vs structured baselines (JSON schema, pseudo-code per Mishra et al. EMNLP 2023) on SWE-Bench Verified, ≥3 model families, fixed decoding params, significance tests | Public harness, logs, and analysis notebook; replaces the unverifiable +22% claim | Planned |
| R-19 | Pipeline compounding experiment: real 5- and 10-step agent chains with prose vs AISP handoffs, measured end-task success | Published results replace the theoretical (0.62)ⁿ vs (0.98)ⁿ table | Planned |
| R-20 | Write up and submit: arXiv preprint → workshop/conference (LLM-agents workshop, or RE/ICSE for the spec-language angle) | Preprint linked from README; submission made | Planned |

---

## Relevant prior work

- Mishra et al., *Prompting with Pseudo-Code Instructions*, EMNLP 2023 — structured prompts can outperform NL (supports the premise)
- Sclar et al., *FormatSpread*, ICLR 2024 — LLMs are highly format-sensitive and effects correlate weakly across models (contradicts "identical interpretation"; motivates R-17)
- Tam et al., *Let Me Speak Freely?*, 2024 — strict format constraints can degrade reasoning (must be measured, not assumed; motivates R-18)
- Cemri et al., *Why Do Multi-Agent LLM Systems Fail?*, 2025 — empirical failure taxonomy for agent pipelines (motivates R-19)

## Sequencing

```
Phase 0 (R-01..R-05)  →  unblocks everything; do first, in parallel PRs
Phase 1 (R-06..R-08)  →  requires no new experiments, only honesty
Phase 2 (R-09..R-13)  →  background refactoring, can interleave
Phase 3 (R-14..R-16)  →  after R-04/R-08; needs green CI with Z3 provisioned
Phase 4 (R-17..R-20)  →  independent of Rust work; can start any time
```
