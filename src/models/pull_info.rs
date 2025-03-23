use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct PullInfo {
    pub model: String,
    pub insecure: bool,
    pub stream: bool,
}
