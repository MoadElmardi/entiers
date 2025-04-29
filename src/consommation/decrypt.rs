use tfhe::prelude::*;
use tfhe::{ConfigBuilder, generate_keys, FheUint8, set_server_key};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigBuilder::default().build();
    let (client_key, server_key) = generate_keys(config);
    set_server_key(server_key);
    let ctxt = FheUint8::try_encrypt(99u8, &client_key)?;
    let _result: u8 = ctxt.decrypt(&client_key);
    Ok(())
}