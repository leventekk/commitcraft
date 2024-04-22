use std::env;

use openai_dive::v1::api::Client;
use openai_dive::v1::models::Gpt35Engine;
use openai_dive::v1::resources::chat::{
	ChatCompletionParameters, ChatCompletionResponse, ChatMessage,
	ChatMessageContent, Role,
};

async fn execute_request(
	system_messge: &str,
	user_message: &str,
) -> ChatCompletionResponse {
	let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

	let client = Client::new(api_key);

	let parameters = ChatCompletionParameters {
		model: Gpt35Engine::Gpt35Turbo1106.to_string(),
		messages: vec![
			ChatMessage {
				role: Role::System,
				content: ChatMessageContent::Text(system_messge.to_string()),
				..Default::default()
			},
			ChatMessage {
				role: Role::User,
				content: ChatMessageContent::Text(user_message.to_string()),
				..Default::default()
			},
		],
		temperature: Some(0.2),
		..Default::default()
	};

	client.chat().create(parameters).await.unwrap()
}

fn prettify_message(message: ChatMessageContent) -> String {
	match message {
		ChatMessageContent::Text(str) => str,
		_ => "".to_string(),
	}
}

fn extract_response_choice(response: ChatCompletionResponse) -> Option<String> {
	response
		.choices
		.into_iter()
		.map(|choice| prettify_message(choice.message.content))
		.collect::<Vec<_>>()
		.first()
		.cloned()
}

// we should provide a basic example of how a git diff looks like
pub async fn generate_message(
	git_diff: &str,
    // TODO: here we need to inject the builder
	instructions: String,
) -> String {
	let response = execute_request(&instructions, git_diff).await;
	let result = extract_response_choice(response);

	match result {
		Some(val) => val,
		_ => "".to_string(),
	}
}
