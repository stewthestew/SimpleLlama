// Simple Llama - A Rust library for interacting with LLM models
// This is the top-level module that re-exports everything

// Module declarations
pub mod chat;
pub mod common;
pub mod models;

// Convenience re-exports
pub use chat::{ChatMessage, ModelMemory, Response, add_message};
pub use common::{DEFAULT_COPY_URL, DEFAULT_DELETE_URL, DEFAULT_PULL_URL, DEFAULT_URL};
pub use models::{
    CopyInfo, DeleteInfo, ModelOptions, PullInfo, copy_model, delete_model, pull_model,
    send_message, send_message_stream,
};

// Legacy tests
#[cfg(test)]
mod tests;
