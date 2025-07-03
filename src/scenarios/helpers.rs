use cosmwasm_std::{to_json_binary, Empty};
use cosmwasm_vm::{
    call_execute, call_query,
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

/// Simulate `{"increment":{}}` and assert count increased
pub fn simulate_increment_and_assert(
    mut instance: Instance<MockApi, MockStorage, MockQuerier<Empty>>,
    before: i32,
) {
    let env = mock_env();
    let info = mock_info("someone", &[]);
    let exec_msg = br#"{ "increment": {} }"#;

    let res = call_execute::<_, _, _, Empty>(&mut instance, &env, &info, exec_msg)
        .expect("Execution failed");

    println!("⚙️  Execute result: {:?}", res);

    let after = query_count(&mut instance);
    println!("🔍 After increment: {}", after);
    assert_eq!(after, before + 1, "Counter did not increment correctly");
}
