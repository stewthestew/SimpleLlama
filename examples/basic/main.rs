use std::{io::{stdin, stdout, Write}, process::exit};

use reqwest::Client;
use simple_llama::{
    add_message, send_message,
    types::{MessageMethods, ModelMemory, ModelOptions, DEFAULT_URL},
};

#[tokio::main]
async fn main() {
    // Variables
    let mut history: ModelMemory = Vec::new();
    let client = Client::new();
    let mut input = String::new();

    loop {
        print!(": ");
        if stdout().flush().is_err() {
            eprintln!("Failed to flush stdout.");
        };

        if stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read user input.");
            exit(0);
        };

        add_message(
            &mut history,
            "user".to_string(),
            input.clone(),
        );
        input.clear();

        add_message(&mut history, "user".to_string(), "hello".to_string());
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
                    "Response: {}",
                    val.get_llm_content()
                );
            }
        }
    }
}