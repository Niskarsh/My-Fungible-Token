use std::error::Error;

use near_workspaces::{ compile_project, sandbox, Account, Contract };


pub async fn prepare_dev_env() -> Result<(Vec<Account>, Contract), Box<dyn Error>> {
    let worker = sandbox().await?; // Create Dep Env
    let wasm = compile_project("./").await?; // Compiled project
    // Deploy the project
    let contract = worker.dev_deploy(&wasm).await?;
    let account1 = worker.dev_create_account().await?;
    let account2 = worker.dev_create_account().await?;
    let account3 = worker.dev_create_account().await?;
    Ok((vec![account1, account2, account3], contract))
}