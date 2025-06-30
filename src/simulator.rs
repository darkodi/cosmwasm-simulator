use std::fs;
use cosmwasm_std::{Env, MessageInfo, Response};
use cosmwasm_vm::testing::{mock_backend, mock_env, mock_info};
use cosmwasm_vm::{Instance, InstanceOptions, Size};

pub fn simulate_instantiate(wasm_path: &str) -> anyhow::Result<()> {
    let wasm = fs::read(wasm_path)?;
    let backend = mock_backend(&[]);
    let options = InstanceOptions {
        gas_limit: 10_000_000_000,
        print_debug: true,
    };

    let mut instance = Instance::from_code(
        &wasm,
        backend,
        options,
        Size::new(1_000_000).unwrap(),
    )?;

    let env: Env = mock_env();
    let info: MessageInfo = mock_info("creator", &[]);
    let instantiate_msg = br#"{
        "name": "Token",
        "symbol": "SIM",
        "decimals": 6,
        "initial_balances": [{"address": "creator", "amount": "1000000"}]
    }"#;

    let result = instance.instantiate(&env, &info, instantiate_msg)?;
    println!("🚀 Instantiate gas used: {}", result.gas_used);
    println!("✅ Response: {:?}", result);

    Ok(())
}
