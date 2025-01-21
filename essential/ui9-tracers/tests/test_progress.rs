use anyhow::Result;
use uiio_element::Progress;

#[test]
fn test_progress() -> Result<()> {
    let fqn = "app.progress".parse()?;
    let mut progress = Progress::new(fqn, 100);
    for value in 0..=100 {
        progress.set_value(value);
    }
    Ok(())
}
