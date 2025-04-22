//use num_bigint::BigUint;
use tfhe::integer::bigint::StaticUnsignedBigInt;
use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint2048, FheUint8};
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

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigBuilder::default().build();
    // Génération des clés
    let (client_key, server_keys) = generate_keys(config);
    set_server_key(server_keys);

    // Chiffrement des entiers
    let a = 150u64;
    //let a = BigUint::from(150u128);

    let mut encrypt_times = Vec::with_capacity(N);
    let mut op_times = Vec::with_capacity(N);
    let mut op2_times = Vec::with_capacity(N);
    let mut op3_times = Vec::with_capacity(N);
    let mut op4_times = Vec::with_capacity(N);
    let mut op5_times = Vec::with_capacity(N);
    let mut op6_times = Vec::with_capacity(N);
    let mut op7_times = Vec::with_capacity(N);
    let mut decrypt_times = Vec::with_capacity(N);

    for _ in 0..N {
        let start_encrypt = Instant::now();
        let ctxt_a = FheUint2048::try_encrypt(a, &client_key)?;
        let encrypt_duration = start_encrypt.elapsed();

        // Opération homomorphe (shift)
        let start_op = Instant::now();
        let ctxt_result: FheUint2048 = &ctxt_a >> 2_u64;
        let op_duration = start_op.elapsed();

        // Opération homomorphe (addition d'une constante)
        let start_op = Instant::now();
        let ctxt_result2 = &ctxt_a + StaticUnsignedBigInt::<32>::from(25u64);
        let op_duration2 = start_op.elapsed();

        // Opération homomorphe (multiplication par une constante)
        let start_op = Instant::now();
        let ctxt_result3 = &ctxt_a * StaticUnsignedBigInt::<32>::from(2u64);
        let op_duration3 = start_op.elapsed();

        let ctxt_a6 = ctxt_a.clone();
        let ctxt_a7 = ctxt_a.clone();

        // Opération homomorphe (ET binaire)
        let start_op = Instant::now();
        let ctxt_result5 = ctxt_a & StaticUnsignedBigInt::<32>::from(0u64);
        let op_duration5 = start_op.elapsed();

        // Opération homomorphe (NOT)
        let start_op = Instant::now();
        let ctxt_result6 = !ctxt_a6;
        let op_duration6 = start_op.elapsed(); 

        // Opération homomorphe (casting)

        let start_op = Instant::now();
        let ctxt_result4: FheUint8 = ctxt_a7.cast_into();
        let op_duration4 = start_op.elapsed();  

        // Opération homomorphe (OU binaire)
        let start_op = Instant::now();
        let ctxt_result7 = ctxt_result4 | 0_u8 ;
        let op_duration7 = start_op.elapsed();      

        // Déchiffrement
        let start_decrypt = Instant::now();
        let result: u64 = ctxt_result.decrypt(&client_key);
        let decrypt_duration = start_decrypt.elapsed();
        let result2: u64 = ctxt_result2.decrypt(&client_key);
        let result3: u64 = ctxt_result3.decrypt(&client_key);
        let result5: u64 = ctxt_result5.decrypt(&client_key);
        let result6: u64 = ctxt_result6.decrypt(&client_key);
        let result7: u64 = ctxt_result7.decrypt(&client_key);


        // pour vérifier la validité
        assert_eq!(result, a >> 2); 
        assert_eq!(result2, a + 25);
        assert_eq!(result3, a * 2);
        assert_eq!(result5, a & 0);
        assert_eq!(result6, !a);
        assert_eq!(result7, a | 0);


        encrypt_times.push(encrypt_duration);
        op_times.push(op_duration);
        op2_times.push(op_duration2);
        op3_times.push(op_duration3);
        op4_times.push(op_duration4);
        op5_times.push(op_duration5);
        op6_times.push(op_duration6);
        op7_times.push(op_duration7);
        decrypt_times.push(decrypt_duration);
    }

    // Moyenne et écart-type
    let encrypt_mean = mean(&encrypt_times);
    let op_mean = mean(&op_times);
    let op2_mean = mean(&op2_times);
    let op3_mean = mean(&op3_times);
    let op4_mean = mean(&op4_times);
    let op5_mean = mean(&op5_times);
    let op6_mean = mean(&op6_times);
    let op7_mean = mean(&op7_times);
    let decrypt_mean = mean(&decrypt_times);

    let encrypt_std = std_dev(&encrypt_times, encrypt_mean);
    let op_std = std_dev(&op_times, op_mean);
    let op2_std = std_dev(&op2_times, op2_mean);
    let op3_std = std_dev(&op3_times, op3_mean);
    let op4_std = std_dev(&op4_times, op4_mean);
    let op5_std = std_dev(&op5_times, op5_mean);
    let op6_std = std_dev(&op6_times, op6_mean);
    let op7_std = std_dev(&op7_times, op7_mean);
    let decrypt_std = std_dev(&decrypt_times, decrypt_mean);

    println!("--- Résultats sur {N} répétitions ---");
    println!(
        "Chiffrement: moyenne = {:?}, écart-type = {:?}",
        encrypt_mean, encrypt_std
    );
    println!(
        "Shift  : moyenne = {:?}, écart-type = {:?}",
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
        "Casting  : moyenne = {:?}, écart-type = {:?}",
        op4_mean, op4_std
    );
    println!(
        "ET binaire : moyenne = {:?}, écart-type = {:?}",
        op5_mean, op5_std
    );
    println!(
        "OU binaire  : moyenne = {:?}, écart-type = {:?}",
        op7_mean, op7_std
    );
    println!(
        "NOT  : moyenne = {:?}, écart-type = {:?}",
        op6_mean, op6_std
    );
    println!(
        "Déchiffrement: moyenne = {:?}, écart-type = {:?}",
        decrypt_mean, decrypt_std
    );

    Ok(())
}
