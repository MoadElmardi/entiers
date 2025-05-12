use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint16};

// Add for pmap
// use std::time::Duration;
// use std::thread::sleep;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigBuilder::default().build();
    let (client_key, server_keys) = generate_keys(config);
    set_server_key(server_keys);

    let ctxt_a = FheUint16::try_encrypt(4000u16, &client_key)?;
    let ctxt_b = FheUint16::try_encrypt(1200u16, &client_key)?;

    let ctxt_cmp = &ctxt_a.gt(&ctxt_b);
    let ctxt_max = ctxt_cmp.if_then_else(&ctxt_a, &ctxt_b);

    let _result: u16 = ctxt_max.decrypt(&client_key);
    //println!("{result}");
    //Add for pmap
    // println!("Addition terminée. Pause pour mesure mémoire...");
    // sleep(Duration::from_secs(120));

    Ok(())
}
