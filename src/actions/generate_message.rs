use std::process;
use std::time::Duration;

use crate::{
	actions::config::Format,
	backup::Backup,
	collector::Collector,
	executor::Executor,
	generator::Generator,
	guard::Guard,
	instruction_builder::InstructionBuilder,
	instructions::{
		conventional::ConventionalCommitInstructionStrategy,
		raw::RawCommitInstructionStrategy, InstructionStrategy,
	},
	prelude::Result,
};
use console::style;
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
			eprintln!("{} {}", style("error").red().bold(), error);
			process::exit(1);
		}

		let collected_changes = Collector::collect_changes()?;

		if collected_changes.diff.is_empty() {
			println!(
				"{} No stashed files were found.",
				style("info").blue().bold()
			);

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

		println!(
			"{}\n{:}",
			style("Files found in the stash").bold().dim(),
			style(formatted_list).dim()
		);

		let message_instructions: String = match format {
			Format::Conventional => {
				ConventionalCommitInstructionStrategy::inject(with_description)
			}
			Format::Raw => RawCommitInstructionStrategy::inject(with_description),
		};

		let recovered_message = Backup::recover()?;

		if !recovered_message.is_empty() {
			println!(
				"{} Seems like there is a recovered message:\n{}",
				style("info").green().bold(),
				recovered_message
			);

			let message_confirmed =
				Confirm::new("Do you want to reuse this message?")
					.with_default(true)
					.prompt();

			if let Ok(true) = message_confirmed {
				execute_commit(recovered_message.as_str());

				return Ok(());
			}

			if let Ok(false) = message_confirmed {
				Backup::destroy()?;
			}
		}

		let progress_bar =
			ProgressBar::new_spinner().with_message("Generating commit message");

		progress_bar.enable_steady_tick(Duration::from_millis(120));

		let generated_message = match Generator::generate_message(
			openai_api_key,
			&collected_changes.diff,
			InstructionBuilder::build(
				message_instructions.as_str(),
				with_description,
			)
			.as_str(),
		)
		.await
		{
			Ok(m) => m,
			Err(message) => {
				println!(
					"{} Failed to generate commit message: {}",
					style("error").red().bold(),
                    message
				);
				process::exit(1);
			}
		};

		progress_bar.finish_and_clear();

		println!(
			"{} Here is the generated message:\n{}",
			style("info").green().bold(),
			generated_message
		);

		let message_confirmed = Confirm::new("Do you want to use this message?")
			.with_default(true)
			.prompt();

		match message_confirmed {
			Ok(true) => {
				execute_commit(generated_message.as_str());
			}
			Ok(false) => {}
			Err(_) => {}
		}

		Ok(())
	}
}

fn execute_commit(message: &str) {
	let commit_response = commit_changes(message);

	match commit_response {
		Ok(message) => {
			let _ = Backup::destroy();
			println!("{} {}", style("success").green().bold(), message)
		}
		Err(error) => {
			let commit = Backup::commit(message);

			if commit.is_err() {
				eprintln!("{} Failed to crate a backup", style("error").red().bold(),)
			}

			eprintln!("{} {}", style("error").red().bold(), error)
		}
	}
}

fn commit_changes(message: &str) -> Result<String> {
	Executor::execute(vec!["--no-pager", "commit", "-m", message])
}
