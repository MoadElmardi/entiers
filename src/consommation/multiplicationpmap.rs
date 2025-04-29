use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint128};
use std::time::Duration;
use std::thread::sleep;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Début");
    let config = ConfigBuilder::default().build();
    let (client_key, server_keys) = generate_keys(config);
    set_server_key(server_keys);
    println!("Clés générées.");

    let ctxt_a = FheUint128::try_encrypt(120u128, &client_key)?;
    let ctxt_b = FheUint128::try_encrypt(20u128, &client_key)?;
    println!("Chiffrement terminé.");

    let ctxt_result = &ctxt_a * &ctxt_b;
    println!("Résultat calculé");

    let _result: u128 = ctxt_result.decrypt(&client_key);

    println!("Division terminée. Pause pour mesure mémoire...");
    sleep(Duration::from_secs(7200));

    Ok(())
}
