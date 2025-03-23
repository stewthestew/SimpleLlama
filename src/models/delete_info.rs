use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DeleteInfo {
    pub model: String,
}
