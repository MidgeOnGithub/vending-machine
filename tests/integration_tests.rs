use vending_machine::vending::{
    VendingItem,
    cashier::{
        Coins,
        process_purchase,
        NICKEL, DIME, QUARTER, DOLLAR
    },
};

#[test]
fn processes_purchases_as_expected() {
    let item = VendingItem::new("Dummy Item", 125);  // 1.25
    let needs_change: Coins = vec![DOLLAR; 2];       // 2.00
    let insufficient: Coins = vec![DIME; 5];         // 0.50
    let inefficient_but_exact: Coins = vec![         // 1.25
        QUARTER, DIME, NICKEL, NICKEL, NICKEL, QUARTER, QUARTER, QUARTER
    ];
    assert_eq!(process_purchase(needs_change, &item).unwrap(), vec![QUARTER; 3]);
    assert_eq!(process_purchase(insufficient, &item), None);
    assert_eq!(process_purchase(inefficient_but_exact, &item), None);
}

