use std::cmp::max;

use clap::ValueEnum;
use console::style;
use serde_derive::{Deserialize, Serialize};

use crate::prelude::Result;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Format {
	/// Conventional commit message
	Conventional,

	/// Raw commit message
	Raw,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
	pub openai_api_key: String,
	pub add_description: bool,
}

impl ::std::default::Default for AppConfig {
	fn default() -> Self {
		Self {
			openai_api_key: "".into(),
			add_description: true,
		}
	}
}

pub struct ConfigCommand {}

const APP_NAME: &str = env!("CARGO_PKG_NAME");

pub struct ConfigOptions {
	pub api_key: Option<String>,
	pub add_description: Option<bool>,
}

impl ConfigCommand {
	pub fn load() -> Result<AppConfig> {
		let cfg: AppConfig = confy::load(APP_NAME, None)?;

		Ok(cfg)
	}

	pub fn update_config(parameters: ConfigOptions) -> Result<bool> {
		let stored_config = ConfigCommand::load()?;
		let mut updated_config = AppConfig {
			openai_api_key: stored_config.openai_api_key.to_string(),
			..stored_config
		};
		let mut has_changed = false;

		if let Some(api_key) = parameters.api_key {
			updated_config.openai_api_key = api_key;
			has_changed = true;
		}

		if let Some(add_description) = parameters.add_description {
			updated_config.add_description = add_description;
			has_changed = true;
		}

		if !has_changed {
			let separator =
				"=".repeat(max(updated_config.openai_api_key.len(), 5) + 25);

			println!(
				"Stored Settings:\n{}\n{}{}\n{}{}\n{}",
				separator,
				style("  OpenAI API key:   ").bold(),
				&stored_config.openai_api_key,
				style("  Add description:  ").bold(),
				&stored_config.add_description,
				separator
			);

			return Ok(false);
		}

		confy::store(APP_NAME, None, updated_config)?;

		println!(
			"{} Settings have been updated.",
			style("success").green().bold()
		);

		Ok(true)
	}
}
