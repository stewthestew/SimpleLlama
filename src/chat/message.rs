use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

pub struct Message {
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

impl MessageMethods for Message {
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