use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint64};
use std::time::Instant;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    for i in 0..5 {
        println!("Itération {:?}", i+1);
        let now = Instant::now();
        let config = ConfigBuilder::default().build();
        let (client_key, server_keys) = generate_keys(config);
        set_server_key(server_keys);

        let a: u64 = 3000;
        let ctxt_a = FheUint64::encrypt(a, &client_key);

        let b: u64 = 594;
        let ctxt_b = FheUint64::encrypt(b, &client_key);

        let ctxt_result = ctxt_a - ctxt_b;

        let _result: u64 = ctxt_result.decrypt(&client_key);

        let duration = now.elapsed();
        println!("Duration: {:?}", duration);
    }

    Ok(())
}
