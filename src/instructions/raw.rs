use crate::instructions::InstructionStrategy;

pub struct RawCommitInstructionStrategy;

impl InstructionStrategy for RawCommitInstructionStrategy {
	fn inject(with_description: &bool) -> String {
		let examples = r###"
#### example 1 ####

##### input #####
```
diff --git a/src/guard.rs b/src/guard.rs
index 563d47d..44c3624 100644
--- a/src/guard.rs
+++ b/src/guard.rs
@@ -8,7 +8,7 @@ pub struct Guard {}
 impl Guard {
 pub fn check_requirements(api_key: &str) -> Result<()> {
   if api_key.is_empty() {
-    return Err(Error::Guard("API key is set".to_string()));
+    return Err(Error::Guard("API key is not set".to_string()));
   }

   if Guard::check_git_status().is_err() {
```

##### output #####

Update error message when API key is not set

#### example 2 ####

##### input #####
```
diff --git a/src/main.rs b/src/main.rs
index 1234567..abcdefg 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,8 +1,11 @@
+use std::time::Duration;
 use tokio::time::delay_for;
 
 #[tokio::main]
 async fn main() {
-    println!("Hello, world!");
+    let message = String::from("Hello, world!");
+    let msg_ref = &message;
+    println!("{}", msg_ref);
+    await!(example_async_function()).unwrap();
 }

diff --git a/src/lib.rs b/src/lib.rs
index 9876543..fedcba9 100644
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -1,5 +1,10 @@
+use tokio::time::Duration;
+use tokio::task;
+
 pub async fn fetch_contacts_from_database() -> Result<(), Box<dyn std::error::Error>> {
-    println!("Welcome to the Rust world!");
-    Ok(())
+    task::spawn(async {
+        delay_for(Duration::from_secs(1)).await;
+        println!("Async function executed!");
+    }).await?;
+    Ok(())
}
```

##### output #####

Introduce tokio to handle async functions

#### example 3 ####

##### input #####
```
diff --git a/src/generator.rs b/src/generator.rs
index d0b8d79..2e33472 100644
--- a/src/generator.rs
+++ b/src/generator.rs
@@ -39,7 +39,7 @@ async fn execute_request(
 				..Default::default()
 			},
 		],
-		temperature: Some(0.2),
+		temperature: Some(0.1),
 		..Default::default()
 	};
 
diff --git a/src/instruction_builder.rs b/src/instruction_builder.rs
index ce88e08..e261cef 100644
--- a/src/instruction_builder.rs
+++ b/src/instruction_builder.rs
@@ -1,42 +1,33 @@
-static INITIAL_PROMPT_INSTRUCTION: &str = r"
-";
-
-static COMMIT_GUIDANCE: &str = r"
-";
-
-static COMMIT_OPTIONAL_BODY: &str = r"
-";
-
 pub struct InstructionBuilder {}
 
 impl InstructionBuilder {
 	pub fn build(instruction_strategy: &str, with_description: &bool) -> String {
-		let mut instructions = vec![
-			INITIAL_PROMPT_INSTRUCTION,
-			instruction_strategy,
-			COMMIT_GUIDANCE,
-		];
+		format!("
 
-		if *with_description {
-			instructions.push(COMMIT_OPTIONAL_BODY);
-		}
-		instructions.join("\n")
+
+        ",
+			instruction_strategy,
+            if *with_description {
+                "Your software should handle this case in order to function properly"
+            } else {
+                ""
+            }
+		)
 	}
 }
diff --git a/src/instructions/conventional.rs b/src/instructions/conventional.rs
index 6ef593a..e1c8bee 100644
--- a/src/instructions/conventional.rs
+++ b/src/instructions/conventional.rs
@@ -4,50 +4,86 @@ pub struct ConventionalCommitInstructionStrategy;
 
 impl InstructionStrategy for ConventionalCommitInstructionStrategy {
 	fn inject() -> &'static str {
-		r"
-# Conventional Commits 1.0.0
+Here are some examples of the input and output of the program:
 	}
 }
```

##### output #####

"###;

		let closing_details = format!(
			r"
Improve commit generation prompt and guidance

{}

### examples end ###

### output details ###

You should respond with a commit message, which is a short and concise description of the changes made in the file.
",
			if *with_description {
				r#"
- Reduce sampling temperature value to get more precise response
- Improve the initial prompt instruction to be more descriptive
            "#
			} else {
				""
			}
		);

		format!("{}{}", examples, closing_details)
	}
}
