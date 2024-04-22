use clap::{Parser, ValueEnum};
use instruction_builder::InstructionBuilder;
use instructions::InstructionStrategy;

mod diff_collector;
mod instruction_builder;
mod instructions;
mod message_generator;

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
}

#[tokio::main]
async fn main() {
	let args = Args::parse();

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

	let generated_message = message_generator::generate_message(
		&collected_changed,
        // TODO: this builder needs a constructor where I can inject the instructions
		InstructionBuilder::build(instructions_injector),
	)
	.await;

	println!("{:?}", generated_message);
}
