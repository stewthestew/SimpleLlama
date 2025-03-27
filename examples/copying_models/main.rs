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
