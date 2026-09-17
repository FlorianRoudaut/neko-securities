use super::security::Security;
use super::persisted::Persisted;
use super::key::Key;

pub const CASH: &str = "Cash";

pub struct Cash {
    pub key: Key,
}

impl Persisted for Cash {
    fn key(&self) -> &Key { &self.key }
    fn persisted_type(&self) -> &str { CASH }
}

impl Security for Cash {}
