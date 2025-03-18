use reqwest::Client;
use simple_llama::{
    self, add_message, send_message,
    types::{ModelData, ModelMemory, ModelMemoryMethods},
};

macro_rules! printf {
    ($arg:expr) => {
        use std::io::Write;
        print!("{}", $arg);
        if let Err(e) = std::io::stdout().flush() {
            panic!("printf macro: Error flushing stdout {}", e);
        };
    };
    () => {
        use std::io::Write;
        print!("");
        if let Err(e) = std::io::stdout().flush() {
            panic!("printf macro: Error flushing stdout {}", e);
        };
    };
}

#[tokio::main]
async fn main() {
    // Variables
    let mut history: ModelMemory = Vec::new();
    let client = Client::new();
    let model_data: ModelData = ModelData {
        messages: history.clone(),
        top_p: 1f32,
        top_k: 1,
        temperature: 1.0,
        model: "llama3.1".to_string(),
    };

    // Messages

    add_message(
        &mut history,
        "system".to_string(),
        "Your name is josipher.".to_string(),
    );
    add_message(&mut history, "user".to_string(), "hello".to_string());

    // Sending messages to the llm

    for _ in 0..=3 {
        match send_message(&client, &model_data).await {
            Err(e) => eprintln!("{e}"),
            Ok(val) => {
                printf!(format!("\r{}", val.text));
            }
        }
    }
    println!("\nJoin: {}", history.join("\n"));
}
