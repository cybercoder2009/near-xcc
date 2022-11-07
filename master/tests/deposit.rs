#[cfg(all(test, not(target_arch = "wasm32")))]
mod deposit {

    use serde_json::json;
    use near_workspaces::{sandbox, Worker, Contract, Account};
    use near_workspaces::network::{Sandbox};
    use near_workspaces::result::{ExecutionFinalResult};

    async fn log_and_set(
        account_master: &Account, account_user: &Account,
        balance_master: u128, balance_user: u128,
    )  -> anyhow::Result<(u128, u128)> {
        println!("[account_master] id={:34} balance={:28} diff={:28}",
            account_master.id(), account_master.view_account().await?.balance, account_master.view_account().await?.balance as i128 - balance_master as i128);
        println!("  [account_user] id={:34} balance={:28} diff={:28}\r\n",
            account_user.id(), account_user.view_account().await?.balance, account_user.view_account().await?.balance as i128 - balance_user as i128);
        Ok((
            account_master.view_account().await?.balance,
            account_user.view_account().await?.balance,
        ))
    }

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {

        // env
        let worker: Worker<Sandbox> = sandbox().await?;
        let wasm_master: Vec<u8> = std::fs::read("../target/wasm32-unknown-unknown/release/master.wasm")?;
        let account_master: Account = worker.dev_create_account().await?;
        let account_remote: Account = worker.dev_create_account().await?;
        let account_user: Account = worker.dev_create_account().await?;

        let mut balance_master: u128 = account_master.view_account().await?.balance;
        let mut balance_user: u128 = account_user.view_account().await?.balance;
        (balance_master, balance_user) = log_and_set(
            &account_master, &account_user,
            balance_master, balance_user,
        ).await?;

        // deploy     
        let _contract_master: Contract = account_master.deploy(&wasm_master).await?.into_result().unwrap();
        (balance_master, balance_user) = log_and_set(
            &account_master, &account_user,
            balance_master, balance_user,
        ).await?;
        
        // init master
        let _result: ExecutionFinalResult = account_master
        .call(account_master.id(), "new")
        .args_json(json!({
            "remote": account_remote.id(),
        }))
        .transact()
        .await?;
        // println!("[contract_master.new] {:?}\r\n", _result.into_result());
        (balance_master, balance_user) = log_and_set(
            &account_master, &account_user,
            balance_master, balance_user,
        ).await?;

        // call add
        let result = account_user
        .call(account_master.id(), "deposit")
        .args_json(json!({}))
        // .deposit(near_sdk::ONE_NEAR)
        .deposit(100)
        // .gas(5_302_247_371_813)
        .gas(5_320_767_376_938)
        .transact()
        .await?;
        println!("[contract_master.deposit] {:#?}\r\n", result);
        log_and_set(
            &account_master, &account_user,
            balance_master, balance_user,
        ).await?;

        Ok(())
    }
}