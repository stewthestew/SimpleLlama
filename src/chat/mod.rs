// Chat module for simple_llama
// Contains chat message types and methods

mod message;
mod memory;

// Re-export types and functions
pub use message::{ChatMessage, Message, MessageMethods};
pub use memory::{ModelMemory, ModelMemoryMethods};

// Functions
pub fn add_message(model_memory: &mut ModelMemory, role: String, content: String) {
    let msg = ChatMessage { role, content };
    model_memory.push(msg);
} 