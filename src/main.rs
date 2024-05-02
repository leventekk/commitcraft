use clap::{Parser, Subcommand};
use commitcraft::{
	actions::{
		config::{AppConfig, ConfigCommand, ConfigOptions, Format},
		generate_message::GenerateMessageCommand,
	},
	prelude::Result,
};

#[derive(Parser)]
#[command(version, about = "A commit generator CLI", long_about = None, arg_required_else_help = true)]
struct Cli {
	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
	/// Update configuration options
	Config {
		/// set api key for the OpenAI API
		#[arg(long)]
		api_key: Option<String>,

		/// add description to the commit message
		#[arg(long)]
		add_description: Option<bool>,
	},

	/// Generate a commit message
	Generate {
		/// Format of the commit message
		#[arg(short, long, value_enum, default_value = "conventional")]
		format: Format,
	},
}

#[tokio::main]
async fn main() -> Result<()> {
	let app_config: AppConfig = ConfigCommand::load()?;
	let args = Cli::parse();

	match args.command {
		Some(Commands::Config {
			api_key,
			add_description,
		}) => {
			let _ = ConfigCommand::update_config(ConfigOptions {
				api_key: api_key.to_owned(),
				add_description: add_description.to_owned(),
			});
		}
		Some(Commands::Generate { format }) => {
			let _ = GenerateMessageCommand::generate_message(
				&app_config.openai_api_key,
				&app_config.add_description,
				&format,
			)
			.await;
		}
		None => {}
	}

	Ok(())
}
