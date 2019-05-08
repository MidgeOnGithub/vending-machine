#[cfg(test)]
mod test {
    use crate::vending_machine::{VendingItem, cashier::*};

    #[test]
    fn processes_purchases_as_expected() {
        let item = VendingItem::new("Dummy Item", 125);
        assert_eq!(process_purchase(vec![DOLLAR; 2], &item).unwrap(), vec![QUARTER; 3]);
        assert_eq!(process_purchase(vec![DIME; 2], &item), None);
    }
}

