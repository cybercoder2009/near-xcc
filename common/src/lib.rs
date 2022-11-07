use near_sdk::{env, Balance, Promise};

pub fn refund(
    storage_diff: u64
) -> u128 {

    let cost: u128 = env::STORAGE_PRICE_PER_BYTE * Balance::from(storage_diff);
    let deposit: u128 = env::attached_deposit();
    assert!(cost <= deposit, "Must attach {} yoctoNEAR to cover storage", cost);
    let amount: Balance = deposit - cost;
    if amount > 1 {
        Promise::new(env::predecessor_account_id()).transfer(amount);
        amount
    } else {
        0
    }
}