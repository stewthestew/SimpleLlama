// Chat module for simple_llama
// Contains chat message types and methods

mod memory;
mod message;

// Re-export types and functions
pub use memory::{ModelMemory, ModelMemoryMethods};
pub use message::{ChatMessage, MessageMethods, Response, add_message};
