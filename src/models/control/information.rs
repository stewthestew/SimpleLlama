use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DeleteInfo {
    pub model: String,
}

#[derive(Deserialize, Serialize)]
pub struct CopyInfo {
    pub source: String,
    pub destination: String,
}

#[derive(Deserialize, Serialize)]
pub struct PullInfo {
    pub model: String,
    pub insecure: bool,
    pub stream: bool,
}