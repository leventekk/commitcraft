use std::error;
use std::process::Command;

pub struct Executor {}

impl Executor {
	pub fn confirm_message(message: &str) -> Result<String, Box<dyn error::Error>> {
		let arguments = vec!["--no-pager", "commit", "-m", message];

		self::Executor::execute(arguments)
	}

	pub fn execute(arguments: Vec<&str>) -> Result<String, Box<dyn error::Error>> {
		let output = Command::new("git").args(&arguments).output()?;
		let response = String::from_utf8(output.stdout)?;

		Ok(response)
	}
}
