use simulator::Simulator;
use std::env;
use std::fs;

mod simulator;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        eprintln!("Usage:\n  simulator <wasm_path> <sender> <instantiate.json> <exec.json> [<query.json>]");
        return;
    }

    let wasm_path = &args[1];
    let sender = &args[2];
    let instantiate_path = &args[3];
    let exec_path = &args[4];
    let query_path = args.get(5);

    let instantiate_msg =
        fs::read_to_string(instantiate_path).expect("Failed to read instantiate.json");
    let exec_msg = fs::read_to_string(exec_path).expect("Failed to read exec.json");

    let mut sim = Simulator::new(wasm_path);
    sim.instantiate(sender, &instantiate_msg);
    sim.execute(sender, &exec_msg);

    if let Some(query_file) = query_path {
        let query_msg = fs::read_to_string(query_file).expect("Failed to read query.json");
        sim.query(&query_msg);
    }
}
