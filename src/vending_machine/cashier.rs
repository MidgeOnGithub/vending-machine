use crate::vending_machine::VendingItem;

// TODO: Look for a better solution to handle the coins.
//  Originally, it was a struct with an impl that
//  defined only the constants... Not sure if this is better.
pub type Money = u32;

pub const NICKEL: Money = 5;
pub const DIME: Money = 10;
pub const QUARTER: Money = 25;
pub const DOLLAR: Money = 100;

pub type Coins = Vec<Money>;

// Used in this file for easy iteration over the possible money types.
const COIN_TYPES: [Money; 4] = [DOLLAR, QUARTER, DIME, NICKEL];

/// Processes a purchase without checking that the money tendered is sufficient.
pub fn process_purchase(money_given: Coins, item: &VendingItem) -> Option<Coins> {
    change_needed(money_given, item.price)
                  .map(|amount| make_change(amount))
}

/// Returns None if no change is needed, otherwise returns the amount as Some.
fn change_needed(tendered: Coins, price: u32) -> Option<u32> {
    let total_given: u32 = tendered.iter().sum();
    // TODO: Search for an idiom that encompasses this if-else.
    if total_given <= price {
        return None;
    } else {
        return Some(total_given - price)
    }
}

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
fn makes_correct_change() {
    // A case with all denominations available.
    let efficient_140_cents = vec![DOLLAR, QUARTER, DIME, NICKEL];
    assert_eq!(make_change(140), efficient_140_cents);
    // A case with all coins available.
    let efficient_40_cents = vec![QUARTER, DIME, NICKEL];
    assert_eq!(make_change(40), efficient_40_cents);
    // Test 0 cents.
    assert_eq!(make_change(0), Vec::new());
    // Check multiples that should be covered by a more valuable denomination.
    assert_eq!(make_change(100), vec![DOLLAR]);                // No quarters.
    assert_eq!(make_change(30), vec![QUARTER, NICKEL]); // Not 3 dimes.
    assert_eq!(make_change(50), vec![QUARTER; 2]);             // No dimes.
    assert_eq!(make_change(20), vec![DIME; 2]);                // No nickels.
    assert_eq!(make_change(10), vec![DIME; 1]);                // No nickels.
}

#[test]
fn correctly_decides_if_or_what_change_is_needed() {
    assert_eq!(change_needed(vec![DOLLAR; 2], 95).unwrap(), 105);
    assert_eq!(change_needed(vec![DIME; 2], 15).unwrap(), 5);
    assert_eq!(change_needed(vec![QUARTER], 25), None);
}
