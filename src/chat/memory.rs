use super::message::ChatMessage;

// Types
pub type ModelMemory = Vec<ChatMessage>;

// Traits
pub trait ModelMemoryMethods {
    /// Joins all messages in the memory into a single string with the specified separator.
    /// Each message is formatted as a JSON object with role and content.
    ///
    /// # Arguments
    /// * `separator` - The string to use between messages
    fn join(&self, separator: &str) -> String;

    /// Returns only the content of all messages joined with the specified separator.
    ///
    /// # Arguments
    /// * `separator` - The string to use between message contents
    fn content(&self, separator: &str) -> String;

    /// Returns only the roles of all messages joined with the specified separator.
    ///
    /// # Arguments
    /// * `separator` - The string to use between roles
    fn role(&self, separator: &str) -> String;
}

// Implementations
impl ModelMemoryMethods for ModelMemory {
    fn join(&self, separator: &str) -> String {
        self.iter()
            .map(|msg| {
                format!(
                    r#"{{"role": "{}", "content": "{}"}}"#,
                    msg.role, msg.content
                )
            })
            .collect::<Vec<String>>()
            .join(separator)
    }

    fn content(&self, separator: &str) -> String {
        self.iter()
            .map(|msg| format!("{}", msg.content))
            .collect::<Vec<String>>()
            .join(separator)
    }

    fn role(&self, separator: &str) -> String {
        self.iter()
            .map(|msg| format!("{}", msg.role))
            .collect::<Vec<String>>()
            .join(separator)
    }
}
