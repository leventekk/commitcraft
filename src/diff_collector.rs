use std::error;

use crate::executor::Executor;

#[derive(Debug)]
pub struct CollectedChanges {
	pub files: Vec<String>,
	pub diff: String,
}

fn get_diff_changes() -> Result<String, Box<dyn error::Error>> {
	let arguments = vec![
		"--no-pager",
		"diff",
		"--staged",
		"--ignore-space-change",
		"--ignore-blank-lines",
	];
	let response = Executor::execute(arguments)?;

	Ok(response)
}

fn get_changed_files() -> Result<Vec<String>, Box<dyn error::Error>> {
	let arguments = vec![
		"--no-pager",
		"diff",
		"--staged",
		"--ignore-space-change",
		"--ignore-blank-lines",
		"--name-only",
	];
	let response = Executor::execute(arguments)?;

	let lines = response
		.lines()
		.map(|line| match line.split_whitespace().last() {
			Some(v) => v.to_string(),
			None => "".to_string(),
		})
		.collect::<Vec<_>>();

	Ok(lines)
}

pub fn collect_changes() -> Result<CollectedChanges, Box<dyn error::Error>> {
	// TODO: Might be worth to consider a factory that can execute git commands
	let git_diff = get_diff_changes()?;
	let changed_files = get_changed_files()?;

	Ok(CollectedChanges {
		files: changed_files,
		diff: git_diff,
	})
}
