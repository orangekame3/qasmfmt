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

// =============================================================================
// Integration Tests
// =============================================================================

#[test]
fn test_basic() {
    run_test("basic");
}

#[test]
fn test_minified() {
    run_test("minified");
}

// =============================================================================
// Rule-based Tests
// See: docs/src/rules/overview.md
// =============================================================================

#[test]
fn test_version_declaration() {
    run_test("version_declaration");
}

#[test]
fn test_include_statement() {
    run_test("include_statement");
}

#[test]
fn test_declarations() {
    run_test("declarations");
}

#[test]
fn test_gate_calls() {
    run_test("gate_calls");
}

#[test]
fn test_gate_definition() {
    run_test("gate_definition");
}

#[test]
fn test_assignment() {
    run_test("assignment");
}

#[test]
fn test_barrier() {
    run_test("barrier");
}
