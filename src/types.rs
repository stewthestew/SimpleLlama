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
// Traits
pub trait Join {
    fn join(&self, separator: &str) -> String;
}

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

impl Join for ModelMemory {
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
}
