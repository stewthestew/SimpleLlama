#[cfg(test)]
mod tests {
    use crate::{chat::{add_message, ModelMemoryMethods}, ChatMessage, ModelMemory};

    #[test]
    fn test_model_memory_content() {
        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: "Your name is john.".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: "Hello".to_string(),
            },
        ];
        assert_eq!(messages.content("\n"), "Your name is john.\nHello")
    }

    #[test]
    fn test_model_memory_join() {
        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: "Your name is john".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: "Hello".to_string(),
            },
        ];
        assert_eq!(
            messages.join("\n"),
            "{\"role\": \"system\", \"content\": \"Your name is john\"}\n{\"role\": \"user\", \"content\": \"Hello\"}"
        )
    }

    #[test]
    fn test_add_message() {
        let mut messages: ModelMemory = Vec::new();
        add_message(
            &mut messages,
            "system".to_string(),
            "Testing...".to_string(),
        );
        assert!(
            messages
                == [ChatMessage {
                    role: "system".to_string(),
                    content: "Testing...".to_string(),
                }]
        )
    }
}
