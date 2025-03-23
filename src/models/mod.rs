// Models module for simple_llama_rs
// Contains model options and configuration

mod client;
mod copy_info;
mod delete_info;
mod options;

// Re-export types and functions
pub use client::{copy_model, send_message, delete_model};
pub use copy_info::CopyInfo;
pub use options::ModelOptions;
pub use delete_info::DeleteInfo;
