# AISP Open Core Workspace

This repository contains the complete AISP (AI Symbolic Protocol) implementation ecosystem organized as a unified Rust workspace.

## 📁 **Project Structure**

```
aisp-open-core/
├── 📦 core/                               # Advanced formal verification system
│   ├── crates/aisp-core/                  # Core parsing and verification engine
│   └── crates/aisp-cli/                   # Command-line tools
├── 📦 tools/
│   ├── validator/                         # Node.js/WebAssembly implementation
│   ├── reference-verifier/                # Standalone Z3 reference checks
│   └── simple_z3_test/                    # Minimal Z3 integration example
├── 🗄️ archive/aisp-rust/                  # Published crate (simple API, archived)
├── 📚 evidence/                           # Test documents and examples
├── 📝 docs/                               # Guides, architecture docs, research notes
├── 🔧 Cargo.toml                          # Unified workspace configuration
└── 🔧 justfile                            # Development workflows
```

## 🚀 **Quick Start**

### Build Everything
```bash
# Build all workspace members
cargo build --workspace --release

# Run tests across all crates
cargo test --workspace

# Run the advanced CLI
./target/release/aisp-cli validate evidence/tic-tac-toe/spec.aisp
```

> **Note:** the `z3-verification` feature requires a system Z3 installation
> (`apt-get install libz3-dev` on Debian/Ubuntu, `brew install z3` on macOS).

### Use Individual Components

**Advanced CLI with Formal Verification:**
```bash
cargo run -p aisp-cli -- --level formal --format detailed validate evidence/tic-tac-toe/spec.aisp
```

**Node.js/WebAssembly:**
```bash
cd tools/validator
npm install
npx aisp-validator validate ../../evidence/tic-tac-toe/spec.aisp
```

**Simple Rust API (archived):**
```bash
cd archive/aisp-rust
cargo run --example basic
```

## 🛠️ **Implementation Levels**

### 1. **core/** - Advanced Engine
- **Purpose**: Complete formal verification and analysis system
- **Status**: 🔬 Research/Advanced implementation
- **Crates**: `aisp-core` (engine), `aisp-cli` (command-line tools)
- **Features**:
  - Structural validation with tiered quality scoring
  - Ambiguity heuristics and semantic analysis
  - Z3 SMT integration (feature-gated, under active development)
  - Protocol state machine analysis
  - Concurrent behavior verification
- **Target**: Formal methods research

### 2. **tools/validator/** - Cross-Platform
- **Purpose**: Node.js/Browser/WebAssembly support
- **Published**: ✅ [npm](https://npmjs.com/package/aisp-validator)
- **Features**: Universal validation, browser support, WASM kernel
- **Target**: Web applications, JavaScript/TypeScript projects

### 3. **archive/aisp-rust/** - Published Library (archived)
- **Purpose**: Simple, stable API for basic AISP validation
- **Published**: ✅ [crates.io](https://crates.io/crates/aisp)
- **Status**: Archived; kept for reference, excluded from workspace tests
- **Target**: Production applications needing simple AISP validation

## 🔧 **Workspace Configuration**

### Shared Dependencies
All crates use workspace-managed versions for consistency:
- `serde` - Serialization
- `clap` - CLI interface
- `thiserror`/`anyhow` - Error handling
- `tokio` - Async runtime
- `uuid` - Unique identifiers

### Build Profiles

| Profile | Purpose | Optimizations |
|---------|---------|---------------|
| `dev` | Development | Minimal optimization, debug info |
| `release` | Production | Full LTO, strip symbols, panic=abort |
| `cli` | Command-line tools | Release + debug symbols stripped |
| `wasm` | WebAssembly | Size optimization (`opt-level="z"`) |

### Optional Features

**Z3 Integration:**
```bash
# Enable Z3 theorem proving (requires system Z3 installation)
cargo build --features z3-verification
```

**Individual Component Features:**
- `aisp-core`: `std`, `serde`, `z3-verification`
- `aisp-cli`: `z3-verification`
- `archive/aisp-rust`: `streaming`, `serde`, `wasm`, `z3`

## 🧪 **Testing Strategy**

### Test Organization
```bash
# Unit tests (individual crates)
cargo test -p aisp-core
cargo test -p aisp-cli

# Integration tests (workspace-level)
cargo test --workspace

# Specific test suites
cargo test test_formal_verification
cargo test test_enumeration_parsing_fix
```

### Test Categories
- **Unit Tests**: Individual component testing
- **Integration Tests**: Cross-component workflows
- **Implementation Validation**: Regression tests for improvements
- **Formal Verification Tests**: Mathematical correctness validation

## 📋 **Development Workflow**

### Building
```bash
# Fast development build
cargo build

# Optimized CLI for testing
cargo build --profile cli -p aisp-cli

# WebAssembly build
cd tools/validator/wasm && cargo build --profile wasm --target wasm32-unknown-unknown
```

### Testing New Features
```bash
# Test parser improvements
cargo test test_enumeration_parsing_fix

# Test formal verification
cargo run -p aisp-cli -- --level formal validate core/test_document.aisp

# Test ambiguity calculation
cargo test test_ambiguity_measurement
```

### Documentation
```bash
# Generate API documentation
cargo doc --workspace --open

# Check documentation
cargo doc --workspace --document-private-items
```

## 🏗️ **Architecture Overview**

### Published Components (Stable)
- **tools/validator**: Node.js/WebAssembly package
- **archive/aisp-rust**: Simple validation library (archived)

### Research Components (Advanced)
- **core/**: Formal verification system
  - **aisp-core**: Advanced parsing and formal verification engine
  - **aisp-cli**: Research tools for formal analysis

### Shared Resources
- **evidence/**: Test documents and examples
- **docs/**: Architecture decisions, guides, and research notes
- **docs/examples/reference.md**: AISP 5.1 specification reference

## 📚 **Documentation**

- **[AI_GUIDE.md](../user-guides/AI_GUIDE.md)**: Complete AISP 5.1 specification for AI agents
- **[HUMAN_GUIDE.md](../user-guides/HUMAN_GUIDE.md)**: Human-readable introduction to AISP
- **[core/docs/adr/](../../core/docs/adr/)** and **[docs/architecture/adrs/](adrs/)**: Architecture Decision Records
- **[ARCHITECTURE_ANALYSIS.md](ARCHITECTURE_ANALYSIS.md)**: Technical analysis
- **[ROADMAP.md](../../ROADMAP.md)**: Phased work plan and backlog

## 🤝 **Contributing**

1. **Choose Component**: Pick core/ (research) or tools/validator (npm package)
2. **Follow Patterns**: Use existing code style and architecture patterns
3. **Test Changes**: Run relevant test suites
4. **Update Documentation**: Keep ADRs and guides current

## 📄 **License**

Dual licensed under MIT OR Apache-2.0.
