use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::fmt;

// Constants
pub const DEFAULT_URL: &str = "http://localhost:11434/api/chat";

// Types
pub type ModelMemory = Vec<ChatMessage>;

// structs

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

pub struct Message {
    pub status_code: StatusCode,
    pub response: String,
}

#[derive(Deserialize, Serialize)]
pub struct ModelOptions {
    pub messages: ModelMemory,
    pub temperature: f32,
    pub top_p: f32,
    pub top_k: i32,
    pub model: String,
    pub stream: bool,
}

// I need a way to figure out how to make this always true
// the easiest option is to probably just force the user to use the correct struct
// and then just have a separate struct for streaming
// but how would I modify ModelDataStream so the stream option to be always true...
// Since reqwest is kind of a pain to work with, I think I want to modify the struct inside the
// function that sends the message.

// Sounds like a good plan.
// I will do that later..

pub struct ModelData {
    pub messages: ModelMemory,
    pub temperature: f32,
    pub top_p: f32,
    pub top_k: i32,
    pub model: String,
    pub stream: bool,
}

// Traits
pub trait ModelMemoryMethods {
    fn join(&self, separator: &str) -> String;
    fn content(&self, separator: &str) -> String;
}

pub trait MessageMethods {
    fn get_llm_content(&self) -> String;
}

// The string trait will be used to get the string of the ModelMemory struct.
// This should be easy enough to implement.
// It will work like join but without formatting just returning the `"content": {}` part of the
// struct.

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

impl ModelMemoryMethods for ModelMemory {
    // TODO:
    // Write documentation for the join method
    fn join(&self, separator: &str) -> String {
        self.iter()
            .map(|msg| {
                format!(
                    r#"{{"role": "{}", "content": "{}"}}"#,
                    msg.role, msg.content
                )
            }) // Format each message
            .collect::<Vec<String>>()
            .join(separator) // Join the strings using the separator
    }

    /// Similar to join but only returns the content of the messages.
    /// # Examples
    /// ``` rust
    /// use simple_llama::types::ChatMessage;
    /// use crate::simple_llama::types::ModelMemoryMethods;
    /// let messages = vec![
    ///    ChatMessage {
    ///     role: "system".to_string(),
    ///     content: "Your name is josipher.".to_string(),
    ///    },
    ///    ChatMessage {
    ///     role: "user".to_string(),
    ///     content: "hello".to_string(),
    ///    },
    /// ];
    /// assert_eq!(messages.content("\n"), "Your name is josipher.\nhello");
    fn content(&self, separator: &str) -> String {
        self.iter()
            .map(|msg| format!("{}", msg.content))
            .collect::<Vec<String>>()
            .join(separator)
    }
    // TODO:
    // add a role method
}

impl MessageMethods for Message {
    fn get_llm_content(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::ChatMessage;
    use crate::types::ModelMemoryMethods;

    #[test]
    pub fn test_model_memory_content() {
        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: "Your name is john.".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: "Hello".to_string(),
            },
        ];
        assert_eq!(messages.content("\n"), "Your name is john.\nHello")
    }

    #[test]
    pub fn test_model_memory_join() {
        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: "Your name is john".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: "Hello".to_string(),
            },
        ];
        assert_eq!(
            messages.join("\n"),
            "{\"role\": \"system\", \"content\": \"Your name is john\"}\n{\"role\": \"user\", \"content\": \"Hello\"}"
        )
    }
}
