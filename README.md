> [!NOTE]
> Simple Llama is still in development, not even in it's alpha stage, so expect stuff to break.
> For now use Ollama-rs if you want something that is reliable.
# Simple Llama
Simple Llama is an Ollama wrapper for rust, that makes using Ollama simple yet customizable.


## Dependencies
`openssl`

`ollama`

If there are any more dependencies this crate requires, then make an issue.

## Examples


Without streaming:
```rust
use reqwest::Client;
use simple_llama::{
    add_message,
    send_message,
    types::{ModelOptions, ModelMemory, DEFAULT_URL},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new HTTP client
    let client = Client::new();
    
    // Initialize an empty message history
    let mut messages: ModelMemory = Vec::new();
    
    // Add a system message to set the context
    add_message(
        &mut messages,
        "system".to_string(),
        "You are a helpful assistant.".to_string(),
    );
    
    // Add a user message
    add_message(
        &mut messages,
        "user".to_string(),
        "What is the capital of France?".to_string(),
    );
    
    // Configure the model options
    // This also gives the model the messages
    let data = ModelOptions {
        messages,
        temperature: 0.7,
        top_p: 0.9,
        top_k: 40,
        model: "llama3.1".to_string(),
        stream: false,
    };
    
    // Send the message to Ollama and get the response
    let response = send_message(&client, &data, DEFAULT_URL).await?;
    
    // Print the response
    println!("Response: {}", response.get_llm_content());
    
    Ok(())
}
```
