#[cfg(test)]
mod tests {
    use crate::chat::{ChatMessage, ModelMemory, ModelMemoryMethods, add_message};
    use crate::models::ModelData;

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
    fn test_model_data_get_model() {
        let model_data = ModelData {
            messages: vec![],
            temperature: 1.0,
            top_p: 1.0,
            top_k: 1,
            model: "llama3.1".to_string(),
            stream: false,
        };
        assert_eq!(model_data.get_model(), "llama3.1");

        let model_data = ModelData {
            messages: vec![],
            temperature: 1.0,
            top_p: 1.0,
            top_k: 1,
            model: "mistral".to_string(),
            stream: false,
        };
        assert_eq!(model_data.get_model(), "mistral");
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
