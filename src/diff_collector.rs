use std::{error, process::Command};

// anyhow crate is better for handling errors
pub fn collect_changes() -> Result<String, Box<dyn error::Error>> {
	let arguments = vec![
		"--no-pager", // disable custom
		"diff",
		"--staged",              // check only staged changes
		"--ignore-space-change", // ignore changes in whitespace
		"--ignore-blank-lines",  // ignore blank lines
	];

    // TODO: Might be worth to consider a factory that can execute git commands
	let output = Command::new("git").args(&arguments).output()?;
    let response = String::from_utf8(output.stdout)?;

	Ok(response)
}
