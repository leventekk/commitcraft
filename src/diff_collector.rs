use std::{error, process::Command};

#[derive(Debug)]
pub struct CollectedChanges {
	pub files: Vec<String>,
	pub diff: String,
}

fn execute(arguments: Vec<&str>) -> Result<String, Box<dyn error::Error>> {
	let output = Command::new("git").args(&arguments).output()?;
	let response = String::from_utf8(output.stdout)?;

	Ok(response)
}

fn get_diff_changes() -> Result<String, Box<dyn error::Error>> {
	let arguments = vec![
		"--no-pager",
		"diff",
		"--staged",
		"--ignore-space-change",
		"--ignore-blank-lines",
	];
	let response = execute(arguments)?;

	Ok(response)
}

fn get_changed_files() -> Result<Vec<String>, Box<dyn error::Error>> {
	let arguments = vec![
		"--no-pager",
		"diff",
		"--staged",
		"--ignore-space-change",
		"--ignore-blank-lines",
        "--name-only"
	];
	let response = execute(arguments)?;

	let lines = response
		.lines()
		.map(|line| match line.split_whitespace().last() {
			Some(v) => v.to_string(),
			None => "".to_string(),
		})
		.collect::<Vec<_>>();

	Ok(lines)
}

// anyhow crate is better for handling errors
pub fn collect_changes() -> Result<CollectedChanges, Box<dyn error::Error>> {
	// TODO: Might be worth to consider a factory that can execute git commands
	let git_diff = get_diff_changes()?;
	let changed_files = get_changed_files()?;

	Ok(CollectedChanges {
		files: changed_files,
		diff: git_diff,
	})
}
