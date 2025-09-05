use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint64};
//use std::time::{Duration, Instant};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let now = Instant::now();
    let config = ConfigBuilder::default().build();
    let (client_key, server_keys) = generate_keys(config);
    set_server_key(server_keys);

    let a = 16u64;
    let b = 61u64;

    let ctxt_a = FheUint64::encrypt(a, &client_key);
    let ctxt_b = FheUint64::encrypt(b, &client_key);

    let ctxt_result = &ctxt_a - &ctxt_b;

    let _scaled_res: u64 = ctxt_result.decrypt(&client_key);
    //let duration = now.elapsed();

    Ok(())
}
