use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint8};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigBuilder::default().build();
    let (client_key, server_keys) = generate_keys(config);
    set_server_key(server_keys);

    let ctxt_a = FheUint8::try_encrypt(45u8, &client_key)?;
    let ctxt_b = FheUint8::try_encrypt(12u8, &client_key)?;

    let ctxt_result = &ctxt_a + &ctxt_b;

    let result: u8 = ctxt_result.decrypt(&client_key);

    println!("Résultat = {}", result);
}