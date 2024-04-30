use std::process;
use std::time::Duration;

use crate::{
	actions::config::Format,
	collector::Collector,
	executor::Executor,
	generator::Generator,
	guard::Guard,
	instruction_builder::InstructionBuilder,
	instructions::{
		conventional::ConventionalCommitInstructionStrategy,
		raw::RawCommitInstructionStrategy, InstructionStrategy,
	},
	prelude::{Error, Result},
};
use colored::Colorize;
use indicatif::ProgressBar;
use inquire::Confirm;
use std::fmt::Write;

pub struct GenerateMessageCommand {}

impl GenerateMessageCommand {
	pub async fn generate_message(
		openai_api_key: &str,
        with_description: &bool,
		format: &Format,
	) -> Result<()> {
		let requirements_check = Guard::check_requirements(openai_api_key);

		if let Err(error) = requirements_check {
			eprintln!("{}", error.to_string().red());
			process::exit(1);
		}

		let collected_changes = Collector::collect_changes()?;

		if collected_changes.files.is_empty() {
			// return Ok(MessageResponse {
			// 	files: None,
			// 	message: "No stashed changes were found".to_string().yellow(),
			// });

			println!("{}", "No stashed changes were found".yellow());

			process::exit(1);
		}

		let formatted_list = collected_changes
			.files
			.iter()
			.map(|line| {
				let mut formatted_line = String::new();
				writeln!(&mut formatted_line, "- {}", line).unwrap();
				formatted_line
			})
			.collect::<String>();

		println!("Files found in the stash:\n{:}", formatted_list);

		let progress_bar =
			ProgressBar::new_spinner().with_message("Generating commit message");
		progress_bar.enable_steady_tick(Duration::from_millis(120));

		let message_instructions: &str = match format {
			Format::Conventional => ConventionalCommitInstructionStrategy::inject(),
			Format::Raw => RawCommitInstructionStrategy::inject(),
		};

		let generated_message = match Generator::generate_message(
			openai_api_key,
			&collected_changes.diff,
			InstructionBuilder::build(message_instructions, with_description).as_str(),
		)
		.await
		{
			Some(m) => m,
			None => {
				return Err(Error::CommitMessage(
					"Failed to generate commit message".to_string(),
				));
			}
		};

		progress_bar.finish_and_clear();

		println!("Here is the generated commit message:\n\n{:}\n", generated_message);

		let message_confirmed = Confirm::new("Do you want to use this message?")
			.with_default(true)
			.prompt();

		match message_confirmed {
			Ok(true) => {
				let commit_response = commit_changes(generated_message.as_str())?;

                println!("{}", commit_response);
			}
			Ok(false) => {}
			Err(_) => {}
		}

		Ok(())
	}
}

fn commit_changes(message: &str) -> Result<String> {
	Executor::execute(vec!["--no-pager", "commit", "-m", message])
}
