use std::time::Duration;

use crate::{
	collector::Collector, generator::Generator,
	instruction_builder::InstructionBuilder,
};
use indicatif::ProgressBar;
use std::fmt::Write;

pub async fn generate_message(
	openai_api_key: &str,
	message_instructions: &str,
) -> String {
	let collected_changes = match Collector::collect_changes() {
		Ok(r) => r,
		Err(e) => {
			eprintln!("No stashed changes were found: {}", e);
			std::process::exit(1);
		}
	};

	if collected_changes.files.is_empty() {
		eprintln!("No stashed changes were found");
		std::process::exit(1);
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

	println!("Stashed files:\n{:}", formatted_list);

	let progress_bar =
		ProgressBar::new_spinner().with_message("Generating commit message");
	progress_bar.enable_steady_tick(Duration::from_millis(120));

	let generated_message = match Generator::generate_message(
		openai_api_key,
		&collected_changes.diff,
		InstructionBuilder::build(message_instructions).as_str(),
	)
	.await
	{
		Some(m) => m,
		None => {
			eprintln!("Failed to generate commit message");
			std::process::exit(1);
		}
	};

	progress_bar.finish_and_clear();

	println!("Here is the generated commit:\n\n{:}\n", generated_message);

    generated_message
}
