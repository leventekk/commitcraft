use openai_dive::v1::api::Client;
use openai_dive::v1::error::APIError;
use openai_dive::v1::models::Gpt35Engine;
use openai_dive::v1::resources::chat::{
	ChatCompletionParameters, ChatCompletionResponse, ChatMessage,
	ChatMessageContent, Role,
};
use serde::Deserialize;

pub struct Generator {}

#[derive(Debug, Deserialize)]
struct RequestError {
	message: String,
}

impl Generator {
	pub async fn generate_message(
		api_key: &str,
		git_diff: &str,
		instructions: &str,
	) -> Result<String, String> {
		let response = execute_request(api_key, instructions, git_diff).await;

		match response {
			Ok(response) => {
				Ok(extract_response_choice(response).unwrap_or_default())
			}
			Err(error) => {
				let v = serde_json::from_str::<RequestError>(&error.to_string())
					.unwrap();

				Err(v.message)
			}
		}
	}
}

async fn execute_request(
	api_key: &str,
	system_messge: &str,
	user_message: &str,
) -> Result<ChatCompletionResponse, APIError> {
	let client = Client::new(api_key.to_string());

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

	client.chat().create(parameters).await
}

fn prettify_message(message: ChatMessageContent) -> Option<String> {
	match message {
		ChatMessageContent::Text(str) => Some(str),
		_ => None,
	}
}

fn extract_response_choice(response: ChatCompletionResponse) -> Option<String> {
	response
		.choices
		.into_iter()
		.filter_map(|choice| prettify_message(choice.message.content))
		.collect::<Vec<_>>()
		.first()
		.cloned()
}
