use crate::fork_loader::fetch_counter_value;
use crate::scenarios::helpers::{
    forked_counter_instance, query_count, simulate_increment_and_assert,
};

#[test]
fn test_forked_counter_query() {
    let count =
        fetch_counter_value("osmo1rhgafruqsszh76trpl8zfaayudzw0q3ndtm9g0qmw5acktlxfngswxu73n");
    println!("🔁 Forked counter value from live chain: {}", count);

    let mut instance = forked_counter_instance(
        count,
        "osmo1deadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
        "artifacts/cw_tpl_osmosis.wasm",
    );
    let result = query_count(&mut instance);
    println!("🔍 Simulated counter result: {}", result);
    assert_eq!(result, count);
}

/* #[test]
fn test_forked_counter_increment() {
    let count =
        fetch_counter_value("osmo1rhgafruqsszh76trpl8zfaayudzw0q3ndtm9g0qmw5acktlxfngswxu73n");
    let instance = forked_counter_instance(
        count,
        "osmo1deadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
        "artifacts/cw_tpl_osmosis.wasm",
    );
    simulate_increment_and_assert(instance, count);
} */

#[test]
fn test_forked_counter_exec() {
    use std::fs;
    use crate::fork_loader::fetch_counter_value;
    use crate::scenarios::helpers::{forked_counter_instance, simulate_increment_and_assert, simulate_reset_and_assert};

    // Read JSON input
    let msg_str = fs::read_to_string("simulations/exec_msg.json").expect("Failed to read input msg");
    let msg: serde_json::Value = serde_json::from_str(&msg_str).expect("Invalid JSON");

    // Fetch live counter state
    let before = fetch_counter_value("osmo1rhgafruqsszh76trpl8zfaayudzw0q3ndtm9g0qmw5acktlxfngswxu73n");

    let instance = forked_counter_instance(
        before,
        "osmo1deadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
        "artifacts/cw_tpl_osmosis.wasm",
    );

    // Dynamically decide what to simulate
    if msg.get("increment").is_some() {
        simulate_increment_and_assert(instance, before);
    } else if let Some(reset) = msg.get("reset") {
        let new_count = reset.get("count").and_then(|v| v.as_i64()).expect("Missing count");
        simulate_reset_and_assert(instance, before, new_count as i32);
    } else {
        panic!("Unsupported exec message: {:?}", msg);
    }
}


