use cosmwasm_std::{to_json_binary, ContractResult, Empty, Response};
use cosmwasm_vm::{
    call_execute_raw, call_query,
    testing::{mock_env, mock_info, MockApi, MockQuerier, MockStorage},
    Instance, InstanceOptions, Size, Storage,
};
use std::fs::read;
#[derive(serde::Serialize)]
pub struct State<'a> {
    pub count: i32,
    pub owner: &'a str,
}

/// Create a forked instance of the counter contract from given count/owner
pub fn forked_counter_instance(
    count: i32,
    owner: &str,
    wasm_path: &str,
) -> Instance<MockApi, MockStorage, MockQuerier<Empty>> {
    let wasm = read(wasm_path).expect("Missing wasm file");
    let mut storage = MockStorage::default();

    let key = b"state".to_vec();
    let value = to_json_binary(&State { count, owner }).unwrap().0;
    let (res, _) = storage.set(&key, &value);
    res.expect("Failed to insert state");

    Instance::from_code(
        &wasm,
        cosmwasm_vm::Backend {
            api: MockApi::default(),
            storage,
            querier: MockQuerier::<Empty>::new(&[]),
        },
        InstanceOptions {
            gas_limit: 100_000_000_000,
            print_debug: true,
        },
        Some(Size::mebi(16)),
    )
    .expect("Failed to create instance")
}

/// Query get_count from instance
pub fn query_count(instance: &mut Instance<MockApi, MockStorage, MockQuerier<Empty>>) -> i32 {
    let env = mock_env();
    let query = br#"{"get_count":{}}"#;
    let result = call_query::<_, _, _>(instance, &env, query).expect("Query failed");

    let parsed: serde_json::Value = match result {
        cosmwasm_std::ContractResult::Ok(bin) => {
            serde_json::from_slice(&bin).expect("Failed to parse query response")
        }
        cosmwasm_std::ContractResult::Err(e) => {
            panic!("Query failed inside contract: {e}")
        }
    };
    parsed["count"].as_i64().unwrap() as i32
}

use crate::output::{ExecuteResult, SimulationResult};

pub fn simulate_increment_and_assert(
    mut instance: Instance<MockApi, MockStorage, MockQuerier<Empty>>,
    before: i32,
) {
    let env = mock_env();
    let info = mock_info("someone", &[]);
    let exec_msg = br#"{ "increment": {} }"#;

    // 🔧 Encode inputs
    let env_bin = cosmwasm_std::to_json_vec(&env).unwrap();
    let info_bin = cosmwasm_std::to_json_vec(&info).unwrap();

    // ⚙️ Run raw execution to capture gas usage
    let raw =
        call_execute_raw(&mut instance, &env_bin, &info_bin, exec_msg).expect("Execution failed");
    let gas_report = instance.create_gas_report();

    // 🧠 Deserialize execution result
    let exec_result: ContractResult<Response> =
        cosmwasm_std::from_json(&raw).expect("Deserialization failed");

    println!("⚙️  Execute result: {:?}", exec_result);
    println!("⛽️  Gas limit: {}", gas_report.limit);
    println!("⛽️  Gas remaining: {}", gas_report.remaining);
    println!("⛽️  Gas used externally: {}", gas_report.used_externally);
    println!("⛽️  Gas used internally: {}", gas_report.used_internally);

    // 🔍 Run query to check new counter value
    let after = query_count(&mut instance);
    println!("🔍 After increment: {}", after);
    assert_eq!(after, before + 1, "Counter did not increment correctly");

    // 🧾 Prepare and write SimulationResult
    let report = SimulationResult {
        wasm_path: "artifacts/cw_tpl_osmosis.wasm".to_string(),
        sender: info.sender.to_string(),
        action: "increment".to_string(),
        query_before: serde_json::json!({ "count": before }),
        query_after: Some(serde_json::json!({ "count": after })),
        execute_result: match exec_result {
            ContractResult::Ok(resp) => Some(ExecuteResult {
                gas_used: gas_report.used_internally + gas_report.used_externally,
                attributes: resp
                    .attributes
                    .into_iter()
                    .map(|attr| (attr.key, attr.value))
                    .collect(),
                messages: resp.messages.len(),
            }),
            ContractResult::Err(err_msg) => {
                println!("❌ Contract execution error: {}", err_msg);
                None
            }
        },
    };

    if let Err(e) = report.write_to_file("simulations/latest_counter_increment.json") {
        panic!("Failed to create simulation result file: {e}");
    }
}
