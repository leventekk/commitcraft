use crate::instructions::InstructionStrategy;

static INITIAL_PROMPT_INSTRUCTION: &str = r"
You're a git commit generator expert.
The user provides a `git diff` format for you and your task is to generate a commit message based on the given diff.
";

static COMMIT_GUIDANCE: &str = r"
Some guidance for generating the commit:
-  Use the present tense. Lines must not be longer than 74 characters.
- As for the [optional body], you can include all the changes that cannot fit into the <description>
- If you're using a list to mention all of the changes in the [optional body], for example:
```
- added a new feature called list items
- fixed a bug when the list is empty
```
- if there is only one item that can be listed in the [optional body] you can write it as a sentence, no need to use a dash
- do not start the sentences with a capital letter, use small case everywhere
- do not add any extra formatting to the commit message
- remove the starting and ending backticks and also the new lines
";

pub struct InstructionBuilder {}

impl InstructionBuilder {
	pub fn build(instruction_strategy: Box<dyn InstructionStrategy>) -> String {
		[
			INITIAL_PROMPT_INSTRUCTION,
			instruction_strategy.inject(),
			COMMIT_GUIDANCE,
		]
		.join("\n")
	}
}
