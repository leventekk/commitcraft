use crate::prelude::*;
use std::process::Command;

pub struct Executor {}

impl Executor {
	pub fn execute(arguments: Vec<&str>) -> Result<String> {
		let output = Command::new("git").args(&arguments).output()?;
		let err_convert =
			Error::Executor("Could not convert output to string".to_string());

        // if there is an error while executing the command, we need to print the error message.
        // for example: if the the pre-commit hook found some errors.
		if !output.status.success() {
			let error_message =
				String::from_utf8(output.stderr).map_err(|_| err_convert)?;

			return Err(Error::Executor(error_message));
		}

		let response =
			String::from_utf8(output.stdout).map_err(|_| err_convert)?;

		Ok(response)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_execute() {
		let arguments = vec!["--version"];
		let response = Executor::execute(arguments).unwrap();
		assert!(response.contains("git version"));
	}

    #[test]
    fn test_execute_error() {
        let arguments = vec!["missing command"];
        let response = Executor::execute(arguments);

        assert!(response.is_err());
    }
}
