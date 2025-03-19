use reqwest::Client;
use simple_llama::{
    add_message, send_message,
    types::{DEFAULT_URL, ModelMemory, ModelOptions},
};

#[tokio::main]
async fn main() {
    // Variables
    let mut history: ModelMemory = Vec::new();
    let client = Client::new();

    // Messages

    add_message(
        &mut history,
        "system".to_string(),
        "You're a time machine.".to_string(),
    );

    add_message(&mut history, "user".to_string(), "hello".to_string());
    println!("{history:?}");

    // Sending messages to the llm
    // Model data needs to be created after adding the messages for now.
    // later on this will most likely be improved.
    let model_data = ModelOptions {
        messages: history.clone(),
        top_p: 1f32,
        top_k: 1,
        temperature: 1.0,
        model: "llama3.1".to_string(),
        stream: false,
    };

    match send_message(&client, &model_data, DEFAULT_URL).await {
        Err(e) => eprintln!("{e}"),
        Ok(val) => {
            println!(
                "Response: {}\nstatus code: {}",
                val.response, val.status_code
            );
        }
    }
}
