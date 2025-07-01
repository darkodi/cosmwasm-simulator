mod simulator;

fn main() -> anyhow::Result<()> {
    let wasm_path = "artifacts/cw20_base.wasm";

    simulator::simulate_instantiate(wasm_path)?;
    simulator::simulate_execute(wasm_path)?;

    Ok(())
}
