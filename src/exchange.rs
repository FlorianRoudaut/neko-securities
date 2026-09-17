use serde::{Deserialize, Serialize};
use neko_tech_persistence::{Key, PersistenceError, Persisted};

pub const EXCHANGE: &str = "Exchange";

#[derive(Serialize, Deserialize)]
pub struct Exchange {
    pub key: Key,
    pub name: String,
}

impl Persisted for Exchange {
    fn key(&self) -> &Key { &self.key }
    fn persisted_type() -> &'static str { EXCHANGE }

    fn to_proto_bytes(items: &[Exchange]) -> Vec<u8> {
        bincode::serialize(items).unwrap()
    }

    fn from_proto_bytes(bytes: &[u8]) -> Result<Vec<Exchange>, PersistenceError> {
        bincode::deserialize(bytes)
            .map_err(|e| PersistenceError::StorageError(e.to_string()))
    }
}
