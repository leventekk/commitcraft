use crate::instructions::InstructionStrategy;

static INITIAL_PROMPT_INSTRUCTION: &str = r"You're a git commit generator expert. The user provides a `git diff` format for you and your task is to generate a commit message based on the given diff.

Here is a sample of what a `git diff` looks like:
```
index ad4db42..f3b18a9 100644
--- a/src/server.ts
+++ b/src/server.ts
@@ -10,7 +10,7 @@
import {
    initWinstonLogger();
    
    const app = express();
    -const port = 7799;
    +const PORT = 7799;
    
    app.use(express.json());
    
    @@ -34,6 +34,6 @@
    app.use((_, res, next) => {
        // ROUTES
        app.use(PROTECTED_ROUTER_URL, protectedRouter);
        
        -app.listen(port, () => {
            -  console.log(\`Server listening on port \${port}\`);
            +app.listen(process.env.PORT || PORT, () => {
                +  console.log(\`Server listening on port \${PORT}\`);
            });`
```
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
- respond with a simple text without any markdown or code blocks
";

static DIFF_SEPARATOR: &str = "now here is the diff";

pub struct InstructionBuilder {}

impl InstructionBuilder {
	pub fn build(instruction_strategy: Box<dyn InstructionStrategy>) -> String {
		[
			INITIAL_PROMPT_INSTRUCTION,
			instruction_strategy.inject(),
			COMMIT_GUIDANCE,
			DIFF_SEPARATOR,
		]
		.join("\n")
	}
}
