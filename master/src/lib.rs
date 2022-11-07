use near_sdk::{env, near_bindgen, AccountId, Promise, PromiseError, Gas, PanicOnDefault};
use near_sdk::serde_json::{json};
use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    nonce: u64,
    remote: AccountId,
}

#[near_bindgen]
impl Contract {
    
    #[init]
    pub fn new(
        remote: AccountId
    ) -> Self {
        
        Self {
            nonce: 0,
            remote,
        }
    }

    #[private] // Public - but only callable by env::current_account_id()
    pub fn remote_callback(
        &self,
        #[callback_result] call_result: Result<u128, PromiseError>
    ) {
        env::log_str(&format!("[master.callback] prepaid_gas={:?} storage_usage={}", env::prepaid_gas(), env::storage_usage()));
        match call_result {
            Ok(refund) => env::log_str(&format!("[master.callback] success refund={}", refund)),
            Err(err) => env::log_str(&format!("[master.callback] err={:?}", err)),
        }
    }

    #[payable]
    pub fn remote(
        &mut self,
        k: u8,
        v: u8
    ) -> Promise {

        let id = self.remote.clone();
        let call: Promise = Promise::new(id)
        .function_call(
            "add".to_string(),
            json!({
                "k": k,
                "v": v,
            }).to_string().into_bytes(),
            430000000000000000000,
            Gas(2_816_191_427_277)
        );
        
        call.then(
            Self::ext(env::current_account_id())
            .remote_callback()
        )
    }

    #[payable]
    pub fn deposit_panic(
        &mut self,
    ) {

        // (0)
        // deposit reverse 
        // total gas burnt = 5_479_171_249_614
        // panic!("stop");
        
        // deposit done
        // total gas burnt = 5_320_652_932_938
        self.nonce = 1;

        // (1)
        // deposit reverse
        // total gas burnt = 5_479_175_235_132
        panic!("stop");
    }

    #[payable]
    pub fn deposit(
        &mut self,
    ) {

        // deposit done
        // total gas burnt = 5_320_767_376_938
        self.nonce = 1;
    }
}