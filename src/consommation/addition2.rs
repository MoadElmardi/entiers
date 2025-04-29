use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint8};
use std::mem::size_of_val;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigBuilder::default().build();
    let (client_key, server_keys) = generate_keys(config);
    set_server_key(server_keys);

    let a = 45u8;

    let before = get_memory_usage();
    
    let ctxt_a = FheUint8::try_encrypt(a, &client_key)?;
    
    let after_encrypt = get_memory_usage();
    
    let ctxt_b = FheUint8::try_encrypt(12u8, &client_key)?;
    
    let after_encrypt_2 = get_memory_usage();
    
    let ctxt_result = &ctxt_a + &ctxt_b;
    
    let after_add = get_memory_usage();
    
    let _result: u8 = ctxt_result.decrypt(&client_key);
    
    let after_decrypt = get_memory_usage();
    
    println!("LWE dimension: {}", client_key.computation_parameters().lwe_dimension().0);
    println!("Size: {}", size_of_val(&a));
    println!("Size: {} bytes", size_of_val(&ctxt_a));
    println!("Size: {} bytes", size_of_val(&ctxt_b));
    println!("Size: {} bytes", size_of_val(&ctxt_result));
    println!("Size: {} bytes", size_of_val(&_result));
    println!("Memory used by encryption a: {} KB", after_encrypt - before);
    println!("Memory used by encryption b: {} KB", after_encrypt_2 - after_encrypt);
    println!("Memory used by addition: {} KB", after_add - after_encrypt_2);
    println!("Memory used by decryption: {} KB", after_decrypt - after_add);

    Ok(())
}

fn get_memory_usage() -> u64 {
    use procfs::process::Process;
    let me = Process::myself().unwrap();
    me.stat().unwrap().rss as u64 * 4 // page size in KB (usually 4 KB)
}
