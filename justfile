# AISP Core Development Tasks
# Modern task runner using just (https://just.systems/)
# Install with: cargo install just

# Default recipe
default:
    @just --list

# Build and test everything
all: check test

# Check code quality  
check:
    @echo "🔍 Running code quality checks..."
    just check-all
    just clippy-all

# Check all workspace packages
check-all:
    cargo check --workspace --all-targets

# Lint all workspace packages
clippy-all:
    cargo clippy --workspace --all-targets -- -W clippy::all -A warnings

# Format all workspace packages
fmt-all *args:
    cargo fmt --all {{args}}

# Run all tests
test:
    @echo "🧪 Running comprehensive test suite..."
    just test-unit
    just test-integration  
    just test-performance
    just test-security
    @echo "✅ All test suites completed successfully!"

# Run all tests without warnings (quiet mode for unused variables)
test-quiet:
    @echo "🔇 Running comprehensive test suite (quiet mode)..."
    just test-unit-quiet
    just test-integration-quiet
    just test-performance-quiet
    just test-security-quiet
    @echo "✅ All test suites completed successfully (quiet mode)!"

# Run unit tests only
test-unit:
    @echo "📦 Running unit tests..."
    cargo test-fast

# Run unit tests (quiet - no unused variable warnings)
test-unit-quiet:
    @echo "📦 Running unit tests (quiet)..."
    @RUSTFLAGS="-A unused-variables -A unused-imports -A dead-code -A unused-mut" cargo test-fast

# Run integration tests
test-integration:
    @echo "🔗 Running integration tests..."
    cargo test integration_comprehensive --all-features

# Run integration tests (quiet - no unused variable warnings)
test-integration-quiet:
    @echo "🔗 Running integration tests (quiet)..."
    @RUSTFLAGS="-A unused-variables -A unused-imports -A dead-code -A unused-mut" cargo test integration_comprehensive --all-features

# Run performance benchmarks
test-performance:
    @echo "🏃 Running performance benchmarks..."
    cargo test performance_benchmarks --all-features --release

# Run performance benchmarks (quiet - no unused variable warnings)
test-performance-quiet:
    @echo "🏃 Running performance benchmarks (quiet)..."
    @RUSTFLAGS="-A unused-variables -A unused-imports -A dead-code -A unused-mut" cargo test performance_benchmarks --all-features --release

# Run security tests
test-security:
    @echo "🔒 Running security tests..."
    cargo test security_regression --features security

# Run security tests (quiet - no unused variable warnings)
test-security-quiet:
    @echo "🔒 Running security tests (quiet)..."
    @RUSTFLAGS="-A unused-variables -A unused-imports -A dead-code -A unused-mut" cargo test security_regression --features security

# Build release version
build:
    @echo "🏗️ Building release version..."
    cargo build-release

# Build documentation
docs:
    @echo "📖 Building documentation..."
    cargo doc-all

# Clean everything
clean:
    @echo "🧹 Cleaning build artifacts..."
    cargo clean
    rm -rf target/
    find . -name "*.tmp" -delete
    find . -name "/tmp/*test*.aisp" -delete 2>/dev/null || true

# Format code
fmt:
    @echo "📐 Formatting code..."
    cargo fmt-all

# Fix common issues
fix:
    @echo "🔧 Auto-fixing issues..."
    cargo fix --allow-dirty --allow-staged
    cargo clippy-all --fix --allow-dirty --allow-staged

# Run specific test by name
test-one TEST:
    @echo "🎯 Running test: {{TEST}}"
    cargo test {{TEST}} -- --nocapture

# Quick development cycle
dev: fmt check test-unit
    @echo "✅ Development cycle complete"

# CI/CD pipeline
ci: check test build
    @echo "🚀 CI pipeline complete"

# Install development tools and setup environment
install-tools:
    @echo "🔧 Installing development tools..."
    cargo install just
    cargo install cargo-watch
    cargo install cargo-audit
    cargo install cargo-outdated

# Setup development environment (replaces Makefile functionality)
setup:
    @echo "🔧 Setting up AISP development environment..."
    @echo "🔍 Checking Z3 installation..."
    @if ! command -v z3 >/dev/null 2>&1; then \
        echo "❌ Z3 not found. Installing via Homebrew..."; \
        brew install z3; \
    else \
        echo "✅ Z3 found: $(z3 --version)"; \
    fi
    @echo "🔍 Checking LLVM/Clang installation..."
    @if ! brew list llvm >/dev/null 2>&1; then \
        echo "❌ LLVM not found. Installing via Homebrew..."; \
        brew install llvm; \
    else \
        echo "✅ LLVM found"; \
    fi
    @echo "✅ Environment setup complete!"

# Debug Z3 configuration
debug-z3:
    @echo "🐛 Z3 Configuration Debug"
    @echo "Environment Variables:"
    @echo "  Z3 Binary: $(which z3 || echo 'Not found')"
    @echo "  Z3 Version: $(z3 --version || echo 'N/A')"
    @echo "  Z3 Headers: $(if [ -f /opt/homebrew/include/z3.h ]; then echo '✅ Found'; else echo '❌ Missing'; fi)"
    @echo "  libclang: $(if [ -f /opt/homebrew/opt/llvm/lib/libclang.dylib ]; then echo '✅ Found'; else echo '❌ Missing'; fi)"

# Watch for changes and run tests
watch:
    @echo "👀 Watching for changes..."
    cargo watch -x "test-fast"

# Security audit
audit:
    @echo "🔍 Running security audit..."
    cargo audit
    cargo outdated

# Generate test coverage (if available)
coverage:
    @echo "📊 Generating test coverage..."
    cargo test-all --no-fail-fast
    @echo "⚠️  Coverage report generation requires additional tools"

# Benchmark specific performance tests
perf-validator:
    @echo "⚡ Benchmarking validator performance..."
    cargo test benchmark_validator_creation --release -- --nocapture

perf-throughput:
    @echo "⚡ Benchmarking throughput..."  
    cargo test benchmark_throughput --release -- --nocapture

# Profile memory usage (requires additional tools)
profile-memory:
    @echo "🧠 Profiling memory usage..."
    @echo "⚠️  Memory profiling requires platform-specific tools (valgrind, heaptrack, etc.)"

# Run adversarial tests only
test-adversarial:
    @echo "🛡️ Running adversarial resistance tests..."
    cargo test test_adversarial_resistance -- --nocapture

# Validate the full workspace (comprehensive validation)
validate:
    @echo "🔍 Running comprehensive workspace validation..."
    just check
    just test
    just audit
    @echo "✅ Workspace validation complete!"

# Validate AISP document using CLI
aisp-validate-cli FILE:
    @echo "🔍 Validating AISP document: {{FILE}}"
    cargo build --release --bin aisp-cli
    ./target/release/aisp-cli validate "{{FILE}}"

# Validate specific fixture
validate-fixture FIXTURE:
    @echo "🔍 Validating fixture: {{FIXTURE}}"
    ./core/target/release/aisp-cli --level formal --format detailed validate "{{FIXTURE}}"

# Build CLI and validate all fixtures
validate-all-fixtures:
    @echo "🎯 Validating all test fixtures..."
    cargo build-release
    @echo "Valid fixtures:"
    find tests/fixtures/valid -name "*.aisp" -exec ./core/target/release/aisp-cli validate {} \; 2>/dev/null || true
    @echo "Invalid fixtures (should fail):"
    find tests/fixtures/invalid -name "*.aisp" -exec ./core/target/release/aisp-cli validate {} \; 2>/dev/null || true

# Production readiness check
production-ready: audit check test build
    @echo "🚀 Production readiness validation complete"
    @echo "✅ Code quality checks passed"
    @echo "✅ All tests passed" 
    @echo "✅ Security audit passed"
    @echo "✅ Release build successful"

# Show project statistics
stats:
    @echo "📊 Project Statistics"
    @echo "====================="
    @echo "Lines of code:"
    find . -name "*.rs" -not -path "./target/*" -not -path "./archive/*" | xargs wc -l | tail -1
    @echo ""
    @echo "Test files:"
    find . -name "*test*.rs" -not -path "./target/*" | wc -l
    @echo ""
    @echo "Test fixtures:"
    find tests/fixtures -name "*.aisp" | wc -l
    @echo ""
    @echo "Dependencies:"
    cargo tree --depth 1 | wc -l

# Help for specific commands
help COMMAND:
    @echo "Help for: {{COMMAND}}"
    @just --show {{COMMAND}}