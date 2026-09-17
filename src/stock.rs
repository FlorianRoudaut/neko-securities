use serde::{Deserialize, Serialize};
use super::security::Security;

pub const STOCK: &str = "Stock";

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Stock {
    pub ticker: String,
    pub exchange: String,
}

impl Security for Stock {
    fn name(&self) -> &str { &self.ticker }
    fn security_type(&self) -> &str { STOCK }
}
