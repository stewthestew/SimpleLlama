use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::fmt;

// Types
pub type ModelMemory = Vec<Json>;

// structs

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Json {
    pub role: String,
    pub content: String,
}

pub struct Message {
    pub status: StatusCode,
    pub text: String,
}

#[derive(Deserialize, Serialize)]
pub struct ModelData {
    pub messages: ModelMemory,
    pub temperature: f32,
    pub top_p: f32,
    pub top_k: i32,
    pub model: String,
}

// I need a way to figure out how to make this always true
// the easiest option is to probably just force the user to use the correct struct
// and then just have a separate struct for streaming
// but how would I modify ModelDataStream so the stream option to be always true...
// Since reqwest is kind of a pain to work with, I think I want to modify the struct inside the
// function that sends the message.

// Sounds like a good plan.

pub struct ModelDataStream {
    pub messages: ModelMemory,
    pub temperature: f32,
    pub top_p: f32,
    pub top_k: i32,
    pub model: String,
}

// Traits
pub trait ModelMemoryMethods {
    fn join(&self, separator: &str) -> String;
    fn content(&self, separator: &str) -> String;
}

// The string trait will be used to get the string of the ModelMemory struct.
// This should be easy enough to implement.
// It will work like join but without formatting just returning the `"content": {}` part of the
// struct.

// Implementations
impl fmt::Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"{{"role": "{}", "content": "{}"}}"#,
            &self.role, &self.content
        )
    }
}

impl ModelMemoryMethods for ModelMemory {
    // Write documentation about how this method works.
    // Start

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
    /// use simple_llama::types::Json;
    /// use crate::simple_llama::types::ModelMemoryMethods;
    /// let messages = vec![
    ///    Json {
    ///     role: "system".to_string(),
    ///     content: "Your name is josipher.".to_string(),
    ///    },
    ///    Json {
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
}

// Make this private later.

#[cfg(test)]
mod tests {
    use crate::Json;
    use crate::types::ModelMemoryMethods;

    #[test]
    pub fn test_model_memory_content() {
        let messages = vec![
            Json {
                role: "system".to_string(),
                content: "Your name is john.".to_string(),
            },
            Json {
                role: "user".to_string(),
                content: "Hello".to_string(),
            },
        ];
        assert_eq!(messages.content("\n"), "Your name is john.\nHello")
    }

    #[test]
    pub fn test_model_memory_join() {
        let messages = vec![
            Json {
                role: "system".to_string(),
                content: "Your name is john".to_string(),
            },
            Json {
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
