use polonius;

#[test]
fn example_a() -> eyre::Result<()> {
    polonius::test_harness("tests/example-a")
}

#[test]
fn issue_47680() -> eyre::Result<()> {
    polonius::test_harness("tests/issue-47680")
}
