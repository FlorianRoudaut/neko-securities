use serde::{Deserialize, Serialize};
use super::security::Security;
use neko_tech_persistence::{Key, PersistenceError, Persisted};

pub const CASH: &str = "Cash";

#[derive(Serialize, Deserialize)]
pub struct Cash {
    pub key: Key,
}

impl Persisted for Cash {
    fn key(&self) -> &Key { &self.key }
    fn persisted_type() -> &'static str { CASH }

    fn to_proto_bytes(items: &[Cash]) -> Vec<u8> {
        bincode::serialize(items).unwrap()
    }

    fn from_proto_bytes(bytes: &[u8]) -> Result<Vec<Cash>, PersistenceError> {
        bincode::deserialize(bytes)
            .map_err(|e| PersistenceError::StorageError(e.to_string()))
    }
}

impl Security for Cash {}
