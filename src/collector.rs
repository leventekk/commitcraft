use crate::prelude::*;

use crate::executor::Executor;

#[derive(Debug)]
pub struct CollectedChanges {
	pub files: Vec<String>,
	pub diff: String,
}

pub struct Collector {}

impl Collector {
	pub fn collect_changes() -> Result<CollectedChanges> {
		Ok(CollectedChanges {
			files: collec_changed_files()?,
			diff: collect_diff()?,
		})
	}
}

fn collect_diff() -> Result<String> {
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

fn collec_changed_files() -> Result<Vec<String>> {
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
		.filter_map(|line| line.split_whitespace().last().map(|v| v.to_string()))
		.collect::<Vec<_>>();

	Ok(lines)
}
