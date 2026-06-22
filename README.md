# AISP Parser & Verifier

**A Rust implementation that parses [AISP](#reference) documents and formally verifies them — turning AISP's "proof-carrying" promise into machine-checkable proofs.**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![Z3](https://img.shields.io/badge/Verification-Z3-blue.svg)](https://github.com/Z3Prover/z3)

This project provides two things the AISP language did not have on its own:

1. **A parser** — a complete, spec-driven parser that turns raw AISP source text into a typed Abstract Syntax Tree.
2. **A verifier / proof engine** — a formal verification pipeline that discharges AISP's invariants, constraints, and proof obligations to the [Z3 SMT solver](https://github.com/Z3Prover/z3), producing machine-checkable results rather than self-asserted claims.

Our work **builds on** the original AISP specification (see [Reference](#reference)) — the language design and notation are taken from there. Everything in this repository is the *tooling* around that language: the parser, the type checker, and the verifier/proof system.

---

## Reference

This implementation is based on the **AISP (AI Symbolic Protocol)** specification by Bradley Ross.

- **Upstream specification & origin:** [`aisp-open-core`](https://github.com/bar181/aisp-open-core) — the AISP 5.1 language design, notation, and rationale.

The upstream project defines *what* AISP is. This repository implements *how* to parse and verify it. For the language itself — symbols, block structure, semantics — consult the upstream specification. The notes below describe only the parser and verifier built here.

---

## What This Repository Implements

### The Parser

`aisp-core` contains a from-scratch, spec-driven parser (`core/crates/aisp-core/src/parser`, with a [pest](https://pest.rs/) grammar) that:

- Tokenizes and parses the full AISP block structure (`⟦Ω⟧`, `⟦Σ⟧`, `⟦Γ⟧`, `⟦Λ⟧`, `⟦Χ⟧`, `⟦Ε⟧`).
- Builds a typed Abstract Syntax Tree (`ast/`) preserving definitions, rules, function signatures, and evidence blocks.
- Handles AISP's mathematical/Unicode notation (quantifiers, lambdas, tuples, type operators) with Unicode normalization.
- Reports precise, source-located parse errors.

### The Verifier / Proof Engine

On top of the parsed AST, the verifier translates AISP constraints into SMT formulas and discharges them with Z3. It is organized as a layered pipeline so you can stop at any depth:

| Level | What it checks |
|-------|----------------|
| **Syntax** | Well-formed structure and grammar |
| **Semantic** | Type checking and semantic analysis of definitions and rules |
| **Relational** | Constraint solving and conflict detection across rules |
| **Temporal** | Temporal-logic properties and model checking |
| **Formal** | Full formal verification: SMT discharge, theorem proving, invariant discovery, soundness/completeness analysis |

Supporting components include an SMT formula converter and generator, a theorem prover and proof search, invariant discovery/exporters, satisfiability checking, and tri-vector signal validation — all producing concrete, checkable verdicts instead of asserted ones.

---

## Build From Source

This workspace builds with **Z3 theorem proving enabled by default** — formal
verification is core to this tooling, so the [Z3 SMT solver](https://github.com/Z3Prover/z3)
is a required build prerequisite:

```bash
# Debian/Ubuntu
sudo apt-get install libz3-dev z3

# macOS
brew install z3

# Then build and test
cargo build --workspace
cargo test --workspace
```

If the build fails with `fatal error: 'z3.h' file not found`, the Z3
development headers are missing — install them as above.

> Parsing-only use cases that don't need formal verification can build the
> `aisp-core` crate with `--no-default-features --features minimal` to skip the
> Z3 dependency.

---

## CLI Usage

The `aisp-cli` crate exposes the parser and verifier as a command-line tool:

```bash
# Validate one or more AISP documents (parse + verify)
cargo run -p aisp-cli -- validate path/to/spec.aisp

# Fast syntax-only check (parser only)
cargo run -p aisp-cli -- check path/to/spec.aisp

# Analyze structure and metrics (symbols, complexity)
cargo run -p aisp-cli -- analyze path/to/spec.aisp --symbols --complexity

# Format / prettify a document
cargo run -p aisp-cli -- format path/to/spec.aisp --in-place

# List the validation levels described above
cargo run -p aisp-cli -- levels
```

Output is available in human-readable, JSON, detailed, and minimal formats for
both interactive use and CI integration.

---

## Library Usage

`aisp-core` can be used directly from Rust to parse and verify documents
programmatically:

```rust
use aisp_core::parser; // parse AISP source into a typed AST
use aisp_core::validator; // run the verification pipeline

// See `core/crates/aisp-core/examples/` for runnable examples.
```

## Repository Layout

| Path | Contents |
|------|----------|
| `core/crates/aisp-core` | The parser, type checker, and verifier/proof engine |
| `core/crates/aisp-cli` | Command-line interface |
| `archive/aisp-rust` | Archived snapshot of the original `aisp` crate (excluded from the workspace) |
| `docs/` | Architecture, development, and research notes |
| `evidence/` | Comparative analyses and test artifacts |

---

## License

This repository is licensed under **MIT OR Apache-2.0** (see `LICENSE`).

The AISP language specification it implements is the work of Bradley Ross; see
the [upstream reference](#reference) for the specification's own terms and
attribution.

## Citation

If you reference this tooling, please also cite the original AISP specification:

```bibtex
@misc{ross2026aisp,
  author = {Ross, Bradley},
  title = {AISP: AI Symbolic Protocol},
  year = {2026},
  publisher = {GitHub},
  howpublished = {\url{https://github.com/bar181/aisp-open-core}}
}
```
