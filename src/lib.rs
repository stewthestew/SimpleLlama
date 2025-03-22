// Simple Llama - A Rust library for interacting with LLM models
// This is the top-level module that re-exports everything

// Module declarations
pub mod chat;
pub mod models;
pub mod common;

// Convenience re-exports
pub use chat::{ChatMessage, Message, ModelMemory, add_message};
pub use models::{ModelOptions, ModelData, send_message};
pub use common::DEFAULT_URL;

// Legacy tests
#[cfg(test)]
mod tests;