use super::options::ModelOptions;
use crate::chat::Message;
use reqwest::Client;

/// Sends a message to the LLM
///
/// # Arguments
/// * `data` - The data which will be sent to the API
/// * `url` - The URL for the API.
///
/// # Returns
/// A `Result` containing a `Message` on success, or a `reqwest::Error` on failure.
///
/// # Examples
/// ```rust
/// use simple_llama::{
///     ModelMemory, ModelOptions, DEFAULT_URL, chat::MessageMethods, send_message
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut messages: ModelMemory = Vec::new();
///     
///     // Create model options
///     let model_data = ModelOptions {
///         messages,
///         temperature: 0.7,
///         top_p: 1.0,
///         top_k: 1,
///         model: "llama3.1".to_string(),
///         stream: false,
///     };
///     
///     // Send message to the model
///     let response = send_message(&model_data, DEFAULT_URL).await?;
///     
///     // Get the content from the response
///     let content = response.get_llm_content();
///     println!("Model response: {}", content);
///     
///     Ok(())
/// }
/// ```
pub async fn send_message(data: &ModelOptions, url: &str) -> Result<Message, reqwest::Error> {
    let client = Client::new();
    let result = client.post(url).json(&data).send().await?;
    let status = result.status();
    let text = result.text().await?;

    Ok(Message {
        status_code: status,
        response: text,
    })
}
