// Build script for Pest grammar compilation

fn main() {
    // Tell cargo to rerun if the grammar file changes
    println!("cargo:rerun-if-changed=src/grammar/aisp.pest");

    // The pest_derive crate will automatically handle grammar compilation
    // when we use #[grammar = "grammar/aisp.pest"] in the parser
}
