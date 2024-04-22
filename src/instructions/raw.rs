use crate::instructions::InstructionStrategy;

pub struct RawCommitInstructionStrategy;

impl InstructionStrategy for RawCommitInstructionStrategy {
	fn inject(&self) -> &str {
		r"You suggest a commit message. Don't add anything else to the response.

        The commit message should be structured as follows:

        <description>

        [optional body]

        [optional footer(s)]"
	}
}
