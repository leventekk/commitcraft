use std::time::Duration;

use clap::{Parser, Subcommand, ValueEnum};
use executor::Executor;
use indicatif::ProgressBar;
use inquire::Confirm;
use instruction_builder::InstructionBuilder;
use instructions::InstructionStrategy;
use serde_derive::{Deserialize, Serialize};
use std::fmt::Write;

mod diff_collector;
mod executor;
mod instruction_builder;
mod instructions;
mod message_generator;

const APP_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format {
	/// Conventional commit message
	Conventional,

	/// Raw commit message
	Raw,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
	/// Format of the commit message
	#[arg(short, long, value_enum, default_value = "conventional")]
	format: Format,

	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
	/// Update configuration options
	Config {
		/// set api key for the OpenAI API
		#[arg(short, long)]
		api_key: String,
	},
}

// TODO: this should be a separate file
// - other possible options can be:
// - language
// - format
#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
	openai_api_key: String,
	// add_description: bool,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for AppConfig {
	fn default() -> Self {
		Self {
			openai_api_key: "".into(),
			// add_description: true,
		}
	}
}

#[tokio::main]
async fn main() -> Result<(), confy::ConfyError> {
	let args = Args::parse();
	let app_config: AppConfig = confy::load(APP_NAME, None)?;

	match &args.command {
		Some(Commands::Config { api_key }) => {
			let mut config = AppConfig { ..app_config };

			if !api_key.is_empty() {
				config.openai_api_key = api_key.clone();
			}

			confy::store(APP_NAME, None, config)?;
			println!("Configuration saved.")
		}
		None => {
			if app_config.openai_api_key.is_empty() {
				eprintln!("OpenAI API key is not set.");
				std::process::exit(1);
			}

			let instructions_injector: Box<dyn InstructionStrategy> = match args.format {
                Format::Conventional => Box::new(
                    instructions::conventional::ConventionalCommitInstructionStrategy,
                ),
                Format::Raw => Box::new(instructions::raw::RawCommitInstructionStrategy),
            };

			let changes = match diff_collector::collect_changes() {
				Ok(r) => r,
				Err(e) => {
					eprintln!("No stashed changes were found: {}", e);
					std::process::exit(1);
				}
			};

			if changes.files.is_empty() {
				eprintln!("No stashed changes were found");
				std::process::exit(1);
			}

			let formatted_list = changes
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

			let generated_message = message_generator::generate_message(
				&app_config.openai_api_key,
				&changes.diff,
				// TODO: this builder needs a constructor where I can inject the instructions
				InstructionBuilder::build(instructions_injector),
			)
			.await;

			progress_bar.finish_and_clear();

			println!("Here is the generated commit:\n\n{:}\n", generated_message);

			let message_confirmed = Confirm::new("Do you want to use this message?")
				.with_default(true)
				.prompt();

			match message_confirmed {
				Ok(true) => Executor::confirm_message(&generated_message),
				Ok(false) => {
					println!("That's too bad, I've heard great things about it.")
				}
				Err(_) => println!("Error with questionnaire, try again later"),
			}
		}
	}

	Ok(())
}
