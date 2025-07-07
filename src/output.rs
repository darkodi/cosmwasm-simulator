use serde::Serialize;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Debug)]
pub struct SimulationResult {
    pub wasm_path: String,
    pub sender: String,
    pub action: String,
    pub query_before: serde_json::Value,
    pub query_after: Option<serde_json::Value>,
    pub execute_result: Option<ExecuteResult>,
}

#[derive(Serialize, Debug)]
pub struct ExecuteResult {
    pub gas_used: u64,
    pub attributes: Vec<(String, String)>,
    pub messages: usize,
}

impl SimulationResult {
    /// Write the simulation result to a file, creating parent dirs if necessary
    pub fn write_to_file(&self, path: &str) -> io::Result<()> {
        let path_obj = Path::new(path);

        // Ensure parent directory exists
        if let Some(parent) = path_obj.parent() {
            fs::create_dir_all(parent)?; // Creates nested dirs if needed
        }

        // Serialize and write JSON
        let json = serde_json::to_string_pretty(self)?;
        let mut file = File::create(path_obj)?;
        file.write_all(json.as_bytes())?;
        println!("📄 Simulation written to {path}");
        Ok(())
    }
}
