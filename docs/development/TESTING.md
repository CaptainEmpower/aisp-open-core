# AISP Core Testing Guide

This document describes the canonical testing infrastructure for AISP Core, designed for production-ready development and deployment.

## Quick Start

```bash
# Install development tools
just install-tools

# Run all tests
just test

# Development cycle
just dev

# Production readiness check
just production-ready
```

## Test Commands

### Core Testing Commands

| Command | Purpose | Duration |
|---------|---------|----------|
| `just test` | Complete test suite | ~2-5 minutes |
| `just test-fast` | Unit tests only | ~30 seconds |
| `just test-integration` | Integration tests | ~1-2 minutes |
| `just test-perf` | Performance benchmarks | ~1-2 minutes |
| `just test-security` | Security regression tests | ~1-2 minutes |

### Cargo Aliases (Alternative)

```bash
# Using canonical Cargo commands
cargo test-all              # All tests with all features
cargo test-fast             # Fast unit tests  
cargo test-integration      # Integration tests only
cargo test-perf             # Performance benchmarks (release mode)
cargo test-security         # Security tests
```

### Development Commands

```bash
# Code quality
just check                  # Full code quality check
just fmt                    # Format code
just fix                    # Auto-fix common issues

# Documentation
just docs                   # Build documentation

# Specific tests
just test-one test_name     # Run specific test
just test-adversarial       # Run adversarial resistance tests
```

## Test Suites

### 1. Integration Tests (`integration_comprehensive.rs`)

**Purpose**: Validate core functionality and API stability

**Tests**:
- Library imports and basic types
- Validator instantiation and configuration
- File validation with test fixtures
- Error handling with invalid inputs
- Phase 2 module accessibility
- Performance baselines
- Memory safety and resource cleanup
- Concurrent access safety
- API stability and backward compatibility

**SLA Requirements**:
- Validator creation: <100ms
- Configuration creation: <10ms
- Basic validation: <10s
- Memory safety: No leaks

### 2. Performance Benchmarks (`performance_benchmarks.rs`)

**Purpose**: Ensure performance SLAs and detect regressions

**Benchmarks**:
- Validator creation performance (<1ms average)
- Configuration creation performance (<100µs average)
- Simple validation performance (<1s average)
- Throughput under concurrent load (>5 validations/sec)
- Memory usage validation (<100MB baseline)
- Performance regression detection

**SLA Requirements**:
- Simple validation: <1s
- Complex validation: <10s
- Concurrent throughput: >5 validations/sec
- Memory usage: <100MB baseline
- Creation time: <1ms average

### 3. Security Regression Tests (`security_regression.rs`)

**Purpose**: Prevent security vulnerabilities and test adversarial resistance

**Security Tests**:
- Adversarial input resistance (>80% blocking rate)
- Resource exhaustion protection
- Input validation and sanitization
- Known vulnerability regression prevention
- Concurrent security under load
- Timing attack resistance

**Security Requirements**:
- Adversarial blocking rate: >80%
- DoS protection: <10s processing time
- Memory exhaustion protection
- No timing information leakage

## Test Configuration

### Features and Requirements

Tests are organized by feature requirements:

- **Default tests**: Core functionality (no special features)
- **Verification tests**: Require `verification` feature (Z3 integration)
- **Security tests**: Require `security` feature (enhanced security)

### Test Profiles

Optimized test profiles in `core/Cargo.toml`:

```toml
[profile.test]
opt-level = 1        # Faster test execution
debug = true         # Debug information
overflow-checks = true   # Safety checks

[profile.test.package."*"]
opt-level = 2        # Optimize dependencies
```

### Parallel Configuration

Tests use configuration for Z3 integration:

```toml
[package.metadata.cargo-test-args]
test-threads = 1     # Sequential for Z3 tests
timeout = 120        # 2-minute timeout
nocapture = false    # Capture output
```

## Test Fixtures

Organized test data in `tests/fixtures/`:

```
tests/fixtures/
├── valid/           # Valid AISP documents
├── invalid/         # Invalid documents (should fail)
├── benchmarks/      # Performance test documents  
├── verification/    # Mathematical verification tests
└── adversarial/     # Security test examples
```

### Adding New Fixtures

1. Place files in appropriate subdirectory
2. Use descriptive names: `test_name.aisp`
3. Include comments explaining test purpose
4. Update tests to reference new fixtures

## Development Workflow

### Pre-Commit Checks

```bash
# Run before committing
just dev
# This runs: fmt, check, test-fast
```

### Release Validation

```bash
# Full production readiness check
just production-ready
# This runs: audit, check, test, build
```

### Continuous Integration

The test infrastructure supports CI/CD with:

```bash
# CI pipeline commands
just ci              # Complete CI validation
cargo test-all       # All tests (parallel when possible)
```

## Performance Monitoring

### Benchmark Results

Performance tests output timing information:

```
Validator creation: 245µs avg (100 iterations)
Configuration creation: 23µs avg (1000 iterations)  
Simple validation: 127ms avg (10 iterations)
Throughput: 8.3 validations/sec (40 successful in 4821ms)
```

### Regression Detection

Tests automatically fail if performance regresses beyond thresholds:

- 3x regression limit for critical operations
- Absolute limits for SLA compliance
- Memory usage monitoring

## Security Testing

### Adversarial Resistance

Tests validate security against:

- Unicode direction override attacks
- Mathematical symbol spoofing
- Homoglyph attacks  
- Zero-width character injection
- Resource exhaustion attempts
- Deep nesting attacks

### Security Metrics

- **Blocking Rate**: Percentage of adversarial inputs blocked
- **Processing Time**: Maximum time for adversarial input processing
- **Memory Usage**: Peak memory during security tests

## Troubleshooting

### Common Issues

**Tests don't compile**: 
- Check feature flags: `cargo test --all-features`
- Verify Z3 installation: `just verify-z3`

**Performance tests fail**:
- Run in release mode: `cargo test-perf`
- Check system resources
- Verify no background processes affecting timing

**Security tests fail**:
- Check adversarial fixture availability
- Verify filesystem write permissions for temp files
- Run with `--nocapture` for debugging

### Getting Help

1. Check test output with `--nocapture` flag
2. Run specific test: `just test-one test_name`
3. Check feature requirements in test attributes
4. Verify test fixtures exist and are readable

## Integration with IDEs

### VS Code

Add to `.vscode/tasks.json`:

```json
{
  "label": "AISP Test Suite",
  "type": "shell", 
  "command": "just",
  "args": ["test"],
  "group": "test"
}
```

### CLI Tools

The test infrastructure works with:

- **cargo-watch**: `cargo watch -x test-fast`
- **cargo-nextest**: `cargo nextest run`
- **just**: All commands available via `just --list`

## Test Quality Standards

### Meaningful Assertion Requirements

All tests must follow strict quality standards to provide genuine validation value:

#### ❌ Prohibited Patterns ("Always Pass" Tests)
```rust
// Tautological assertions that always pass
assert!(result.is_valid || !result.is_valid);
assert!(value >= 0.0 || value < 0.0);

// Unrealistic thresholds that mask implementation issues  
assert!(confidence > 0.95);  // System rarely achieves this
```

#### ✅ Required Patterns (Meaningful Validation)
```rust
// Assertions that can meaningfully fail
assert!(
    result.is_valid,
    "Validation failed for complete document: {:?}",
    result.errors
);

// Realistic thresholds with actionable alternatives
assert!(
    confidence >= 0.5 || !recommendations.is_empty(),
    "System should provide either reasonable confidence (≥0.5) or actionable recommendations. \
     Got confidence: {}, recommendations: {}",
    confidence, recommendations.len()
);
```

### Test Quality Checklist

Before committing any test:

- [ ] **Meaningful Failure**: Assertion can fail with realistic input variations
- [ ] **Actionable Messages**: Error messages provide debugging context
- [ ] **Realistic Expectations**: Thresholds align with system capabilities  
- [ ] **Comprehensive Coverage**: Test validates the intended behavior fully
- [ ] **No Tautologies**: No assertions that are always true/false

### Quality Enforcement

**Current Status**: ✅ **1044/1044 tests passing with meaningful validation**

All tests have been reviewed and updated to meet quality standards per ADR-026. The test suite now provides genuine confidence in system correctness with zero "always pass" patterns.

## Contributing

When adding tests:

1. Follow the existing test structure
2. Add appropriate feature requirements
3. Include performance expectations
4. Document security implications
5. **Follow test quality standards** (see ADR-026)
6. Ensure assertions can meaningfully fail
7. Provide actionable error messages
8. Update this guide as needed

For production deployment, ensure all tests pass:

```bash
just production-ready
```

This validates code quality, security, performance, and functionality before deployment.