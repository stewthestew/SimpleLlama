#![cfg(feature = "stream")]
use reqwest::Client;
use bytes::Bytes;
use crate::models::options::ModelOptions;
pub async fn send_message_stream(data: &ModelOptions, url: &str) -> Result<impl tokio_stream::Stream<Item = Result<Bytes, reqwest::Error>>, reqwest::Error> {
    let client = Client::new();
    
    let resp = client.post(url).json(&data).send().await?;
    Ok(resp.bytes_stream())
}