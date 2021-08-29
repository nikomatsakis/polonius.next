fn main() -> eyre::Result<()> {
    for arg in std::env::args().skip(1) {
        polonius::test_harness(&arg)?;
    }
    Ok(())
}
