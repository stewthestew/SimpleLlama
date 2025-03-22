use std::{
    io::{Write, stdin, stdout},
    process::exit,
};

use simple_llama_rs::{
    DEFAULT_URL, ModelMemory, ModelOptions, add_message, chat::MessageMethods, send_message,
};

#[tokio::main]
async fn main() {
    // Variables
    let mut history: ModelMemory = Vec::new();

    loop {
        let mut input = String::new();
        print!(": ");
        if let Err(e) = stdout().flush() {
            eprintln!("Failed to flush stdout. {e}");
        };

        if let Err(e) = stdin().read_line(&mut input) {
            eprintln!("Failed to read user input. {e}");
            exit(1);
        };

        add_message(&mut history, "user".to_string(), input);

        let model_data = ModelOptions {
            messages: history.clone(), // You should find a way not to clone it
            top_p: 1f32,
            top_k: 1,
            temperature: 0.7,
            model: "llama3.1".to_string(),
            stream: false,
        };

        match send_message(&model_data, DEFAULT_URL).await {
            Err(e) => eprintln!("{e}"),
            Ok(val) => {
                println!("{}: {}", model_data.model, val.get_llm_content());
                add_message(&mut history, "assistant".to_string(), val.get_llm_content());
            }
        }
    }
}
