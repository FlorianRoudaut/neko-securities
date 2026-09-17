use uuid::Uuid;
use crate::{Stock, Key};

mod generated {
    include!(concat!(env!("OUT_DIR"), "/neko.rs"));
}

pub use generated::StockList;

pub fn stock_to_proto(s: &Stock) -> generated::Stock {
    generated::Stock {
        key: Some(generated::Key {
            id: s.key.id.to_string(),
            version: s.key.version,
            unique_name: s.key.unique_name.clone(),
        }),
        name: s.name.clone(),
        bbg_id: s.bbg_id.clone(),
        cusip: s.cusip.clone(),
        isin: s.isin.clone(),
        ibkr_contract_id: s.ibkr_contract_id.clone(),
        exchange_ids: s.exchange_ids.clone(),
    }
}

pub fn proto_to_stock(p: generated::Stock) -> Stock {
    let key = p.key.unwrap_or_default();
    Stock {
        key: Key {
            id: key.id.parse().unwrap_or_else(|_| Uuid::new_v4()),
            version: key.version,
            unique_name: key.unique_name,
        },
        name: p.name,
        bbg_id: p.bbg_id,
        cusip: p.cusip,
        isin: p.isin,
        ibkr_contract_id: p.ibkr_contract_id,
        exchange_ids: p.exchange_ids,
    }
}
