#[test]
fn capture_id_derives_have_stable_diagnostics() {
    let tests = trybuild::TestCases::new();
    tests.pass("tests/ui/capture_ids_pass.rs");
    tests.compile_fail("tests/ui/capture_id_path.rs");
    tests.compile_fail("tests/ui/capture_id_duplicate.rs");
    tests.compile_fail("tests/ui/capture_route_size.rs");
    tests.compile_fail("tests/ui/capture_scenario_diagnostics.rs");
    tests.compile_fail("tests/ui/capture_route_diagnostics.rs");
}
