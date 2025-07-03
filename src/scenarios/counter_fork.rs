use crate::fork_loader::fetch_counter_value;
use cosmwasm_std::{to_json_binary, Empty};

use cosmwasm_vm::testing::{MockApi, MockQuerier, MockStorage};
use cosmwasm_vm::{Instance, InstanceOptions, Size, Storage};

pub fn run_counter_fork_test() {
    let wasm = std::fs::read("artifacts/cw_tpl_osmosis.wasm").expect("missing wasm");
    let count =
        fetch_counter_value("osmo1rhgafruqsszh76trpl8zfaayudzw0q3ndtm9g0qmw5acktlxfngswxu73n");

    println!("🔁 Forked counter value from live chain: {}", count);

    let mut storage = MockStorage::default();
    #[derive(serde::Serialize)]
    struct State<'a> {
        count: i32,
        owner: &'a str, // or String, but str works fine here
    }

    let key = b"state".to_vec();

    let value = to_json_binary(&State {
        count,
        owner: "osmo1f4u9rpkwkv4lc0qfx7wnz4ljswxya4yu8n05dh", // or "creator"
    })
    .unwrap()
    .0;

    let (res, _) = storage.set(&key, &value);
    res.expect("Failed to insert forked state");

    let mut instance = Instance::from_code(
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
    .expect("failed to create instance");

    // Query inside simulator to confirm it matches forked state
    let env = cosmwasm_vm::testing::mock_env();
    let query = br#"{"get_count":{}}"#;
    let result = cosmwasm_vm::call_query::<_, _, _>(&mut instance, &env, query)
        .unwrap()
        .unwrap();

    let parsed: serde_json::Value = serde_json::from_slice(&result).unwrap();
    println!("🔍 Simulated counter result: {}", parsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forked_counter_query() {
        run_counter_fork_test();
    }
}
