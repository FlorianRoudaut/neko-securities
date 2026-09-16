use crate::currency::Currency;
use crate::security::Security;

pub struct Position {
    pub security: Box<dyn Security>,
    pub quantity: f64,
    pub currency: Currency,
}
