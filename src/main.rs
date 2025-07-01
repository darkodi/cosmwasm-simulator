mod simulator;

fn main() {
    let creator = "creator";
    let recipient = "cosmos1deadbeefdeadbeefdeadbeefdeadbeefdead00e";

    let mut instance = simulator::instantiate_contract(creator, recipient);

    let before_creator = simulator::simulate_query_balance(&mut instance, creator);
    let before_recipient = simulator::simulate_query_balance(&mut instance, recipient);

    let transfer_amount = "12345";
    simulator::simulate_transfer(&mut instance, creator, recipient, transfer_amount);

    let after_creator = simulator::simulate_query_balance(&mut instance, creator);
    let after_recipient = simulator::simulate_query_balance(&mut instance, recipient);

    println!("💰 creator:     before = {}, after = {}", before_creator, after_creator);
    println!("💰 recipient:   before = {}, after = {}", before_recipient, after_recipient);

    let transfer_value: u128 = transfer_amount.parse().unwrap();
    assert_eq!(before_creator - transfer_value, after_creator, "Creator balance mismatch");
    assert_eq!(before_recipient + transfer_value, after_recipient, "Recipient balance mismatch");

    println!("✅ Balance transfer simulation passed!");
}
