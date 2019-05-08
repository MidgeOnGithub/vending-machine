pub mod cashier;

#[derive(Debug, Clone)]
pub struct VendingItem {
    name: String,
    price: u32,  // Interpreted as cents, rather than dollars.
}

impl VendingItem {
    pub fn new(name: &str, price: u32) -> Self {
        Self{name: name.to_string(), price}
    }
}

pub type MoneyBox = u32;

// These may prove handy later on.
//
//type Row = Vec<VendingItem>;
//type Machine = Vec<Row>;
//
//pub enum VendingEvents {
//    EmptyMoneyBox,
//    ProcessPurchaseAttempt,
//    StockItems
//}