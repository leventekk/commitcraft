use crate::AppConfig;

pub struct ConfigCommand {}

const APP_NAME: &str = env!("CARGO_PKG_NAME");

impl ConfigCommand {
	pub fn load() -> Result<AppConfig, confy::ConfyError> {
		let cfg: AppConfig = confy::load(APP_NAME, None)?;

		Ok(cfg)
	}

	pub fn update_config(arguments: AppConfig) -> Result<(), confy::ConfyError> {
		let cfg = match Self::load() {
			Ok(c) => c,
			Err(e) => AppConfig { openai_api_key: "".to_string() },
		};

		let mut config = AppConfig { ..cfg };

		if !arguments.openai_api_key.is_empty() {
			config.openai_api_key = arguments.openai_api_key.clone();
		}

		confy::store(APP_NAME, None, config)?;
		println!("Configuration saved.");

		Ok(())
	}
	pub fn retrieve_config() {}
}
