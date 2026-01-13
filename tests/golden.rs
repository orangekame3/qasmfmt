use std::fs;
use std::path::Path;

use qasmfmt::format;

fn run_test(name: &str) {
    let testdata = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name);

    let input = fs::read_to_string(testdata.join("input.qasm"))
        .unwrap_or_else(|e| panic!("{}/input.qasm: {}", name, e));
    let expected = fs::read_to_string(testdata.join("output.qasm"))
        .unwrap_or_else(|e| panic!("{}/output.qasm: {}", name, e));

    let actual = format(&input).unwrap_or_else(|e| panic!("{}: {}", name, e));

    if actual != expected {
        panic!(
            "\n\n=== {} ===\n\nExpected:\n{}\n\nActual:\n{}\n",
            name, expected, actual
        );
    }
}

#[test]
fn test_basic() {
    run_test("basic");
}

#[test]
fn test_gate_def() {
    run_test("gate_def");
}
