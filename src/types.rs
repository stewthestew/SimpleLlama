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

impl ModelData {
    /// Returns the model name from the ModelData struct.
    /// 
    /// # Examples
    /// ```rust
    /// use simple_llama::types::ModelData;
    /// 
    /// let model_data = ModelData {
    ///     messages: vec![],
    ///     temperature: 1.0,
    ///     top_p: 1.0,
    ///     top_k: 1,
    ///     model: "llama2".to_string(),
    ///     stream: false,
    /// };
    /// assert_eq!(model_data.get_model(), "llama2");
    /// ```
    pub fn get_model(&self) -> &str {
        &self.model
    }
}

// Traits
pub trait ModelMemoryMethods {
    /// Joins all messages in the memory into a single string with the specified separator.
    /// Each message is formatted as a JSON object with role and content.
    /// 
    /// # Arguments
    /// * `separator` - The string to use between messages
    /// 
    /// # Examples
    /// ```rust
    /// use simple_llama::types::{ChatMessage, ModelMemoryMethods};
    /// 
    /// let messages = vec![
    ///     ChatMessage {
    ///         role: "system".to_string(),
    ///         content: "You are a helpful assistant.".to_string(),
    ///     },
    ///     ChatMessage {
    ///         role: "user".to_string(),
    ///         content: "Hello".to_string(),
    ///     },
    /// ];
    /// let result = messages.join("\n");
    /// assert!(result.contains(r#"{"role": "system", "content": "You are a helpful assistant."}"#));
    /// ```
    fn join(&self, separator: &str) -> String;

    /// Returns only the content of all messages joined with the specified separator.
    /// 
    /// # Arguments
    /// * `separator` - The string to use between message contents
    /// 
    /// # Examples
    /// ```rust
    /// use simple_llama::types::{ChatMessage, ModelMemoryMethods};
    /// 
    /// let messages = vec![
    ///     ChatMessage {
    ///         role: "system".to_string(),
    ///         content: "You are a helpful assistant.".to_string(),
    ///     },
    ///     ChatMessage {
    ///         role: "user".to_string(),
    ///         content: "Hello".to_string(),
    ///     },
    /// ];
    /// assert_eq!(messages.content("\n"), "You are a helpful assistant.\nHello");
    /// ```
    fn content(&self, separator: &str) -> String;

    /// Returns only the roles of all messages joined with the specified separator.
    /// 
    /// # Arguments
    /// * `separator` - The string to use between roles
    /// 
    /// # Examples
    /// ```rust
    /// use simple_llama::types::{ChatMessage, ModelMemoryMethods};
    /// 
    /// let messages = vec![
    ///     ChatMessage {
    ///         role: "system".to_string(),
    ///         content: "You are a helpful assistant.".to_string(),
    ///     },
    ///     ChatMessage {
    ///         role: "user".to_string(),
    ///         content: "Hello".to_string(),
    ///     },
    /// ];
    /// assert_eq!(messages.role("\n"), "system\nuser");
    /// ```
    fn role(&self, separator: &str) -> String;
}

pub trait MessageMethods {
    /// Extracts the content from the LLM response JSON.
    /// 
    /// # Returns
    /// The content field from the response, or an empty string if parsing fails.
    /// 
    /// # Examples
    /// ```rust
    /// use simple_llama::types::{Message, MessageMethods};
    /// 
    /// let message = Message {
    ///     status_code: reqwest::StatusCode::OK,
    ///     response: r#"{"message": {"content": "Hello, how can I help?"}}"#.to_string(),
    /// };
    /// assert_eq!(message.get_llm_content(), "Hello, how can I help?");
    /// ```
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
    /// Joins all messages in the memory into a single string with the specified separator.
    /// Each message is formatted as a JSON object with role and content.
    /// 
    /// # Arguments
    /// * `separator` - The string to use between messages
    /// 
    /// # Examples
    /// ```rust
    /// use simple_llama::types::{ChatMessage, ModelMemoryMethods};
    /// 
    /// let messages = vec![
    ///     ChatMessage {
    ///         role: "system".to_string(),
    ///         content: "You are a helpful assistant.".to_string(),
    ///     },
    ///     ChatMessage {
    ///         role: "user".to_string(),
    ///         content: "Hello".to_string(),
    ///     },
    /// ];
    /// let result = messages.join("\n");
    /// assert!(result.contains(r#"{"role": "system", "content": "You are a helpful assistant."}"#));
    /// ```
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

    /// Returns only the content of all messages joined with the specified separator.
    /// 
    /// # Arguments
    /// * `separator` - The string to use between message contents
    /// 
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
    /// Similar to content but only returns the role of the messages.
    /// # Examples
    /// ``` rust
    /// use simple_llama::types::ChatMessage;
    /// use crate::simple_llama::types::ModelMemoryMethods;
    /// let messages = vec![
    ///     ChatMessage {
    ///         role: "system".to_string(),
    ///         content: "Your name is josipher.".to_string(),
    ///     },
    ///     ChatMessage {
    ///         role: "user".to_string(),
    ///         content: "hello".to_string(),
    ///     },
    /// ];
    /// assert_eq!(messages.role("\n"), "system\nuser");
    fn role(&self, separator: &str) -> String {
        self.iter()
            .map(|msg| format!("{}", msg.role))
            .collect::<Vec<String>>()
            .join(separator)
    }
}

impl MessageMethods for Message {
    /// Extracts the content from the LLM response JSON.
    /// Returns the content field from the response, or an empty string if parsing fails.
    fn get_llm_content(&self) -> String {
        match serde_json::from_str::<serde_json::Value>(&self.response) {
            Ok(json) => {
                if let Some(content) = json.get("message").and_then(|msg| msg.get("content")) {
                    content.as_str().unwrap_or_default().to_string()
                } else {
                    String::new()
                }
            }
            Err(_) => String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ChatMessage;
    use crate::types::{ModelMemoryMethods, ModelData};

    #[test]
    fn test_model_memory_content() {
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
    fn test_model_memory_join() {
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

    #[test]
    fn test_model_data_get_model() {
        let model_data = ModelData {
            messages: vec![],
            temperature: 1.0,
            top_p: 1.0,
            top_k: 1,
            model: "llama2".to_string(),
            stream: false,
        };
        assert_eq!(model_data.get_model(), "llama2");

        let model_data = ModelData {
            messages: vec![],
            temperature: 1.0,
            top_p: 1.0,
            top_k: 1,
            model: "mistral".to_string(),
            stream: false,
        };
        assert_eq!(model_data.get_model(), "mistral");
    }
}
