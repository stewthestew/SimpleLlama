use crate::chat::ModelMemory;
use serde::{Deserialize, Serialize};

/// The struct which contains the options and the memory for the model.
#[derive(Deserialize, Serialize)]
pub struct ModelOptions {
    pub messages: ModelMemory,
    pub temperature: f32,
    pub top_p: f32,
    pub top_k: i32,
    pub model: String,
    pub stream: bool,
}