use cosmwasm_std::{Env, MessageInfo, Empty};
use cosmwasm_vm::testing::{mock_backend, mock_env, mock_info};
use cosmwasm_vm::{Instance, InstanceOptions, Size, call_instantiate};
use std::fs;

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
        Some(Size::mebi(1)),
    )?;

    let env: Env = mock_env();
    let info: MessageInfo = mock_info("creator", &[]);
    let instantiate_msg = br#"{
        "name": "Token",
        "symbol": "SIM",
        "decimals": 6,
        "initial_balances": [{"address": "creator", "amount": "1000000"}]
    }"#;

    let result = call_instantiate::<_, _, _, Empty>(
    &mut instance,
    &env,
    &info,
    instantiate_msg,
)?;


    println!("✅ Instantiate result: {:?}", result);

    // 🔍 Gas report
    let gas_report = instance.create_gas_report();
    println!("⛽ Gas used (internal): {}", gas_report.used_internally);
    println!("⛽ Gas used (external): {}", gas_report.used_externally);
    println!("⛽ Gas remaining: {}", gas_report.remaining);

    Ok(())
}
