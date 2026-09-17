use serde::{Deserialize, Serialize};
use super::security::Security;
use super::persisted::Persisted;
use super::key::Key;

pub const STOCK: &str = "Stock";

#[derive(Serialize, Deserialize)]
pub struct Stock {
    pub key: Key,
    pub name: String,
    pub bbg_id: String,
    pub cusip: String,
    pub isin: String,
    pub ibkr_contract_id: String,
    pub exchange_ids: Vec<String>,
}

impl Persisted for Stock {
    fn key(&self) -> &Key { &self.key }
    fn persisted_type(&self) -> &str { STOCK }
}

impl Security for Stock {}
