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

    // ✅ Write to expected output file
    use std::fs;
    use std::env;
    use std::path::PathBuf;

    let output_path = PathBuf::from(
        env::var("SIMULATION_QUERY_OUTPUT_PATH")
            .unwrap_or_else(|_| "frontend/public/simulations/cw_tpl_osmosis/query/result.json".to_string()),
    );

    let json = serde_json::json!({
        "count": result
    });

    fs::create_dir_all(output_path.parent().unwrap()).unwrap();
    fs::write(output_path, serde_json::to_string_pretty(&json).unwrap()).unwrap();
}

#[test]
fn test_forked_counter_exec() {
    use std::fs;
    use crate::scenarios::helpers::{
        forked_counter_instance, simulate_increment_and_assert, simulate_reset_and_assert,
    };
    use crate::output::SimulationResult;

    let input_path = std::env::var("SIMULATION_INPUT_PATH")
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

    // Fetch counter value from chain
    let before = crate::fork_loader::fetch_counter_value(
        "osmo1rhgafruqsszh76trpl8zfaayudzw0q3ndtm9g0qmw5acktlxfngswxu73n",
    );

    let instance = forked_counter_instance(
        before,
        "osmo1deadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
        "artifacts/cw_tpl_osmosis.wasm",
    );

    let report: SimulationResult = if msg.get("increment").is_some() {
        simulate_increment_and_assert(instance, before)
    } else if let Some(reset) = msg.get("reset") {
        let new_count = reset
            .get("count")
            .and_then(|v| v.as_i64())
            .expect("Missing count in reset");
        simulate_reset_and_assert(instance, before, new_count as i32)
    } else {
        panic!("Unsupported exec message: {:?}", msg);
    };

    report
        .write_to_file(output_path)
        .expect("Failed to write output file");
}

