// Models module for simple_llama
// Contains model options and configuration

mod client;
mod options;

// Re-export types and functions
pub use client::send_message;
pub use options::{ModelData, ModelOptions};
