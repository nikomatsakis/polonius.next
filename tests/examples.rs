use polonius;

#[test]
fn example_a() -> eyre::Result<()> {
    polonius::test_harness("tests/example-a")
}
