#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Generic {0}")]
	Generic(String),

	#[error("Guard {0}")]
	Guard(String),

	#[error("{0}")]
	Executor(String),

	#[error(transparent)]
	IO(#[from] std::io::Error),

	#[error("Confy error: {0}")]
	Confy(#[from] confy::ConfyError),
}
