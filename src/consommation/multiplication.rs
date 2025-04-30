use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint64};

// Add for pmap
// use std::time::Duration;
// use std::thread::sleep;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigBuilder::default().build();
    let (client_key, server_keys) = generate_keys(config);
    set_server_key(server_keys);

    let ctxt_a = FheUint64::try_encrypt(42u64, &client_key)?;
    let ctxt_b = FheUint64::try_encrypt(120u64, &client_key)?;

    let ctxt_result = &ctxt_a * &ctxt_b;

    let _result: u64 = ctxt_result.decrypt(&client_key);
    
    //Add for pmap
    // println!("Opération terminée. Pause pour mesure mémoire...");
    // sleep(Duration::from_secs(120));

    Ok(())
}
