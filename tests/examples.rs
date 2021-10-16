use polonius;

#[test]
fn example_a() -> eyre::Result<()> {
    polonius::test_harness("tests/example-a")
}

#[test]
fn issue_47680() -> eyre::Result<()> {
    polonius::test_harness("tests/issue-47680")
}

#[test]
fn vec_temp() -> eyre::Result<()> {
    polonius::test_harness("tests/vec-temp")
}

#[test]
fn canonical_liveness() -> eyre::Result<()> {
    polonius::test_harness("tests/canonical-liveness")
}

#[test]
fn canonical_liveness_err() -> eyre::Result<()> {
    polonius::test_harness("tests/canonical-liveness-err")
}

#[test]
fn killing_and_murder() -> eyre::Result<()> {
    polonius::test_harness("tests/killing-and-murder")
}

#[test]
fn killing_and_murder_err() -> eyre::Result<()> {
    polonius::test_harness("tests/killing-and-murder-err")
}

#[test]
fn self_invalidation_loop() -> eyre::Result<()> {
    polonius::test_harness("tests/self-invalidation-loop")
}

#[test]
fn self_invalidation_loop_shared() -> eyre::Result<()> {
    polonius::test_harness("tests/self-invalidation-loop-shared")
}

#[test]
fn diamond_ref_mod() -> eyre::Result<()> {
    polonius::test_harness("tests/diamond-ref-mod")
}
