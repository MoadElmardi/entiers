use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint8};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Début");
    let config = ConfigBuilder::default().build();
    let (client_key, server_keys) = generate_keys(config);
    set_server_key(server_keys);
    println!("Clés générées.");

    let mut ctxt_a = FheUint8::try_encrypt(1u8, &client_key)?;
    println!("Chiffrement terminé.");

    for i in 1..10 {
        println!("Iteration: {}", i);
        let ctxt_i = FheUint8::try_encrypt(i as u8, &client_key)?;
        ctxt_a = &ctxt_a * &ctxt_i;
    }
    println!("Résultat calculé");

    let result: u8 = ctxt_a.decrypt(&client_key);
    println!("Result: {}", result);

    Ok(())
}