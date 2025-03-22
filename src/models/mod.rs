// Models module for simple_llama
// Contains model options and configuration

mod options;
mod client;

// Re-export types and functions
pub use options::{ModelOptions, ModelData};
pub use client::send_message; 