use polonius;

#[test]
fn test() -> eyre::Result<()> {
    polonius::test_harness("tests/example-a")
}
