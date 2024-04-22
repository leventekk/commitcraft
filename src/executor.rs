use std::process::Command;

pub struct Executor {}

impl Executor {
	pub fn confirm_message(message: &str) {
		let arguments = vec!["--no-pager", "commit", "-m", message];

		let output = match Command::new("git").args(&arguments).output() {
			Ok(r) => r,
			Err(..) => panic!("Error executing git command."),
		};

		let response = match String::from_utf8(output.stdout) {
			Ok(r) => r,
			Err(..) => panic!("Error parsing git response."),
		};

		println!("{}", response);
	}
}
