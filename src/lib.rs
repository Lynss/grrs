use exitfailure::ExitFailure;
use failure::ResultExt;
use std::io::Write;

pub fn find_matches(
    content: &str,
    pattern: &str,
    mut writer: impl Write,
) -> Result<(), ExitFailure> {
    for line in content.lines().filter(|line| line.contains(pattern)) {
        writeln!(writer, "{}", line)
            .with_context(|_| format!("failed to write lines into writer"))?;
    }
    Ok(())
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    let _ = find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
