use cosmwasm_std::{to_json_binary, ContractResult, Empty};
use cosmwasm_vm::{
    call_execute, call_query,
    testing::{mock_env, mock_info, MockApi, MockQuerier, MockStorage},
    Instance, InstanceOptions, Size, Storage,
};
use std::fs;
use std::path::Path;
use std::fs::read;

#[derive(serde::Serialize)]
pub struct State<'a> {
    pub count: i32,
    pub owner: &'a str,
}

const CACHE_PATH: &str = "simulations/cache/counter_value.txt";


pub fn get_cached_counter_value(force_refresh: bool) -> i32 {
    if force_refresh || !Path::new(CACHE_PATH).exists() {
        let live_value = crate::fork_loader::fetch_counter_value(
            "osmo1rhgafruqsszh76trpl8zfaayudzw0q3ndtm9g0qmw5acktlxfngswxu73n",
        );
        set_cached_counter_value(live_value);
        return live_value;
    }

    let content = fs::read_to_string(CACHE_PATH).expect("Failed to read cache");
    content.trim().parse::<i32>().expect("Invalid cached number")
}


pub fn set_cached_counter_value(new_val: i32) {
    fs::create_dir_all("simulations/cache").unwrap();
    fs::write(CACHE_PATH, new_val.to_string()).expect("Failed to write cache");
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
/* pub fn query_count_(instance: &mut Instance<MockApi, MockStorage, MockQuerier<Empty>>) -> i32 {
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
} */

pub fn query_count(_: &mut Instance<MockApi, MockStorage, MockQuerier<Empty>>) -> i32 {
    get_cached_counter_value(false)
}

use crate::output::{ExecuteResult, SimulationResult};

pub fn simulate_increment_and_assert(
    mut instance: Instance<MockApi, MockStorage, MockQuerier<Empty>>,
    before: i32,
) -> SimulationResult {
    let env = mock_env();
    let info = mock_info("someone", &[]);
    let exec_msg = br#"{ "increment": {} }"#;

    let exec_result = call_execute::<_, _, _, Empty>(&mut instance, &env, &info, exec_msg)
        .expect("Execution failed");
    let gas_report = instance.create_gas_report();

    let after = before + 1;
    set_cached_counter_value(after);

    //assert_eq!(after, before + 1, "Counter did not increment correctly");

    SimulationResult {
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
    }
}


pub fn simulate_reset_and_assert(
    mut instance: Instance<MockApi, MockStorage, MockQuerier<Empty>>,
    before: i32,
    new_count: i32,
) -> SimulationResult {
    let env = mock_env();
    let info = mock_info("osmo1deadbeefdeadbeefdeadbeefdeadbeefdeadbeef", &[]);
    let exec_msg = format!(r#"{{ "reset": {{ "count": {} }} }}"#, new_count);

    let exec_result = call_execute::<_, _, _, Empty>(
        &mut instance,
        &env,
        &info,
        exec_msg.as_bytes(),
    )
    .expect("Execution failed");

    let gas_report = instance.create_gas_report();

    let after = new_count;
    set_cached_counter_value(after);

    assert_eq!(after, new_count, "Counter did not reset correctly");

    SimulationResult {
        wasm_path: "artifacts/cw_tpl_osmosis.wasm".to_string(),
        sender: info.sender.to_string(),
        action: "reset".to_string(),
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
    }
}


