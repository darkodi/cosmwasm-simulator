mod simulator;

fn main() -> anyhow::Result<()> {
    simulator::simulate_instantiate("artifacts/cw20_base.wasm")?;
    Ok(())
}
