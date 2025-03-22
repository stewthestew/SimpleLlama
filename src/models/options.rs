use crate::chat::ModelMemory;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ModelOptions {
    pub messages: ModelMemory,
    pub temperature: f32,
    pub top_p: f32,
    pub top_k: i32,
    pub model: String,
    pub stream: bool,
}

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
    pub fn get_model(&self) -> &str {
        &self.model
    }
}
