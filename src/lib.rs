pub mod types;
mod utils;
use reqwest::Client;
use types::{ChatMessage, Message, ModelMemory, ModelOptions};

/// Adds message to history vector
pub fn add_message(model_memory: &mut ModelMemory, role: String, content: String) {
    let msg = ChatMessage { role, content };

    model_memory.push(msg);
}

/// Sends message to ollama.
pub async fn send_message(
    client: &Client,
    data: &ModelOptions,
    url: &str,
) -> Result<Message, reqwest::Error> {
    let result = client.post(url).json(&data).send().await?;

    let status = result.status();
    let text = result.text().await?;

    Ok(Message {
        status_code: status,
        response: text,
    })
}

#[cfg(test)]
mod tests {
    use crate::{
        add_message,
        types::{ChatMessage, ModelMemory},
    };

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
