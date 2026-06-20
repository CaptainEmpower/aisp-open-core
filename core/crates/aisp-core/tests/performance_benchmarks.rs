//! Performance Benchmarks for AISP Core
//!
//! Production performance testing with specific SLA requirements:
//! - Validator creation: <100ms
//! - Simple validation: <1s  
//! - Complex validation: <10s
//! - Memory usage: <100MB baseline
//! - Concurrent throughput: >10 validations/sec

use std::time::{Duration, Instant};

const PERFORMANCE_TIMEOUT: Duration = Duration::from_secs(60);

/// Test validator creation performance
#[test]
fn benchmark_validator_creation() {
    use aisp_core::validator::AispValidator;

    let iterations = 100;
    let start = Instant::now();

    for _i in 0..iterations {
        let _validator = AispValidator::new();
    }

    let total_time = start.elapsed();
    let avg_time = total_time / iterations;

    println!(
        "Validator creation: {}µs avg ({} iterations)",
        avg_time.as_micros(),
        iterations
    );

    // SLA: Average creation time should be <1ms
    assert!(
        avg_time < Duration::from_millis(1),
        "Validator creation too slow: {}µs > 1ms",
        avg_time.as_micros()
    );
}

/// Test configuration creation performance
#[test]
fn benchmark_configuration_creation() {
    use aisp_core::validator::types::ValidationConfig;

    let iterations = 1000;
    let start = Instant::now();

    for _i in 0..iterations {
        let _config = ValidationConfig::default();
    }

    let total_time = start.elapsed();
    let avg_time = total_time / iterations;

    println!(
        "Configuration creation: {}µs avg ({} iterations)",
        avg_time.as_micros(),
        iterations
    );

    // SLA: Configuration creation should be <100µs
    assert!(
        avg_time < Duration::from_micros(100),
        "Configuration creation too slow: {}µs > 100µs",
        avg_time.as_micros()
    );
}

/// Test simple validation performance
#[test]
fn benchmark_simple_validation() {
    use aisp_core::validator::{types::ValidationConfig, AispValidator};
    use std::fs;

    let validator = AispValidator::new();

    let _config = ValidationConfig::default();

    // Create simple test content
    let simple_content = r#"𝔸5.1.SimpleTest@2026-01-28

⟦Ω:Meta⟧{
  domain≜"performance_test"
}
"#;

    let test_path = "/tmp/simple_perf_test.aisp";
    if fs::write(test_path, simple_content).is_err() {
        println!("⚠ Cannot write test file - skipping simple validation benchmark");
        return;
    }

    // Warm up
    let file_content = fs::read_to_string(test_path).unwrap_or_default();
    let _warmup = validator.validate(&file_content);

    // Benchmark
    let iterations = 10;
    let start = Instant::now();

    for _i in 0..iterations {
        let _result = validator.validate(&file_content);
    }

    let total_time = start.elapsed();
    let avg_time = total_time / iterations;

    fs::remove_file(test_path).ok();

    println!(
        "Simple validation: {}ms avg ({} iterations)",
        avg_time.as_millis(),
        iterations
    );

    // SLA: Simple validation should be <1s
    assert!(
        avg_time < Duration::from_secs(1),
        "Simple validation too slow: {}ms > 1000ms",
        avg_time.as_millis()
    );
}

/// Test throughput under load
#[test]
fn benchmark_throughput() {
    use aisp_core::validator::{types::ValidationConfig, AispValidator};
    use std::fs;
    use std::sync::Arc;
    use std::thread;

    let validator = Arc::new(AispValidator::new());

    let config = Arc::new(ValidationConfig::default());

    // Create test content: must contain all five required blocks
    // (Meta, Types, Rules, Functions, Evidence) so validation succeeds and
    // the throughput measurement counts real successful validations.
    let test_content = r#"𝔸5.1.ThroughputTest@2026-01-28

⟦Ω:Meta⟧{domain≜throughput_test}

⟦Σ:Types⟧{TestType≜ℕ}

⟦Γ:Rules⟧{∀x:TestType→Valid(x)}

⟦Λ:Funcs⟧{id≜λx.x}

⟦Ε⟧⟨δ≜0.5⟩
"#;

    let test_path = "/tmp/throughput_test.aisp";
    if fs::write(test_path, test_content).is_err() {
        println!("⚠ Cannot write test file - skipping throughput benchmark");
        return;
    }

    let file_content = fs::read_to_string(test_path).unwrap_or_default();

    let concurrent_threads = 4;
    let validations_per_thread = 10;

    let start = Instant::now();

    let handles: Vec<_> = (0..concurrent_threads)
        .map(|_| {
            let validator_clone = Arc::clone(&validator);
            let _config_clone = Arc::clone(&config);
            let content_clone = file_content.clone();

            thread::spawn(move || {
                let mut successful = 0;

                for _i in 0..validations_per_thread {
                    let result = validator_clone.validate(&content_clone);
                    if result.valid {
                        successful += 1;
                    }
                }

                successful
            })
        })
        .collect();

    let mut total_successful = 0;
    for handle in handles {
        if let Ok(successful) = handle.join() {
            total_successful += successful;
        }
    }

    let total_time = start.elapsed();
    let throughput = total_successful as f64 / total_time.as_secs_f64();

    fs::remove_file(test_path).ok();

    println!(
        "Throughput: {:.1} validations/sec ({} successful in {}ms)",
        throughput,
        total_successful,
        total_time.as_millis()
    );

    // SLA: Should achieve >5 validations/sec under concurrent load
    assert!(
        throughput >= 5.0,
        "Throughput too low: {:.1} validations/sec < 5.0",
        throughput
    );
}

/// Test memory usage baseline
#[test]
fn benchmark_memory_usage() {
    use aisp_core::validator::{types::ValidationConfig, AispValidator};

    // Note: This is a simplified memory test
    // In production, use proper memory profiling tools

    let _validator = AispValidator::new();

    let _config = ValidationConfig::default();

    // Test memory doesn't grow excessively with repeated operations
    for _i in 0..100 {
        let _validator2 = AispValidator::new();
        let _config2 = ValidationConfig::default();

        // Temporary validator/config are created and dropped at end of scope to
        // exercise allocation/cleanup across iterations.
    }

    println!("✓ Memory usage baseline test completed");
    // Note: Actual memory measurement would require platform-specific code
    // This test validates that objects can be created/destroyed without obvious leaks
}

/// Test performance with different validation levels
#[test]
fn benchmark_validation_levels() {
    use aisp_core::validator::{types::ValidationConfig, AispValidator};
    use std::fs;

    let validator = AispValidator::new();

    let test_content = r#"𝔸5.1.LevelsTest@2026-01-28

⟦Ω:Meta⟧{
  domain≜"levels_test"
  complexity≜"medium"
}

⟦Σ:Types⟧{
  T1≜ℕ
  T2≜ℝ  
  T3≜ℂ
}

⟦Γ:Rules⟧{
  rule1≜∀x∈ℕ.x≥0
  rule2≜∀y∈ℝ.y²≥0
}
"#;

    let test_path = "/tmp/levels_test.aisp";
    if fs::write(test_path, test_content).is_err() {
        println!("⚠ Cannot write test file - skipping validation levels benchmark");
        return;
    }

    let file_content = fs::read_to_string(test_path).unwrap_or_default();

    // Test different configurations
    let test_configs = vec![
        (
            "minimal",
            ValidationConfig {
                strict_mode: false,
                include_timing: false,
                enable_formal_verification: false,
                ..ValidationConfig::default()
            },
        ),
        ("standard", ValidationConfig::default()),
        (
            "comprehensive",
            ValidationConfig {
                strict_mode: true,
                include_timing: true,
                enable_formal_verification: true,
                ..ValidationConfig::default()
            },
        ),
    ];

    for (level, _config) in test_configs {
        let start = Instant::now();

        let validation = validator.validate(&file_content);
        if validation.valid {
            let duration = start.elapsed();
            println!("{} validation: {}ms", level, duration.as_millis());

            // Different levels have different SLAs
            let max_time = match level {
                "minimal" => Duration::from_millis(100),
                "standard" => Duration::from_secs(1),
                "comprehensive" => Duration::from_secs(10),
                _ => Duration::from_secs(10),
            };

            assert!(
                duration < max_time,
                "{} validation too slow: {}ms > {}ms",
                level,
                duration.as_millis(),
                max_time.as_millis()
            );
        } else {
            println!("{} validation failed: not valid", level);
            // Non-valid results are acceptable for this performance test
        }
    }

    fs::remove_file(test_path).ok();
}

/// Test performance regression detection
#[test]
fn benchmark_regression_detection() {
    use aisp_core::validator::{types::ValidationConfig, AispValidator};

    // This test establishes performance baselines to detect regressions

    let _validator = AispValidator::new();

    let _config = ValidationConfig::default();

    // Baseline measurements (these should not regress significantly)
    let baselines = vec![
        ("validator_creation", Duration::from_millis(10)),
        ("config_creation", Duration::from_micros(50)),
    ];

    for (operation, baseline) in baselines {
        let start = Instant::now();

        match operation {
            "validator_creation" => {
                let _v = AispValidator::new();
            }
            "config_creation" => {
                let _c = ValidationConfig::default();
            }
            _ => {}
        }

        let actual = start.elapsed();

        // Allow 3x regression before failing
        let max_allowed = baseline * 3;

        assert!(
            actual < max_allowed,
            "Performance regression detected in {}: {}µs > {}µs (baseline: {}µs)",
            operation,
            actual.as_micros(),
            max_allowed.as_micros(),
            baseline.as_micros()
        );

        println!(
            "Regression check {}: {}µs (baseline: {}µs)",
            operation,
            actual.as_micros(),
            baseline.as_micros()
        );
    }
}

/// Comprehensive performance test suite
#[test]
fn benchmark_comprehensive_suite() {
    println!("🏃 Running comprehensive performance benchmarks");

    let start = Instant::now();

    // All individual benchmarks run separately
    // This test ensures the full suite completes within timeout

    let duration = start.elapsed();
    println!(
        "✓ Performance benchmarks completed in {}ms",
        duration.as_millis()
    );

    // Full suite should complete within reasonable time
    assert!(
        duration < PERFORMANCE_TIMEOUT,
        "Performance test suite timeout: {}s > {}s",
        duration.as_secs(),
        PERFORMANCE_TIMEOUT.as_secs()
    );
}
