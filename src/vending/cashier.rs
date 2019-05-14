use crate::vending::VendingItem;
use std::fs::read_to_string;

// TODO: Look for a better solution to handle the coins.
//  Originally, it was a struct with an impl that
//  defined only the constants... Not sure if this is better.
pub type Money = u32;

pub const NICKEL: Money = 5;
pub const DIME: Money = 10;
pub const QUARTER: Money = 25;
pub const DOLLAR: Money = 100;

pub type Coins = Vec<Money>;

/// Processes a purchase without checking that the money tendered is sufficient.
pub fn process_purchase(money_given: Coins, item: &VendingItem) -> Option<Coins> {
    change_needed(money_given, item.price)
                  .map(|amount| make_change(amount))
}

/// Returns None if no change is needed, otherwise returns Some(amount).
fn change_needed(tendered: Coins, price: u32) -> Option<u32> {
    let total_given: u32 = tendered.iter().sum();
    // TODO: Search if this can be condensed by another idiom.
    match total_given <= price {
        true => return None,
        false => return Some(total_given - price)
    }
}

// Used for easy iteration over the possible money types.
const COIN_TYPES: [Money; 4] = [DOLLAR, QUARTER, DIME, NICKEL];
/// Returns the most efficient set of coins to make change for an amount.
fn make_change(mut amount: u32) -> Coins {
    // TODO: Search for iterator methods that encompass the actions in the for loop.
    let mut change: Coins = Vec::new();
    for coin in COIN_TYPES.iter() {
        while amount >= *coin {
            amount -= *coin;
            change.push(*coin)
        }
    }
    change
}


#[cfg(test)]
#[test]
fn makes_efficient_change() {
    let efficient_140_cents = vec![DOLLAR, QUARTER, DIME, NICKEL];
    assert_eq!(make_change(140), efficient_140_cents);
    let efficient_40_cents = vec![QUARTER, DIME, NICKEL];
    assert_eq!(make_change(40), efficient_40_cents);

    assert_eq!(make_change(200), vec![DOLLAR; 2]);       // Not 8 quarters.
    assert_eq!(make_change(50), vec![QUARTER; 2]);       // Not 5 dimes.
    assert_eq!(make_change(30), vec![QUARTER, NICKEL]);  // Not 3 dimes.
    assert_eq!(make_change(25), vec![QUARTER]);          // Not 1 quarter + 1 nickel.
    assert_eq!(make_change(10), vec![DIME; 1]);          // Not 2 nickels.
    assert_eq!(make_change(0), Vec::new());
}

#[test]
fn correctly_decides_if_or_what_change_is_needed() {
    assert_eq!(change_needed(vec![DOLLAR; 2], 95).unwrap(), 105);
    assert_eq!(change_needed(vec![DIME; 2], 15).unwrap(), 5);
    assert_eq!(change_needed(vec![QUARTER], 25), None);
    assert_eq!(change_needed(Vec::new(), 25), None);
}
