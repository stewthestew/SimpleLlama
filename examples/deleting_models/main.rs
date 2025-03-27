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
