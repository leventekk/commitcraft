use crate::prelude::*;

use std::process::Command;

pub struct Executor {}

impl Executor {
	pub fn execute(arguments: Vec<&str>) -> Result<String> {
		let output = Command::new("git").args(&arguments).output()?;
		let convert_error =
			Error::Executor("Could not convert output to string".to_string());

		if !output.status.success() {
			let error_message =
				String::from_utf8(output.stderr).map_err(|_| convert_error)?;

			return Err(Error::Executor(error_message));
		}

		let response =
			String::from_utf8(output.stdout).map_err(|_| convert_error)?;

		Ok(response)
	}
}
