use crate::balances;

#[test]
fn init_balances() {
    let mut balance = balances::Pallet::new();
    assert_eq!(balance.balance(&"alice".to_string()), 0);
}

#[test]
fn shecking_balances() {
    let mut balances2 = balances::Pallet::new();
    balances2.set_balance(&"alice".to_string(), 100);
    assert_eq!(balances2.balance(&"alice".to_string()), 100);
}

#[test]
fn fail_test() {
    assert_eq!(1, 1);
}
