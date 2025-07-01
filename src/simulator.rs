use cosmwasm_std::Empty;
use cosmwasm_vm::testing::{mock_backend, mock_env, mock_info, MockApi, MockQuerier, MockStorage};
use cosmwasm_vm::{call_execute, call_instantiate, call_query, Instance, InstanceOptions, Size};
use serde_json::Value;

/// Create and return a pre-initialized Instance
pub fn instantiate_contract(
    creator: &str,
    recipient: &str,
) -> Instance<MockApi, MockStorage, MockQuerier> {
    let wasm = std::fs::read("artifacts/cw20_base.wasm").expect("WASM file not found");
    let backend = mock_backend(&[]);
    let options = InstanceOptions {
        gas_limit: 100_000_000_000,
        print_debug: true,
    };

    let mut instance = Instance::from_code(&wasm, backend, options, Some(Size::mebi(16)))
        .expect("Instance creation failed");

    let env = mock_env();
    let info = mock_info(creator, &[]);
    let instantiate_msg = format!(
        r#"{{
        "name": "MyToken",
        "symbol": "MTK",
        "decimals": 6,
        "initial_balances": [
            {{ "address": "{}", "amount": "1000000" }},
            {{ "address": "{}", "amount": "0" }}
        ]
    }}"#,
        creator, recipient
    );

    call_instantiate::<_, _, _, Empty>(&mut instance, &env, &info, instantiate_msg.as_bytes())
        .expect("instantiate failed");

    instance
}

/// Simulate a CW20 `transfer` execution
pub fn simulate_transfer(
    instance: &mut Instance<MockApi, MockStorage, MockQuerier>,
    from: &str,
    to: &str,
    amount: &str,
) {
    let env = mock_env();
    let info = mock_info(from, &[]);
    let msg = format!(
        r#"{{
        "transfer": {{
            "recipient": "{}",
            "amount": "{}"
        }}
    }}"#,
        to, amount
    );

    let res = call_execute::<_, _, _, Empty>(instance, &env, &info, msg.as_bytes())
        .expect("transfer failed");

    println!("✅ Execute result: {:?}", res);
}

/// Simulate a CW20 balance query and return balance as u128
pub fn simulate_query_balance(
    instance: &mut Instance<MockApi, MockStorage, MockQuerier>,
    address: &str,
) -> u128 {
    let env = mock_env();
    let msg = format!(
        r#"{{
        "balance": {{
            "address": "{}"
        }}
    }}"#,
        address
    );

    let result = call_query::<_, _, _>(instance, &env, msg.as_bytes()).expect("query failed");
    let binary = result.unwrap(); // unwrap ContractResult

    let parsed: Value =
        serde_json::from_slice(binary.as_slice()).expect("failed to parse JSON balance");
    let balance_str = parsed["balance"]
        .as_str()
        .expect("missing balance field");

    let balance = balance_str
        .parse::<u128>()
        .expect("balance is not a number");

    println!("🔍 Balance for {:<42}: {}", address, balance);
    balance
}
