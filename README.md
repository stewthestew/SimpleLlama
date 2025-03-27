# Simple Llama
Simple Llama is an Ollama wrapper for Rust.


## Dependencies
`openssl`

`ollama`

If there are any more dependencies this crate requires, then make an issue.

## Installation
Add this to your Cargo.toml file
```toml
[dependencies]
simple_llama_rs = "1.0.4"
```
or
```toml
[dependencies]
simple_llama_rs = { git = "https://github.com/stewthestew/SimpleLlama", branch = "main"}
```

### Examples

#### Without streaming
```rust
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
```

#### With streaming
```rust
use std::{
    io::{Write, stdin, stdout},
    process::exit,
};

use simple_llama_rs::{DEFAULT_URL, ModelMemory, ModelOptions, add_message, send_message_stream};
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
```

#### Pulling a model
```rust
use simple_llama_rs::{DEFAULT_PULL_URL, PullInfo, pull_model};

#[tokio::main]
async fn main() {
    match pull_model(
        &PullInfo{
            model: "llama3.1".to_string(),
            insecure: false,
            stream: false,
        },
        DEFAULT_PULL_URL,
    )
    .await
    {
        Err(e) => {
            eprintln!("Error happened with deleting the model: {e}"); // This will handle reqwest errors not ollama.
        }
        Ok(val) => {
            println!("{} {}", val.status_code, val.response)
        }
    };
}
```

#### Deleting a model
```rust
use simple_llama_rs::{DEFAULT_DELETE_URL, DeleteInfo, delete_model};

#[tokio::main]
async fn main() {
    match delete_model(
        &DeleteInfo {
            model: "myllama".to_string(),
        },
        DEFAULT_DELETE_URL,
    )
    .await
    {
        Err(e) => {
            eprintln!("Error happened with deleting the model: {e}"); // This will handle reqwest errors not ollama.
        }
        Ok(val) => {
            println!("{} {}", val.status_code, val.response)
        }
    };
}
```

#### Copying a model
```rust
use simple_llama_rs::{CopyInfo, DEFAULT_COPY_URL, copy_model};

#[tokio::main]
async fn main() {
    match copy_model(
        &CopyInfo {
            source: "llama3.1".to_string(),
            destination: "myllama".to_string(),
        },
        DEFAULT_COPY_URL,
    )
    .await
    {
        Err(e) => {
            eprintln!("Error happened with copying the model: {e}"); // This will handle reqwest errors not ollama.
        }
        Ok(val) => {
            println!("{} {}", val.status_code, val.response)
        }
    };
}
```

## Todo
- [ ] Listing models
- [x] Streaming support
- [x] Pulling models
- [x] Deleting models
- [x] Copying models

## Similar crates:
[Mistral.rs](https://github.com/EricLBuehler/mistral.rs)  
[Ollama-rs](https://github.com/pepperoni21/ollama-rs)

## License
This project is Licensed under the MIT License
