use std::{
	env,
	fs::{self, File},
	io::{Read, Write},
	path::PathBuf,
};

use crate::prelude::Result;

pub struct Backup {}

impl Backup {
	pub fn commit(message: &str) -> Result<()> {
		let file_path = get_file_path()?;
		let mut file = File::create(file_path)?;

		file.write_all(message.as_bytes())?;

		Ok(())
	}

	pub fn recover() -> Result<String> {
		let file_path = get_file_path()?;
		let mut content = String::new();
		let file = File::open(file_path);

		match file {
			Ok(mut file) => {
				file.read_to_string(&mut content)?;

				Ok(content)
			}
			Err(_) => Ok(String::new()),
		}
	}

	pub fn destroy() -> Result<()> {
		let file_path = get_file_path()?;
		fs::remove_file(file_path)?;

		Ok(())
	}
}

const BACKUP_FILE_NAME: &str = "commitcraft-backup";

fn get_file_path() -> Result<PathBuf> {
	let dir = env::temp_dir();
	let file_path = dir.join(BACKUP_FILE_NAME);

	Ok(file_path)
}
