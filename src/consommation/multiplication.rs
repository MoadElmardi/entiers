use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint128};

// Add for pmap
// use std::time::Duration;
// use std::thread::sleep;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigBuilder::default().build();
    let (client_key, server_keys) = generate_keys(config);
    set_server_key(server_keys);

    let ctxt_a = FheUint128::try_encrypt(42u128, &client_key)?;
    let ctxt_b = FheUint128::try_encrypt(120u128, &client_key)?;

    let ctxt_result = &ctxt_a * &ctxt_b;

    let _result: u128 = ctxt_result.decrypt(&client_key);
    
    //Add for pmap
    // println!("Opération terminée. Pause pour mesure mémoire...");
    // sleep(Duration::from_secs(120));

    Ok(())
}
