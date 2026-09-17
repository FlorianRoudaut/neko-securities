use prost::Message;
use serde::{Deserialize, Serialize};
use super::security::Security;
use neko_tech_persistence::{Key, PersistenceError, Persisted};
use crate::proto::{cash_to_proto, proto_to_cash, CashList};

pub const CASH: &str = "Cash";

#[derive(Serialize, Deserialize)]
pub struct Cash {
    pub key: Key,
}

impl Persisted for Cash {
    fn key(&self) -> &Key { &self.key }
    fn persisted_type() -> &'static str { CASH }

    fn to_proto_bytes(items: &[Cash]) -> Vec<u8> {
        CashList { cashes: items.iter().map(cash_to_proto).collect() }.encode_to_vec()
    }

    fn from_proto_bytes(bytes: &[u8]) -> Result<Vec<Cash>, PersistenceError> {
        CashList::decode(bytes)
            .map(|l| l.cashes.into_iter().map(proto_to_cash).collect())
            .map_err(|e| PersistenceError::StorageError(e.to_string()))
    }
}

impl Security for Cash {}
