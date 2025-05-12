use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint16};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigBuilder::default().build();
    let (client_key, server_keys) = generate_keys(config);
    set_server_key(server_keys);

    let mut ctxt_a = FheUint16::try_encrypt(1u16, &client_key)?;

    for i in 2..52 {
        //println!("Iteration: {}", i-1);
        let ctxt_i = FheUint16::try_encrypt(i as u16, &client_key)?;
        ctxt_a = &ctxt_a * &ctxt_i;
    }

    let _result: u16 = ctxt_a.decrypt(&client_key);
    //println!("{result}");

    Ok(())
}