#[cfg(all(test, not(target_arch = "wasm32")))]
mod xcc {

    use serde_json::json;
    use near_workspaces::{sandbox, Worker, Contract, Account};
    use near_workspaces::network::{Sandbox};
    use near_workspaces::result::{ExecutionFinalResult};
    
    async fn log_and_set(
        account_master: &Account, account_remote: &Account, account_user: &Account,
        balance_master: u128, balance_remote: u128, balance_user: u128,
    )  -> anyhow::Result<(u128, u128, u128)> {
        println!("[account_master] id={:34} balance={:28} diff={:28}",
            account_master.id(), account_master.view_account().await?.balance, account_master.view_account().await?.balance as i128 - balance_master as i128);
        println!("[account_remote] id={:34} balance={:28} diff={:28}",
            account_remote.id(), account_remote.view_account().await?.balance, account_remote.view_account().await?.balance as i128 - balance_remote as i128);
        println!("  [account_user] id={:34} balance={:28} diff={:28}\r\n",
            account_user.id(), account_user.view_account().await?.balance, account_user.view_account().await?.balance as i128 - balance_user as i128);
        Ok((
            account_master.view_account().await?.balance,
            account_remote.view_account().await?.balance,
            account_user.view_account().await?.balance,
        ))
    }

    #[tokio::test]
    async fn test_add() -> anyhow::Result<()> {

        // env
        let worker: Worker<Sandbox> = sandbox().await?;
        let wasm_master: Vec<u8> = std::fs::read("../target/wasm32-unknown-unknown/release/master.wasm")?;
        let wasm_remote: Vec<u8> = std::fs::read("../target/wasm32-unknown-unknown/release/remote.wasm")?;
        let account_master: Account = worker.dev_create_account().await?;
        let account_remote: Account = worker.dev_create_account().await?;
        let account_user: Account = worker.dev_create_account().await?;

        let mut balance_master: u128 = account_master.view_account().await?.balance;
        let mut balance_remote: u128 = account_remote.view_account().await?.balance;
        let mut balance_user: u128 = account_user.view_account().await?.balance;
        (balance_master, balance_remote, balance_user) = log_and_set(
            &account_master, &account_remote, &account_user,
            balance_master, balance_remote, balance_user,
        ).await?;

        // deploy     
        let _contract_master: Contract = account_master.deploy(&wasm_master).await?.into_result().unwrap();
        let _contract_remote: Contract = account_remote.deploy(&wasm_remote).await?.into_result().unwrap();
        (balance_master, balance_remote, balance_user) = log_and_set(
            &account_master, &account_remote, &account_user,
            balance_master, balance_remote, balance_user,
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
        (balance_master, balance_remote, balance_user) = log_and_set(
            &account_master, &account_remote, &account_user,
            balance_master, balance_remote, balance_user,
        ).await?;

        // init remote
        let _result = account_remote
        .call(account_remote.id(), "new")
        .args_json(json!({
        }))
        .transact()
        .await?;
        // println!("[contract_remote.new] {:?}\r\n", _result);
        log_and_set(
            &account_master, &account_remote, &account_user,
            balance_master, balance_remote, balance_user,
        ).await?;

        // call add
        let result = account_user
        .call(account_master.id(), "remote")
        .args_json(json!({
            "k": 0,
            "v": 0,
        }))
        .deposit(322493045481700000000)
        // .gas(300_000_000_000_000)
        .gas(16_367_340_387_808)
        .transact()
        .await?;
        println!("[contract_master.remote] {:#?}\r\n", result);
        log_and_set(
            &account_master, &account_remote, &account_user,
            balance_master, balance_remote, balance_user,
        ).await?;

        Ok(())
    }
}