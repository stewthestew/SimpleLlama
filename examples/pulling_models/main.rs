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
