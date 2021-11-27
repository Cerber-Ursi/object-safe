use trybuild::TestCases;

#[test]
fn compile_pass() {
    let t = TestCases::new();
    t.pass("tests/compile_pass/*.rs");
}
