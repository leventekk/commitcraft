#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Generic {0}")]
	Generic(String),

    #[error("CommitMessage {0}")]
    CommitMessage(String),

	#[error("Guard {0}")]
	Guard(String),

	#[error(transparent)]
	IO(#[from] std::io::Error),

	#[error("Confy error: {0}")]
	Confy(#[from] confy::ConfyError),
}
