use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Key {
    pub id: Uuid,
    pub version: u64,
    pub unique_name: String,
}
