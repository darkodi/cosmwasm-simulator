use crate::simulator::Simulator;

/// Simulate and assert balance changes for a CW20 transfer
pub fn run_transfer_diff_scenario() {
    let wasm_path = "artifacts/cw20_base.wasm";
    let sender = "creator";
    let recipient = "recipient";

    let instantiate_msg =
        std::fs::read_to_string("data/instantiate.json").expect("instantiate.json missing");
    let exec_msg = std::fs::read_to_string("data/exec.json").expect("exec.json missing");

    let mut sim = Simulator::new(wasm_path);
    sim.instantiate(sender, &instantiate_msg);

    let before_sender = sim.query(&format!(
        r#"{{ "balance": {{ "address": "{}" }} }}"#,
        sender
    ));
    let before_recipient = sim.query(&format!(
        r#"{{ "balance": {{ "address": "{}" }} }}"#,
        recipient
    ));

    let before_sender_bal: u128 = before_sender["balance"].as_str().unwrap().parse().unwrap();
    let before_recipient_bal: u128 = before_recipient["balance"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap();

    sim.execute(sender, &exec_msg);

    let after_sender = sim.query(&format!(
        r#"{{ "balance": {{ "address": "{}" }} }}"#,
        sender
    ));
    let after_recipient = sim.query(&format!(
        r#"{{ "balance": {{ "address": "{}" }} }}"#,
        recipient
    ));

    let after_sender_bal: u128 = after_sender["balance"].as_str().unwrap().parse().unwrap();
    let after_recipient_bal: u128 = after_recipient["balance"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap();

    println!(
        "💰 Creator balance:   {} → {}",
        before_sender_bal, after_sender_bal
    );
    println!(
        "💰 Recipient balance: {} → {}",
        before_recipient_bal, after_recipient_bal
    );

    assert_eq!(
        before_sender_bal - 12345,
        after_sender_bal,
        "Sender balance mismatch"
    );
    assert_eq!(
        before_recipient_bal + 12345,
        after_recipient_bal,
        "Recipient balance mismatch"
    );

    println!("✅ Transfer scenario passed!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_balance_change() {
        run_transfer_diff_scenario();
    }
}
