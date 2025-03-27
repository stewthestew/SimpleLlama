use crate::ModelMemory;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

pub struct Response {
    pub status_code: StatusCode,
    pub response: String,
}

// Traits
pub trait MessageMethods {
    /// Extracts the content from the LLM response JSON.
    ///
    /// # Returns
    /// The content field from the response, or an empty string if parsing fails.
    fn get_llm_content(&self) -> String;
}

// Implementations
impl fmt::Display for ChatMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"{{"role": "{}", "content": "{}"}}"#,
            &self.role, &self.content
        )
    }
}

impl MessageMethods for Response {
    fn get_llm_content(&self) -> String {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&self.response) {
            if let Some(content) = json.get("message").and_then(|m| m.get("content")) {
                if let Some(text) = content.as_str() {
                    return text.to_string();
                }
            }
        }
        String::new()
    }
}

/// Adds a message to the `model_memory` vector.
///
/// # Arguments
/// * `model_memory` - The vector to which the message will be added.
/// * `role` - The role associated with the content.
/// * `content` - The text that gets added to the vector.
///
/// # Examples
/// ```rust
/// use simple_llama_rs::{
///     ModelMemory, ModelOptions, add_message, chat::MessageMethods,
/// };
/// // Make the vector which will hold the messages.
/// let mut messages: ModelMemory = Vec::new();
///
/// // Adds a message to the vector
/// add_message(
///     &mut messages,
///     "system".to_string(),
///     "Testing...".to_string(),
/// );
/// ```
pub fn add_message(model_memory: &mut ModelMemory, role: String, content: String) {
    let msg = ChatMessage { role, content };
    model_memory.push(msg);
}
