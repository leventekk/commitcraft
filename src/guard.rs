use crate::{
	executor::Executor,
	prelude::{Error, Result},
};

pub struct Guard {}

impl Guard {
	pub fn check_requirements(api_key: &str) -> Result<()> {
		if api_key.is_empty() {
			return Err(Error::Guard("API key is not set".to_string()));
		}

		if Guard::check_git_status().is_err() {
			return Err(Error::Guard("Not a git repository".to_string()));
		}

		Ok(())
	}

	fn check_git_status() -> Result<String> {
		Executor::execute(vec!["status"])
	}
}
