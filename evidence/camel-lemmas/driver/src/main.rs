//! Acceptance test for the library path: run `FormalVerifier::verify` (the
//! library's Z3-backed path) on every lemma in a corpus directory, and judge each
//! result against `MANIFEST.tsv`. Usage: `camel-lemma-driver <lemmas-dir>`.
//! Exits 1 on any disagreement and 2 on a malformed corpus.
//!
//! The manifest, not the lemma document, holds each lemma's expected verdict and
//! partner, and the corpus checks match `corpus.sh`: every lemma is listed, every
//! partner is mutual and expects the opposite verdict, no document carries an
//! `expect≜`, and the two partners are byte-identical outside their `⟦Γ⟧` block.
//!
//! Verdict on this path:
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
//! This is stricter than `run.sh`'s verdict for the CLI. The CLI's JSON reports
//! only how many components passed and hides parse errors, so there "refuted"
//! means some component failed, for whatever reason.
//!
//! The pairing rule is shared with `run.sh`: a row agrees only if its verdict
//! matches the manifest AND differs from its partner's. `FormalVerifier` also
//! derives properties from `⟦Σ⟧` types and does not say which rule a property
//! came from, but partners are identical outside `⟦Γ⟧`. So any result that does
//! not depend on the rule's content gives both partners the same verdict and
//! fails both rows.
use aisp_core::{
    formal_verification::{FailureReason, FormalVerifier, VerificationFailure, VerificationStatus},
    parser::AispParser,
};
use std::collections::BTreeMap;
use std::path::Path;

struct Row {
    status: String,
    counts: String,
    verdict: &'static str,
}

fn verdict_name(v: &str) -> Option<&'static str> {
    match v {
        "proved" => Some("proved"),
        "refuted" => Some("refuted"),
        _ => None,
    }
}

/// The document with its `⟦Γ…⟧{ … }` block removed, as `corpus.sh` strips it.
fn strip_rules(src: &str) -> String {
    let mut out = String::new();
    let mut skip = false;
    for line in src.lines() {
        if line.starts_with("⟦Γ") {
            skip = true;
        }
        if !skip {
            out.push_str(line);
            out.push('\n');
        }
        if skip && line.starts_with('}') {
            skip = false;
        }
    }
    out
}

struct Entry {
    expected: &'static str,
    partner: String,
}

/// Loads `MANIFEST.tsv` and checks the corpus; exits 2 if it is malformed.
fn load_corpus(dir: &Path) -> BTreeMap<String, Entry> {
    let fail = |msg: String| -> ! {
        eprintln!("corpus: {msg}");
        eprintln!("corpus: malformed, no verdicts read");
        std::process::exit(2)
    };
    let manifest = std::fs::read_to_string(dir.join("MANIFEST.tsv"))
        .unwrap_or_else(|e| fail(format!("cannot read MANIFEST.tsv: {e}")));
    let mut entries = BTreeMap::new();
    for line in manifest
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
    {
        let cols: Vec<&str> = line.split('\t').collect();
        let [id, exp, partner] = cols[..] else {
            fail(format!("bad manifest line: {line}"))
        };
        let expected =
            verdict_name(exp).unwrap_or_else(|| fail(format!("{id}: bad verdict {exp}")));
        entries.insert(
            id.to_string(),
            Entry {
                expected,
                partner: partner.to_string(),
            },
        );
    }
    let read = |id: &str| {
        std::fs::read_to_string(dir.join(format!("{id}.aisp")))
            .unwrap_or_else(|e| fail(format!("{id}.aisp: {e}")))
    };
    if let Ok(listing) = std::fs::read_dir(dir) {
        for f in listing.flatten() {
            let p = f.path();
            if p.extension().is_some_and(|x| x == "aisp") {
                let id = p
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                if !entries.contains_key(&id) {
                    fail(format!("{id} is not in MANIFEST.tsv"));
                }
            }
        }
    }
    for (id, e) in &entries {
        let p = entries
            .get(&e.partner)
            .filter(|p| p.partner == *id)
            .unwrap_or_else(|| {
                fail(format!(
                    "{id}'s partner {} is missing or not mutual",
                    e.partner
                ))
            });
        if p.expected == e.expected {
            fail(format!("{id} and {} expect the same verdict", e.partner));
        }
        let (a, b) = (read(id), read(&e.partner));
        if a.contains("expect≜") {
            fail(format!("{id}.aisp carries its expected verdict"));
        }
        if strip_rules(&a) != strip_rules(&b) {
            fail(format!("{id} and {} differ outside ⟦Γ⟧", e.partner));
        }
    }
    entries
}

fn has_counterexample(failures: &[VerificationFailure]) -> bool {
    failures
        .iter()
        .any(|f| matches!(f.reason, FailureReason::Counterexample(_)))
}

fn judge(src: &str) -> Row {
    let row = |status: String, counts: String, verdict| Row {
        status,
        counts,
        verdict,
    };
    let parse = AispParser::new(src.to_string()).parse(src);
    let Some(doc) = parse.document else {
        return row("PARSE-FAIL".into(), "-".into(), "none");
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
        return row(status, "-".into(), "none");
    }
    let r = match FormalVerifier::new().verify(&doc) {
        Err(e) => return row(format!("ERROR: {e}"), "-".into(), "none"),
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
    row(label, counts, verdict)
}

fn main() {
    let Some(dir) = std::env::args().nth(1) else {
        eprintln!("usage: camel-lemma-driver <lemmas-dir>");
        std::process::exit(2);
    };
    let dir = Path::new(&dir);
    let corpus = load_corpus(dir);
    let rows: BTreeMap<&str, Row> = corpus
        .keys()
        .map(|id| {
            let src = std::fs::read_to_string(dir.join(format!("{id}.aisp"))).unwrap_or_default();
            (id.as_str(), judge(&src))
        })
        .collect();
    let mut disagreements = 0;
    println!(
        "| lemma | expected | status | checked/ok/failed | verdict | partner verdict | agrees |"
    );
    println!("|---|---|---|---|---|---|---|");
    for (id, r) in &rows {
        let e = &corpus[*id];
        let partner = rows[e.partner.as_str()].verdict;
        let agrees = r.verdict == e.expected && partner != r.verdict;
        if !agrees {
            disagreements += 1;
        }
        println!(
            "| {id} | {} | {} | {} | {} | {partner} | {} |",
            e.expected,
            r.status,
            r.counts,
            r.verdict,
            if agrees { "yes" } else { "**no**" }
        );
    }
    if disagreements > 0 {
        std::process::exit(1);
    }
}
