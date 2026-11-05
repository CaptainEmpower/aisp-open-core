//! Parse each lemma file and run `FormalVerifier::verify`, the library's
//! Z3-backed path, then print what it said about each property.
use aisp_core::{formal_verification::FormalVerifier, parser::AispParser};

fn main() {
    for path in std::env::args().skip(1) {
        let src = std::fs::read_to_string(&path).expect("read");
        let parse = AispParser::new(src.clone()).parse(&src);
        let name = std::path::Path::new(&path).file_stem().unwrap().to_string_lossy().to_string();
        let Some(doc) = parse.document else {
            println!("{name}\tPARSE-FAIL\t{:?}", parse.errors);
            continue;
        };
        match FormalVerifier::new().verify(&doc) {
            Err(e) => println!("{name}\tERROR\t{e}"),
            Ok(r) => {
                let status = match &r.status {
                    aisp_core::formal_verification::VerificationStatus::Verified => "Verified".to_string(),
                    other => format!("{other:?}"),
                };
                let invs: Vec<String> = r
                    .verified_invariants
                    .iter()
                    .map(|v| format!("{}", v.invariant.name))
                    .collect();
                println!(
                    "{name}\t{status}\tchecked={} ok={} failed={}\tinvariants={:?}",
                    r.statistics.properties_checked,
                    r.statistics.successful_verifications,
                    r.statistics.failed_verifications,
                    invs
                );
            }
        }
    }
}
