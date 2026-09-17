use uuid::Uuid;
use crate::{Cash, Exchange, Stock};
use neko_tech_persistence::Key;

mod generated {
    include!(concat!(env!("OUT_DIR"), "/neko.rs"));
}

pub use generated::StockList;
pub use generated::CashList;
pub use generated::ExchangeList;

pub fn stock_to_proto(s: &Stock) -> generated::Stock {
    generated::Stock {
        key: Some(key_to_proto(&s.key)),
        name: s.name.clone(),
        bbg_id: s.bbg_id.clone(),
        cusip: s.cusip.clone(),
        isin: s.isin.clone(),
        ibkr_contract_id: s.ibkr_contract_id.clone(),
        exchange_ids: s.exchange_ids.clone(),
    }
}

pub fn proto_to_stock(p: generated::Stock) -> Stock {
    Stock {
        key: proto_to_key(p.key.unwrap_or_default()),
        name: p.name,
        bbg_id: p.bbg_id,
        cusip: p.cusip,
        isin: p.isin,
        ibkr_contract_id: p.ibkr_contract_id,
        exchange_ids: p.exchange_ids,
    }
}

fn key_to_proto(k: &Key) -> generated::Key {
    generated::Key {
        id: k.id.to_string(),
        version: k.version,
        unique_name: k.unique_name.clone(),
    }
}

fn proto_to_key(k: generated::Key) -> Key {
    Key {
        id: k.id.parse().unwrap_or_else(|_| Uuid::new_v4()),
        version: k.version,
        unique_name: k.unique_name,
    }
}

pub fn cash_to_proto(c: &Cash) -> generated::Cash {
    generated::Cash { key: Some(key_to_proto(&c.key)) }
}

pub fn proto_to_cash(p: generated::Cash) -> Cash {
    Cash { key: proto_to_key(p.key.unwrap_or_default()) }
}

pub fn exchange_to_proto(e: &Exchange) -> generated::Exchange {
    generated::Exchange {
        key: Some(key_to_proto(&e.key)),
        name: e.name.clone(),
    }
}

pub fn proto_to_exchange(p: generated::Exchange) -> Exchange {
    Exchange {
        key: proto_to_key(p.key.unwrap_or_default()),
        name: p.name,
    }
}
