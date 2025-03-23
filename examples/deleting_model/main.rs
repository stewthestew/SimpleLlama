use simple_llama_rs::{
    common::DEFAULT_DELETE_URL,
    models::{DeleteInfo, delete_model},
};
#[tokio::main]
async fn main() {
    match delete_model(
        &DeleteInfo {
            model: "model_name".to_string(),
        },
        DEFAULT_DELETE_URL,
    )
    .await
    {
        Err(e) => {
            // If it returns a `404 Not Found ` or `404 Page Not Found` then the model doesn't exist.
            eprintln!("Error happened with deleting the model: {e}"); // This will handle reqwest errors not ollama.
        }
        Ok(val) => {
            println!("{} {}", val.status_code, val.response)
        }
    };
}
