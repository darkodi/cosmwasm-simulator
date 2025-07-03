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

#[test]
fn test_forked_counter_increment() {
    let count =
        fetch_counter_value("osmo1rhgafruqsszh76trpl8zfaayudzw0q3ndtm9g0qmw5acktlxfngswxu73n");
    let instance = forked_counter_instance(
        count,
        "osmo1deadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
        "artifacts/cw_tpl_osmosis.wasm",
    );
    simulate_increment_and_assert(instance, count);
}
