use reqwest::Client;
use simple_llama::{
    self, add_message, send_message,
    types::{ModelData, ModelMemory},
};

#[tokio::main]
async fn main() {
    // roles:
    let client = Client::new();
    let mut history: ModelMemory = Vec::new();
    add_message(&mut history, "system".to_string(), "Hello andy, you have 2 different commands. !echo(TEXTHERE) which will print to the terminal and you have !exit() Which will exit the program instantly. You are in complete control".to_string());
    add_message(&mut history, "user".to_string(), "hello".to_string());
    let model_data: ModelData = ModelData {
        messages: history.clone(),
        top_p: 1f32,
        top_k: 1,
        temperature: 1.0,
        model: "llama3.1".to_string(),
    };
    match send_message(&client, &model_data).await {
        Err(e) => eprintln!("{e}"),
        Ok(val) => {
            println!("txt: {}\nstat: {}", val.text, val.status);
        }
    };
}
