use serde::{Deserialize, Serialize};
use super::security::Security;
use neko_tech_persistence::Key;

pub const STOCK: &str = "Stock";

#[derive(Serialize, Deserialize, Default)]
pub struct Stock {
    pub key: Key,
    pub name: String,
    pub bbg_id: String,
    pub cusip: String,
    pub isin: String,
    pub ibkr_contract_id: String,
    pub exchange_ids: Vec<String>,
}

impl Security for Stock {}
