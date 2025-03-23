// Simple Llama - A Rust library for interacting with LLM models
// This is the top-level module that re-exports everything

// Module declarations
pub mod chat;
pub mod common;
pub mod models;

// Convenience re-exports
pub use chat::{ChatMessage, ModelMemory, Response, add_message};
pub use common::{DEFAULT_COPY_URL, DEFAULT_URL};
pub use models::{CopyInfo, DeleteInfo, ModelOptions, copy_model, delete_model, send_message};

// Legacy tests
#[cfg(test)]
mod tests;
