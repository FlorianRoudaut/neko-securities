use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Key {
    pub id: String,
    pub version: u64,
    pub unique_name: String,
}
