pub mod types;
mod utils;
use reqwest::Client;
use types::{Json, Message, ModelData, ModelMemory};

/// Adds message to history vector
pub fn add_message(model_memory: &mut ModelMemory, role: String, content: String) {
    let msg = Json { role, content };

    model_memory.push(msg);
}

/// Sends message to ollama.
pub async fn send_message(client: &Client, data: &ModelData) -> Result<Message, reqwest::Error> {
    let res = client
        .post("http://localhost:11434/api/chat")
        .json(data) // Send the entire `ModelData` struct as JSON
        .send()
        .await?;

    let status = res.status();
    let text = res.text().await?;

    Ok(Message { status, text })
}
