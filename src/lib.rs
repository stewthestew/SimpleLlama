// Simple Llama - A Rust library for interacting with LLM models
// This is the top-level module that re-exports everything

// Module declarations
pub mod chat;
pub mod common;
pub mod models;

// Convenience re-exports
pub use chat::{ChatMessage, Message, ModelMemory, add_message};
pub use common::DEFAULT_URL;
pub use models::{ModelOptions, send_message};

// Legacy tests
#[cfg(test)]
mod tests;
