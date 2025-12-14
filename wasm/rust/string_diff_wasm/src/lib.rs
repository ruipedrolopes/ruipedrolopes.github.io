use wasm_bindgen::prelude::*;
use similar::{TextDiff, Algorithm};

#[wasm_bindgen]
pub fn diff_unified(a: &str, b: &str, a_label: &str, b_label: &str) -> String {
    let diff = TextDiff::configure()
        .algorithm(Algorithm::Myers)
        .diff_lines(a, b);

    let mut output = String::new();
    for change in diff.unified_diff().header(a_label, b_label).to_string().lines() {
        output.push_str(change);
        output.push('\n');
    }

    output
}