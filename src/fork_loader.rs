use serde::Deserialize;
use std::process::Command;

/// Result format for {"count": <number>} response
#[derive(Debug, Deserialize)]
pub struct CountResponse {
    pub count: i32,
}

/// Use `osmosisd` CLI to fetch smart query result from live contract
pub fn fetch_counter_value(contract_addr: &str) -> i32 {
    let query = r#"{"get_count":{}}"#;

    let output = Command::new("osmosisd")
        .args([
            "query",
            "wasm",
            "contract-state",
            "smart",
            contract_addr,
            query,
            "--output",
            "json",
        ])
        .output()
        .expect("Failed to run osmosisd smart query");

    if !output.status.success() {
        panic!(
            "osmosisd error:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[derive(Deserialize)]
    struct OuterWrapper {
        data: CountResponse,
    }

    let parsed: OuterWrapper =
        serde_json::from_slice(&output.stdout).expect("Failed to parse count response");

    parsed.data.count
}
