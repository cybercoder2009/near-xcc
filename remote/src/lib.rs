use near_sdk::{env, near_bindgen, BorshStorageKey, PanicOnDefault};
use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};
use near_sdk::collections::{LookupMap};

use common::{refund};

#[derive(BorshSerialize, BorshStorageKey, Debug)]
pub enum StorageKey {

    R
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {

    kv: LookupMap<u8, u8>,
}

#[near_bindgen]
impl Contract {

    #[init]
    pub fn new(
    ) -> Self {
        env::log_str(&format!("[remote.new] prepaid_gas={:?}", env::prepaid_gas()));
        Self {
            kv: LookupMap::new(StorageKey::R),
        }
    }

    #[payable]
    pub fn add(
        &mut self,
        k: u8,
        v: u8
    ) -> u128 {
        let storage_begin: u64 = env::storage_usage();
        self.kv.insert(&k, &v);
        refund(env::storage_usage() - storage_begin)
    }
}