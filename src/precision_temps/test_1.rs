use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint8};
use std::time::{Duration, Instant};

const N: usize = 100;

fn mean(durations: &[Duration]) -> Duration {
    durations.iter().sum::<Duration>() / (durations.len() as u32)
}

fn std_dev(durations: &[Duration], mean: Duration) -> Duration {
    let var = durations
        .iter()
        .map(|d| {
            let diff = (*d).as_secs_f64() - mean.as_secs_f64();
            diff * diff
        })
        .sum::<f64>()
        / durations.len() as f64;
    Duration::from_secs_f64(var.sqrt())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigBuilder::default().build();
    // Génération des clés
    let (client_key, server_keys) = generate_keys(config);
    set_server_key(server_keys);

    // Chiffrement des entiers
    let a: u8 = 10;
    let b: u8 = 20;

    let mut encrypt_times = Vec::with_capacity(N);
    let mut op_times = Vec::with_capacity(N);
    let mut op2_times = Vec::with_capacity(N);
    let mut op3_times = Vec::with_capacity(N);
    let mut op4_times = Vec::with_capacity(N);
    let mut decrypt_times = Vec::with_capacity(N);

    for _ in 0..N {
        let start_encrypt = Instant::now();
        let ctxt_a = FheUint8::try_encrypt(a, &client_key)?;
        let encrypt_duration = start_encrypt.elapsed();
        let ctxt_b = FheUint8::try_encrypt(b, &client_key)?;

        // Opération homomorphe (addition)
        let start_op = Instant::now();
        let ctxt_result = &ctxt_a + &ctxt_b;
        let op_duration = start_op.elapsed();

        // Opération homomorphe (addition d'une constante)
        let start_op = Instant::now();
        let ctxt_result2 = &ctxt_a + 1;
        let op_duration2 = start_op.elapsed();

        // Opération homomorphe (multiplication par une constante)
        let start_op = Instant::now();
        let ctxt_result3 = &ctxt_a * 2;
        let op_duration3 = start_op.elapsed();
        
        // Opération homomorphe (multiplication)
        let start_op = Instant::now();
        let ctxt_result4 = &ctxt_a * &ctxt_b;
        let op_duration4 = start_op.elapsed();


        // Déchiffrement
        let start_decrypt = Instant::now();
        let result: u8 = ctxt_result.decrypt(&client_key);
        let decrypt_duration = start_decrypt.elapsed();
        let result2: u8 = ctxt_result2.decrypt(&client_key);
        let result3: u8 = ctxt_result3.decrypt(&client_key);
        let result4: u8 = ctxt_result4.decrypt(&client_key);

        // pour vérifier la validité
        assert_eq!(result, a + b); 
        assert_eq!(result2, a + 1);
        assert_eq!(result3, a * 2);
        assert_eq!(result4, a * b);

        encrypt_times.push(encrypt_duration);
        op_times.push(op_duration);
        op2_times.push(op_duration2);
        op3_times.push(op_duration3);
        op4_times.push(op_duration4);
        decrypt_times.push(decrypt_duration);
    }

    // Moyenne et écart-type
    let encrypt_mean = mean(&encrypt_times);
    let op_mean = mean(&op_times);
    let op2_mean = mean(&op2_times);
    let op3_mean = mean(&op3_times);
    let op4_mean = mean(&op4_times);
    let decrypt_mean = mean(&decrypt_times);

    let encrypt_std = std_dev(&encrypt_times, encrypt_mean);
    let op_std = std_dev(&op_times, op_mean);
    let op2_std = std_dev(&op2_times, op2_mean);
    let op3_std = std_dev(&op3_times, op3_mean);
    let op4_std = std_dev(&op4_times, op4_mean);
    let decrypt_std = std_dev(&decrypt_times, decrypt_mean);

    println!("--- Résultats sur {N} répétitions ---");
    println!(
        "Chiffrement: moyenne = {:?}, écart-type = {:?}",
        encrypt_mean, encrypt_std
    );
    println!(
        "Addition  : moyenne = {:?}, écart-type = {:?}",
        op_mean, op_std
    );
    println!(
        "Addition d'une constante  : moyenne = {:?}, écart-type = {:?}",
        op2_mean, op2_std
    );
    println!(
        "Multiplication par une constante  : moyenne = {:?}, écart-type = {:?}",
        op3_mean, op3_std
    );
    println!(
        "Multiplication  : moyenne = {:?}, écart-type = {:?}",
        op4_mean, op4_std
    );
    println!(
        "Déchiffrement: moyenne = {:?}, écart-type = {:?}",
        decrypt_mean, decrypt_std
    );

    Ok(())
}
