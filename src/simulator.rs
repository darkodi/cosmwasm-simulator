use cosmwasm_std::Empty;
use cosmwasm_vm::testing::{mock_backend, mock_env, mock_info, MockApi, MockQuerier, MockStorage};
use cosmwasm_vm::{call_execute, call_instantiate, call_query, Instance, InstanceOptions, Size};
use serde_json::Value;
use std::fs;

pub struct Simulator {
    pub instance: Instance<MockApi, MockStorage, MockQuerier>,
}

impl Simulator {
    pub fn new(wasm_path: &str) -> Self {
        let wasm = fs::read(wasm_path).expect("WASM file not found");
        let backend = mock_backend(&[]);
        let options = InstanceOptions {
            gas_limit: 100_000_000_000,
            print_debug: true,
        };

        let instance = Instance::from_code(&wasm, backend, options, Some(Size::mebi(16)))
            .expect("Instance creation failed");

        Self { instance }
    }

    pub fn instantiate(&mut self, sender: &str, json_msg: &str) {
        let env = mock_env();
        let info = mock_info(sender, &[]);
        call_instantiate::<_, _, _, Empty>(&mut self.instance, &env, &info, json_msg.as_bytes())
            .expect("instantiate failed");
    }

    pub fn execute(&mut self, sender: &str, json_msg: &str) {
        let env = mock_env();
        let info = mock_info(sender, &[]);
        let res =
            call_execute::<_, _, _, Empty>(&mut self.instance, &env, &info, json_msg.as_bytes())
                .expect("execute failed");

        println!("✅ Execute result: {:?}", res);
    }

    pub fn query(&mut self, json_msg: &str) -> Value {
        let env = mock_env();
        let result = call_query::<_, _, _>(&mut self.instance, &env, json_msg.as_bytes())
            .expect("query failed");
        let binary = result.unwrap();

        let parsed: Value =
            serde_json::from_slice(binary.as_slice()).expect("failed to parse JSON");
        println!("🔍 Query result: {}", parsed);
        parsed
    }
}
