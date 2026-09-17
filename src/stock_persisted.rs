use prost::Message;
use neko_tech_persistence::{Key, PersistenceError, Persisted};
use crate::stock::STOCK;
use crate::Stock;
use crate::proto::{stock_to_proto, proto_to_stock, StockList};

impl Persisted for Stock {
    fn key(&self) -> &Key { &self.key }
    fn persisted_type() -> &'static str { STOCK }

    fn to_proto_bytes(items: &[Stock]) -> Vec<u8> {
        StockList { stocks: items.iter().map(stock_to_proto).collect() }.encode_to_vec()
    }

    fn from_proto_bytes(bytes: &[u8]) -> Result<Vec<Stock>, PersistenceError> {
        StockList::decode(bytes)
            .map(|l| l.stocks.into_iter().map(proto_to_stock).collect())
            .map_err(|e| PersistenceError::StorageError(e.to_string()))
    }
}
