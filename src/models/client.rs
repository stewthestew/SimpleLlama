use super::{options::ModelOptions, CopyInfo, DeleteInfo};
use crate::chat::Response;
use reqwest::Client;

/// Sends a message to the LLM
///
/// # Arguments
/// * `data` - The data which will be sent to the API
/// * `url` - The URL for the API.
///
/// # Returns
/// A `Result` containing a `Response` on success, or a `reqwest::Error` on failure.
///
/// # Examples
/// ```rust
/// use simple_llama_rs::{
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
pub async fn send_message(data: &ModelOptions, url: &str) -> Result<Response, reqwest::Error> {
    let client = Client::new();
    let result = client.post(url).json(&data).send().await?;
    let status = result.status();
    let text = result.text().await?;

    Ok(Response {
        status_code: status,
        response: text,
    })
}
/// Copies a model under a new name.
///
/// # Arguments
/// * `copy_info` - The information required to copy the model, including source and destination names.
/// * `url` - The URL for the API endpoint that handles model copying. Make sure to use the correct one. Otherwise it won't work
///
/// # Returns
/// A `Result` containing a `Response` on success, or a `reqwest::Error` on failure.
///
/// # Examples
/// ```rust
/// use simple_llama_rs::{CopyInfo, copy_model, DEFAULT_COPY_URL};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let copy_info = CopyInfo {
///         source: "llama3.1".to_string(),
///         destination: "llama_llm".to_string(),
///     };
///
///     let response = copy_model(&copy_info, DEFAULT_COPY_URL).await?;
///     println!("Copy status: {}", response.status_code);
///
///     Ok(())
/// }
/// ```
pub async fn copy_model(copy_info: &CopyInfo, url: &str) -> Result<Response, reqwest::Error> {
    let client = Client::new();
    let result = client.post(url).json(&copy_info).send().await?;
    let status = result.status();
    let text =  result.text().await?;
    Ok(Response { status_code: status, response: text })
}

/// Deletes a model.
///
/// # Arguments
/// * `delete_info` - The information required to delete the model, including the model name.
/// * `url` - The URL for the API endpoint that handles model deletion. Make sure to use the correct one.
///
/// # Returns
/// A `Result` containing a `Response` on success, or a `reqwest::Error` on failure.
///
/// # Examples
/// ```rust
/// use simple_llama_rs::{DeleteInfo, delete_model, DEFAULT_URL};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let delete_info = DeleteInfo {
///         model: "llama_model".to_string(),
///     };
///
///     let response = delete_model(&delete_info, DEFAULT_URL).await?;
///     println!("Delete status: {}", response.status_code);
///
///     Ok(())
/// }
/// ```
pub async fn delete_model(delete_info: &DeleteInfo, url: &str) -> Result<Response, reqwest::Error> {
    let client = Client::new();
    let result = client.post(url).json(&delete_info).send().await?;
    let status = result.status();
    let text =  result.text().await?;
    Ok(Response { status_code: status, response: text })
}
