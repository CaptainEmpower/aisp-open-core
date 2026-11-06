//! Acceptance test for the library path: parse each lemma, run
//! `FormalVerifier::verify` (the library's Z3-backed path), and judge the result
//! against the lemma's `expect≜` verdict. Exits 1 on any disagreement.
//!
//! The criterion is the one `run.sh` applies to the CLI:
//!
//! - proved: status `Verified` with at least one property checked. A `Verified`
//!   over zero properties examined nothing and is not a proof.
//! - refuted: at least one failure whose reason is a counterexample, under
//!   `Failed` or `PartiallyVerified`.
//! - none: anything else (timeouts, unsupported properties,
//!   `ProofGenerationFailed`, any parse error, since the parser drops a block it
//!   cannot parse and carries on), matching the README's "Unknown counts
//!   as not refuted".
//!
//! A verdict counts only if the lemma's rule decided it. `FormalVerifier` also
//! derives properties from `⟦Σ⟧` types, and the result does not say which rule a
//! property came from. So each lemma is paired with the one that shares its
//! skeleton and differs only in the rule (`T_n` ↔ `F_n`, `C2` ↔ `C1`,
//! `C4` ↔ `C3`). A row agrees only if its verdict matches its expectation AND
//! differs from its partner's. A result that does not depend on the rule's content
//! gives both members the same verdict, and so fails both rows.
use aisp_core::{
    formal_verification::{FailureReason, FormalVerifier, VerificationFailure, VerificationStatus},
    parser::AispParser,
};
use std::collections::HashMap;

struct Row {
    name: String,
    expected: Option<&'static str>,
    status: String,
    counts: String,
    verdict: &'static str,
}

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

/// The lemma that shares this one's skeleton and differs only in its rule.
fn partner_prefix(name: &str) -> Option<String> {
    let rest = name.get(1..)?;
    match name.chars().next()? {
        'T' => Some(format!("F{rest}")),
        'F' => Some(format!("T{rest}")),
        'C' => {
            let p = match rest.chars().next()? {
                '1' => '2',
                '2' => '1',
                '3' => '4',
                '4' => '3',
                _ => return None,
            };
            // Controls have different suffixes (C1-contradiction, C2-tautology):
            // match on the "C<n>-" prefix only.
            Some(format!("C{p}-"))
        }
        _ => None,
    }
}

fn judge(path: &str) -> Row {
    let name = std::path::Path::new(path)
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string());
    let row = |expected, status: String, counts: String, verdict| Row {
        name: name.clone(),
        expected,
        status,
        counts,
        verdict,
    };
    let src = match std::fs::read_to_string(path) {
        Err(e) => return row(None, format!("READ-FAIL: {e}"), "-".into(), "none"),
        Ok(src) => src,
    };
    let exp = expected(&src);
    let parse = AispParser::new(src.clone()).parse(&src);
    let Some(doc) = parse.document else {
        return row(exp, "PARSE-FAIL".into(), "-".into(), "none");
    };
    // The parser recovers from a block it cannot parse by dropping it and
    // recording an error, so a document can come back with its ⟦Γ⟧ block gone.
    // Verifying the remnant would judge a document that no longer holds the
    // lemma, so any parse error is no verdict.
    if !parse.errors.is_empty() {
        let blocks: Vec<String> = doc
            .blocks
            .iter()
            .map(|b| {
                format!("{b:?}")
                    .split('(')
                    .next()
                    .unwrap_or("?")
                    .to_string()
            })
            .collect();
        let status = format!(
            "PARSE-ERRORS({}): kept {}",
            parse.errors.len(),
            blocks.join(",")
        );
        return row(exp, status, "-".into(), "none");
    }
    let r = match FormalVerifier::new().verify(&doc) {
        Err(e) => return row(exp, format!("ERROR: {e}"), "-".into(), "none"),
        Ok(r) => r,
    };
    let st = &r.statistics;
    let counts = format!(
        "{}/{}/{}",
        st.properties_checked, st.successful_verifications, st.failed_verifications
    );
    let (label, verdict) = match &r.status {
        VerificationStatus::Verified if st.properties_checked > 0 => {
            ("Verified".to_string(), "proved")
        }
        VerificationStatus::Verified => ("Verified (nothing checked)".to_string(), "none"),
        VerificationStatus::Failed(fs)
        | VerificationStatus::PartiallyVerified { failures: fs, .. }
            if has_counterexample(fs) =>
        {
            ("counterexample".to_string(), "refuted")
        }
        other => (format!("{other:?}").chars().take(60).collect(), "none"),
    };
    row(exp, label, counts, verdict)
}

fn main() {
    let rows: Vec<Row> = std::env::args().skip(1).map(|p| judge(&p)).collect();
    let verdicts: HashMap<&str, &str> = rows.iter().map(|r| (r.name.as_str(), r.verdict)).collect();
    let mut disagreements = 0;
    println!(
        "| lemma | expected | status | checked/ok/failed | verdict | partner verdict | agrees |"
    );
    println!("|---|---|---|---|---|---|---|");
    for r in &rows {
        let partner = partner_prefix(&r.name).and_then(|prefix| {
            verdicts
                .iter()
                .find(|(n, _)| {
                    if prefix.ends_with('-') {
                        n.starts_with(&prefix)
                    } else {
                        **n == prefix
                    }
                })
                .map(|(_, v)| *v)
        });
        let agrees = r.expected == Some(r.verdict) && partner.is_some_and(|p| p != r.verdict);
        if !agrees {
            disagreements += 1;
        }
        println!(
            "| {} | {} | {} | {} | {} | {} | {} |",
            r.name,
            r.expected.unwrap_or("missing"),
            r.status,
            r.counts,
            r.verdict,
            partner.unwrap_or("missing"),
            if agrees { "yes" } else { "**no**" }
        );
    }
    if disagreements > 0 {
        std::process::exit(1);
    }
}
