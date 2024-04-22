use clap::{Parser, Subcommand, ValueEnum};
use instruction_builder::InstructionBuilder;
use instructions::InstructionStrategy;
use serde_derive::{Deserialize, Serialize};

mod diff_collector;
mod instruction_builder;
mod instructions;
mod message_generator;

static APP_NAME: &str = "commitcraft";

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
	/// does testing things
	Config {
		/// lists test values
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
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for AppConfig {
	fn default() -> Self {
		Self {
			openai_api_key: "".into(),
		}
	}
}

#[tokio::main]
async fn main() -> Result<(), confy::ConfyError> {
	let args = Args::parse();
	let cfg: AppConfig = confy::load(APP_NAME, None)?;

	match &args.command {
		Some(Commands::Config { api_key }) => {
			if !api_key.is_empty() {
				confy::store(
					APP_NAME,
					None,
					AppConfig {
						openai_api_key: api_key.clone(),
					},
				)?;

				println!("Configuration saved.")
			}
		}
		None => {
			if cfg.openai_api_key.is_empty() {
				eprintln!("OpenAI API key is not set.");
				std::process::exit(1);
			}

			let instructions_injector: Box<dyn InstructionStrategy> = match args.format {
                Format::Conventional => Box::new(
                    instructions::conventional::ConventionalCommitInstructionStrategy,
                ),
                Format::Raw => Box::new(instructions::raw::RawCommitInstructionStrategy),
            };

			let collected_changed = match diff_collector::collect_changes() {
				Ok(r) => r,
				Err(e) => {
					eprintln!("No files were found: {}", e);
					std::process::exit(1);
				}
			};

			if collected_changed.is_empty() {
				eprintln!("No files were found.");
				std::process::exit(1);
			}

			let generated_message = message_generator::generate_message(
				&cfg.openai_api_key,
				&collected_changed,
				// TODO: this builder needs a constructor where I can inject the instructions
				InstructionBuilder::build(instructions_injector),
			)
			.await;

			println!("{:}", generated_message);
		}
	}

	Ok(())
}
