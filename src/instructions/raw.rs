use crate::instructions::InstructionStrategy;

pub struct RawCommitInstructionStrategy;

impl InstructionStrategy for RawCommitInstructionStrategy {
	fn inject(with_description: &bool) -> String {
		format!(
			r"
You suggest a commit message. Don't add anything else to the response.
The commit message should be structured as follows:

<description>

{}

[optional footer(s)]",
			if *with_description {
				"[optional body]"
			} else {
				""
			}
		)
	}
}
