use clap::{Parser, ValueEnum};

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
	#[arg(value_enum, default_value = "conventional")]
	format: Format,
}

fn main() {
	let args = Args::parse();

	match args.format {
		Format::Conventional => println!("Conventional commit message"),
		Format::Raw => println!("Raw commit message"),
	}
}
