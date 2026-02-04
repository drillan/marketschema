//! Compile-fail tests to verify type safety constraints.
//!
//! These tests use trybuild to verify that certain code patterns
//! fail to compile, documenting type safety guarantees as "living documentation".

#[test]
fn compile_fail_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
