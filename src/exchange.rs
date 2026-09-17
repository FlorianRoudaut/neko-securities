use prost::Message;
use serde::{Deserialize, Serialize};
use neko_tech_persistence::{Key, PersistenceError, Persisted};
use crate::proto::{exchange_to_proto, proto_to_exchange, ExchangeList};

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
        ExchangeList { exchanges: items.iter().map(exchange_to_proto).collect() }.encode_to_vec()
    }

    fn from_proto_bytes(bytes: &[u8]) -> Result<Vec<Exchange>, PersistenceError> {
        ExchangeList::decode(bytes)
            .map(|l| l.exchanges.into_iter().map(proto_to_exchange).collect())
            .map_err(|e| PersistenceError::StorageError(e.to_string()))
    }
}
