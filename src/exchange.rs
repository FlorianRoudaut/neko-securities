use super::persisted::Persisted;
use super::key::Key;

pub const EXCHANGE: &str = "Exchange";

pub struct Exchange {
    pub key: Key,
    pub name: String,
}

impl Persisted for Exchange {
    fn key(&self) -> &Key { &self.key }
    fn persisted_type(&self) -> &str { EXCHANGE }
}
