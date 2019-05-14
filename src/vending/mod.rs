pub mod cashier;

use crate::vending::cashier::*;
use std::os::raw::c_void;

#[derive(Debug, Clone)]
pub struct VendingItem {
    pub name: String,
    price: u32,  // Interpreted as cents, rather than dollars.
}

impl VendingItem {
    /// Returns a VendingItem.
    pub fn new<S> (name: S, mut price: u32) -> Self where S: Into<String> {
        // Ensure the price is divisible by our smallest coin.
        match price % NICKEL {
            0 => {},
            remainder => price += (NICKEL - remainder)
        }
        Self{name: name.into(), price}
    }

    /// Returns the VendingItem's price.
    pub fn price(self) -> u32 { self.price }
    /// Changes the VendingItem's price.
    pub fn change_price(mut self, price: u32) { self.price = price; }
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

#[cfg(test)]
#[test]
fn new_item_prices_rounded_if_needed() {
    let first = VendingItem::new("Fizzy Water", 125);   // Should not round.
    assert_eq!(125, first.price());
    let second = VendingItem::new("Spicy Potato", 231);  // Should round up.
    assert_eq!(235, second.price());
}