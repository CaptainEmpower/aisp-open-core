//! Acceptance test for the library path: parse each lemma, run
//! `FormalVerifier::verify` (the library's Z3-backed path), and judge the result
//! against the lemma's `expect≜` verdict. Exits 1 on any disagreement.
//!
//! - proved: status `Verified` with at least one property checked. A `Verified`
//!   over zero properties examined nothing and is not a proof.
//! - refuted: at least one failure whose reason is a counterexample.
//!
//! Anything else (timeouts, unsupported properties, `ProofGenerationFailed`,
//! parse errors) is not a verdict, matching the README's "Unknown counts as not
//! refuted".
use aisp_core::{
    formal_verification::{FailureReason, FormalVerifier, VerificationFailure, VerificationStatus},
    parser::AispParser,
};

fn expected(src: &str) -> Option<&'static str> {
    src.lines()
        .find_map(|l| l.trim().strip_prefix("expect≜"))
        .and_then(|v| match v.trim() {
            "proved" => Some("proved"),
            "refuted" => Some("refuted"),
            _ => None,
        })
}

fn has_counterexample(failures: &[VerificationFailure]) -> bool {
    failures
        .iter()
        .any(|f| matches!(f.reason, FailureReason::Counterexample(_)))
}

fn main() {
    let mut disagreements = 0;
    println!("| lemma | expected | status | checked/ok/failed | verdict | agrees |");
    println!("|---|---|---|---|---|---|");
    for path in std::env::args().skip(1) {
        let name = std::path::Path::new(&path)
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| path.clone());
        let (exp, status, counts, verdict) = match std::fs::read_to_string(&path) {
            Err(e) => (None, format!("READ-FAIL: {e}"), "-".into(), "none"),
            Ok(src) => {
                let exp = expected(&src);
                let parse = AispParser::new(src.clone()).parse(&src);
                match parse.document {
                    None => (exp, "PARSE-FAIL".into(), "-".into(), "none"),
                    Some(doc) => match FormalVerifier::new().verify(&doc) {
                        Err(e) => (exp, format!("ERROR: {e}"), "-".into(), "none"),
                        Ok(r) => {
                            let st = &r.statistics;
                            let counts = format!(
                                "{}/{}/{}",
                                st.properties_checked,
                                st.successful_verifications,
                                st.failed_verifications
                            );
                            let (label, verdict) = match &r.status {
                                VerificationStatus::Verified if st.properties_checked > 0 => {
                                    ("Verified".to_string(), "proved")
                                }
                                VerificationStatus::Verified => {
                                    ("Verified (nothing checked)".to_string(), "none")
                                }
                                VerificationStatus::Failed(fs)
                                | VerificationStatus::PartiallyVerified { failures: fs, .. }
                                    if has_counterexample(fs) =>
                                {
                                    ("counterexample".to_string(), "refuted")
                                }
                                other => (format!("{other:?}").chars().take(60).collect(), "none"),
                            };
                            (exp, label, counts, verdict)
                        }
                    },
                }
            }
        };
        let agrees = exp == Some(verdict);
        if !agrees {
            disagreements += 1;
        }
        println!(
            "| {name} | {} | {status} | {counts} | {verdict} | {} |",
            exp.unwrap_or("missing"),
            if agrees { "yes" } else { "**no**" }
        );
    }
    if disagreements > 0 {
        std::process::exit(1);
    }
}
