pub struct InstructionBuilder {}

impl InstructionBuilder {
	pub fn build(instruction_strategy: &str, with_description: &bool) -> String {
		format!("
### instructions ###

Act as a professional developer and text analyst, who can deliver its best in parsing Git diff.

The user will provide you a Git diff file, and you will have to parse it and extract the following information:
- What is the filename?
- What changes were made in the related file? You should summarize the changes in a few sentences.

When summarizing the changes, you should not include the actual code changes, but the context of the changes.
If the given input contains some mardown or a git diff, exclude it from the processing.

Do not focus on lock files, binaries, or any other files that are not related to the codebase.

### examples ###

Here are some examples of the input given by the user and the desired output you should build.

{}

### output rules ###

In the response, please use present tense and do not exceed 74 characters per line.
Do not include the found diffs under the examples section when generating a message.
If the given input is just a new file without any changes, output the filename and that the file was added.
Output only plain text. Do not output markdown.
{}
        ",
			instruction_strategy,
            if *with_description {
                "Please include a summarized list of changes separated by - in the commit body, but for each item be concise and focus on why the change was made."
            } else {
                "Please do not add any changes to the commit body."
            }
		)
	}
}
