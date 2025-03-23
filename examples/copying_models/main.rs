use simple_llama_rs::{
    common::DEFAULT_COPY_URL,
    models::{CopyInfo, copy_model},
};
#[tokio::main]
async fn main() {
    match copy_model(
        &CopyInfo {
            source: "model_name".to_string(),
            destination: "model_copy_name".to_string(),
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
