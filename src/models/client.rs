use reqwest::Client;
use super::options::ModelOptions;
use crate::chat::Message;

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