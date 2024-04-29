use std::process;

use clap::{Parser, Subcommand, ValueEnum};
use commitcraft::{
	actions::generate_message::generate_message,
	executor::Executor,
	guard::Guard,
	instructions::{
		conventional::ConventionalCommitInstructionStrategy,
		raw::RawCommitInstructionStrategy, InstructionStrategy,
	},
	prelude::Result,
};
use inquire::Confirm;
use serde_derive::{Deserialize, Serialize};

const APP_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Format {
	/// Conventional commit message
	Conventional,

	/// Raw commit message
	Raw,
}

#[derive(Parser)]
#[command(version, about, arg_required_else_help = true)]
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

	/// Generate a commit message
	Generate,
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

impl ::std::default::Default for AppConfig {
	fn default() -> Self {
		Self {
			openai_api_key: "".into(),
			// add_description: true,
		}
	}
}

#[tokio::main]
async fn main() -> Result<()> {
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
		Some(Commands::Generate) => {
			let requirements_check =
				Guard::check_requirements(&app_config.openai_api_key);

			if let Err(e) = requirements_check {
				eprintln!("{}", e);
				process::exit(1);
			}

			let message_instructions: Box<dyn InstructionStrategy> =
				match args.format {
					Format::Conventional => {
						Box::new(ConventionalCommitInstructionStrategy)
					}
					Format::Raw => Box::new(RawCommitInstructionStrategy),
				};

			let generated_message = generate_message(
				&app_config.openai_api_key,
				message_instructions.inject(),
			)
			.await;

			let message_confirmed = Confirm::new("Do you want to use this message?")
				.with_default(true)
				.prompt();

			match message_confirmed {
				Ok(true) => {
					let result = commit_changes(generated_message.as_str())?;

					println!("{}", result);
				}
				Ok(false) => {}
				Err(_) => {}
			}
		}
		None => {}
	}

	Ok(())
}

fn commit_changes(message: &str) -> Result<String> {
	Executor::execute(vec!["--no-pager", "commit", "-m", message])
}
