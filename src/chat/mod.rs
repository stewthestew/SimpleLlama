// Chat module for simple_llama
// Contains chat message types and methods

mod memory;
mod message;

// Re-export types and functions
pub use memory::{ModelMemory, ModelMemoryMethods};
pub use message::{ChatMessage, Message, MessageMethods};

/// Adds a message to the `model_memory` vector.
/// 
/// # Arguments
/// * `model_memory` - The vector to which the message will be added.
/// * `role` - The role associated with the content.
/// * `content` - The text that gets added to the vector.
/// 
/// # Examples
/// ```rust
/// use simple_llama::{
///     ModelMemory, ModelOptions, add_message, chat::MessageMethods,
/// };
/// // Make the vector which will hold the messages.
/// let mut messages: ModelMemory = Vec::new();
/// 
/// // Adds a message to the vector
/// add_message(
///     &mut messages,
///     "system".to_string(),
///     "Testing...".to_string(),
/// );
/// ```
pub fn add_message(model_memory: &mut ModelMemory, role: String, content: String) {
    let msg = ChatMessage { role, content };
    model_memory.push(msg);
}
