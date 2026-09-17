use serde::{Deserialize, Serialize};
use super::security::Security;
use super::persisted::Persisted;
use super::key::Key;

pub const STOCK: &str = "Stock";

#[derive(Serialize, Deserialize)]
pub struct Stock {
    pub key: Key,
    pub ticker: String,
    pub exchange: String,
}

impl Persisted for Stock {
    fn key(&self) -> &Key { &self.key }
    fn persisted_type(&self) -> &str { STOCK }
}

impl Security for Stock {}
