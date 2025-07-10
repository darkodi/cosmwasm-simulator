use std::env;
use std::fs;
use std::path::PathBuf;

use serde_json::json;

use crate::fork_loader::fetch_counter_value;
use crate::output::SimulationResult;
use crate::scenarios::helpers::{
    forked_counter_instance,
    get_cached_counter_value,
    query_count,
    simulate_increment_and_assert,
    simulate_reset_and_assert,
};

#[test]
fn test_forked_counter_query() {
    use std::{env, fs};
    use std::path::PathBuf;
    use crate::scenarios::helpers::{get_cached_counter_value, forked_counter_instance, query_count};
    use serde_json::json;

    let count = get_cached_counter_value(false); // ← ✅ Read cached
    println!("📦 Loaded cached counter value: {}", count);

    let mut instance = forked_counter_instance(
        count,
        "osmo1deadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
        "artifacts/cw_tpl_osmosis.wasm",
    );

    let result = query_count(&mut instance);
    println!("🔍 Simulated counter result: {}", result);

    let output_path = PathBuf::from(
        env::var("SIMULATION_QUERY_OUTPUT_PATH")
            .unwrap_or_else(|_| "frontend/public/simulations/cw_tpl_osmosis/query/result.json".to_string()),
    );

    let json = json!({ "count": result });

    fs::create_dir_all(output_path.parent().unwrap()).unwrap();
    fs::write(output_path, serde_json::to_string_pretty(&json).unwrap()).unwrap();
}


#[test]
fn test_fork_live_state() {
    use crate::fork_loader::fetch_counter_value;
    use crate::scenarios::helpers::set_cached_counter_value;

    let count = fetch_counter_value("osmo1rhgafruqsszh76trpl8zfaayudzw0q3ndtm9g0qmw5acktlxfngswxu73n");
    println!("🌐 Forked live counter value: {}", count);

    set_cached_counter_value(count);
    println!("💾 Cached to counter_value.txt");
}

#[test]
fn test_forked_counter_exec() {
    let input_path = env::var("SIMULATION_INPUT_PATH")
        .unwrap_or_else(|_| "simulations/exec_msg.json".to_string());

    println!("📂 Simulation input path: {}", input_path);
    let input_str = fs::read_to_string(&input_path).expect("Failed to read input msg");
    let input_json: serde_json::Value = serde_json::from_str(&input_str).expect("Invalid JSON");

    let msg = input_json
        .get("msg")
        .expect("❌ Missing `msg` field in input")
        .clone();

    let output_path = input_json
        .get("output_path")
        .and_then(|v| v.as_str())
        .expect("❌ Missing `output_path` field in input");

    println!("✅ Parsed input msg: {}", msg);
    println!("📝 Will write result to: {}", output_path);

    let before = get_cached_counter_value(false);
    println!("🟡 BEFORE cached value: {}", before);

    let instance = forked_counter_instance(
        before,
        "osmo1deadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
        "artifacts/cw_tpl_osmosis.wasm",
    );

    let report: SimulationResult = if msg.get("increment").is_some() {
        simulate_increment_and_assert(instance, before, &msg)
    } else if let Some(reset) = msg.get("reset") {
        let new_count = reset
            .get("count")
            .and_then(|v| v.as_i64())
            .expect("Missing count in reset");
        simulate_reset_and_assert(instance, before, new_count as i32, &msg)
    } else {
        panic!("Unsupported exec message: {:?}", msg);
    };

    println!("🟢 AFTER cached value: {}", get_cached_counter_value(false));

    report
        .write_to_file(output_path)
        .expect("Failed to write output file");
}
