use openai_dive::v1::api::Client;
use openai_dive::v1::error::APIError;
use openai_dive::v1::models::Gpt4Engine;
use openai_dive::v1::resources::chat::{
	ChatCompletionParametersBuilder, ChatCompletionResponse,
	ChatCompletionResponseFormat, ChatMessage, ChatMessageContent,
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

	let parameters = ChatCompletionParametersBuilder::default()
		.model(Gpt4Engine::Gpt4OMini.to_string())
		.messages(vec![
			ChatMessage::System {
				content: ChatMessageContent::Text(system_messge.to_string()),
				name: None,
			},
			ChatMessage::User {
				content: ChatMessageContent::Text(user_message.to_string()),
				name: None,
			},
		])
		.temperature(0.2)
		.response_format(ChatCompletionResponseFormat::Text)
		.build();

	if let Ok(parameters) = parameters {
		match client.chat().create(parameters).await {
			Ok(response) => Ok(response),
			Err(error) => Err(error),
		}
	} else {
		Err(APIError::BadRequestError(
			("Failed to build parameters").to_string(),
		))
	}
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
		.filter_map(|choice| match choice.message {
			ChatMessage::User { content, .. } => prettify_message(content),
			ChatMessage::System { content, .. } => prettify_message(content),
			ChatMessage::Assistant { content, .. } => {
				if let Some(content) = content {
					prettify_message(content)
				} else {
					None
				}
			}
			_ => None,
		})
		.collect::<Vec<_>>()
		.first()
		.cloned()
}
