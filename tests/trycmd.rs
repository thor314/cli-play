#[test]
fn trycmd() {
    trycmd::TestCases::new()
        .case("README.md")
        .case("tests/cmd/**/*.toml")
        // .insert_var("[REPLACEMENT]", "runtime-value")
        // .unwrap();
        //         .fail("tests/cmd/buggy-case.toml");
        ;
}