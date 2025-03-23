use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CopyInfo {
    pub source: String,
    pub destination: String,
}
