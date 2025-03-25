use std::{
    io::{Write, stdin, stdout},
    process::exit,
};

use simple_llama_rs::{
    add_message, stream::send_message_stream, ModelMemory, ModelOptions, DEFAULT_URL
};
use tokio_stream::StreamExt;

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
            messages: history.clone(),
            top_p: 1f32,
            top_k: 1,
            temperature: 0.7,
            model: "llama3.1".to_string(),
            stream: true,
        };

        match send_message_stream(&model_data, DEFAULT_URL).await {
            Err(e) => eprintln!("{e}"),
            Ok(mut val) => {
                while let Some(chunk) = val.next().await {
                    if let Ok(chunk) = chunk {
                        println!("{}", String::from_utf8_lossy(&chunk))
                    }

                }
            }
        }
    }
}