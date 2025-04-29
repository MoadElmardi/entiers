use tfhe::prelude::*;
use tfhe::{ConfigBuilder, generate_keys, FheUint8, set_server_key};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigBuilder::default().build();
    let (client_key, server_key) = generate_keys(config);
    set_server_key(server_key);
    let ctxt_a = FheUint8::try_encrypt(10u8, &client_key)?;
    let ctxt_b = FheUint8::try_encrypt(32u8, &client_key)?;
    let _sum = &ctxt_a + &ctxt_b;
    Ok(())
}