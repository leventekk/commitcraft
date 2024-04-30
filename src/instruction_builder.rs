static INITIAL_PROMPT_INSTRUCTION: &str = r"
You're a git commit generator expert.
The user provides a `git diff` format for you and your task is to generate a commit message based on the given diff.
";

static COMMIT_GUIDANCE: &str = r"
Some guidance for generating the commit:
- Use the present tense. Lines must not be longer than 74 characters.
- Do not start the sentences with a capital letter, use small case everywhere
- Do not add any extra formatting to the commit message
";

static COMMIT_OPTIONAL_BODY: &str = r"
- As for the [optional body], you can include all the changes that cannot fit into the <description>
- If you're using a list to mention all of the changes in the [optional body], for example:
```
- added a new feature called list items
- fixed a bug when the list is empty
```
- If there is only one item that can be listed in the [optional body] you can write it as a sentence, no need to use a dash
- Focus on code changes and the reason why those changes were made
- Remove the starting and ending backticks and also the new lines
";

pub struct InstructionBuilder {}

impl InstructionBuilder {
	pub fn build(instruction_strategy: &str, with_description: &bool) -> String {
		let mut instructions = vec![
			INITIAL_PROMPT_INSTRUCTION,
			instruction_strategy,
			COMMIT_GUIDANCE,
		];

		if *with_description {
			instructions.push(COMMIT_OPTIONAL_BODY);
		}

		instructions.join("\n")
	}
}
