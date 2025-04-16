use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint512};
use std::time::{Duration, Instant};

const N: usize = 100;

fn mean(durations: &[Duration]) -> Duration {
    durations.iter().sum::<Duration>() / (durations.len() as u32)
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigBuilder::default().build();
    // Génération des clés
    let start_gen_key = Instant::now();
    let (client_key, server_keys) = generate_keys(config);
    let gen_key_duration = start_gen_key.elapsed();
    set_server_key(server_keys);

    // Chiffrement des entiers
    let a: u64 = 150;
    let b: u64 = 50;

    //let start_encrypt = Instant::now();
    let ctxt_a = FheUint512::try_encrypt(a, &client_key)?;
    //let encrypt_duration = start_encrypt.elapsed();
    let ctxt_b = FheUint512::try_encrypt(b, &client_key)?;

    //let mut encrypt_times = Vec::with_capacity(N);
    let mut op_times = Vec::with_capacity(N);
    let mut op2_times = Vec::with_capacity(N);
    let mut op3_times = Vec::with_capacity(N);
    // let mut op4_times = Vec::with_capacity(N);
    let mut op5_times = Vec::with_capacity(N);
    let mut op6_times = Vec::with_capacity(N);
    let mut op7_times = Vec::with_capacity(N);
    let mut op8_times = Vec::with_capacity(N);
    let mut op9_times = Vec::with_capacity(N);
    let mut op10_times = Vec::with_capacity(N);
    let mut op11_times = Vec::with_capacity(N);
    //let mut decrypt_times = Vec::with_capacity(N);

    for i in 0..N {

        println!("Itération {i}");

//  *** Opérations arithmétiques *******************************      

        // Addition homomorphe
        let start_op = Instant::now();
        let ctxt_result = &ctxt_a + &ctxt_b;
        let op_duration = start_op.elapsed();

        // Soustraction homomorphe
        let start_op = Instant::now();
        let ctxt_result2 = &ctxt_a - &ctxt_b;
        let op_duration2 = start_op.elapsed();

        // Multiplication homomorphe
        let start_op = Instant::now();
        let ctxt_result3 = &ctxt_a * &ctxt_b;
        let op_duration3 = start_op.elapsed();

        // Division homomorphe
        // let start_op = Instant::now();
        // let ctxt_result4 = &ctxt_a / &ctxt_b;
        // let op_duration4 = start_op.elapsed();

//  *** Opérations bit à bit (logiques) *************************      

        /* let ctxt_a6 = ctxt_a.clone();
        let ctxt_b6 = ctxt_b.clone();
        let ctxt_a7 = ctxt_a.clone();
        let ctxt_b7 = ctxt_b.clone(); */
        
        // ET binaire
        let start_op = Instant::now();
        let ctxt_result5 = &ctxt_a & &ctxt_b;
        let op_duration5 = start_op.elapsed(); 

        // OU binaire
        let start_op = Instant::now();
        let ctxt_result6= &ctxt_a | &ctxt_b;
        let op_duration6 = start_op.elapsed();  

        // XOR binaire
        let start_op = Instant::now();
        let ctxt_result7 = &ctxt_a ^ &ctxt_b;
        let op_duration7 = start_op.elapsed(); 

//  *** Comparaisons (retournent un booléen chiffré) *************

        // Egalité
        let start_op = Instant::now();
        let ctxt_result8 = ctxt_a.eq(&ctxt_b);
        let op_duration8 = start_op.elapsed();

        // Inégalité
        let start_op = Instant::now();
        let ctxt_result9 = ctxt_a.ne(&ctxt_b);
        let op_duration9 = start_op.elapsed(); 

        // Supérieur strictement
        let start_op = Instant::now();
        let ctxt_result10 = ctxt_a.gt(&ctxt_b);
        let op_duration10 = start_op.elapsed();  

        // Supérieur ou égal
        let start_op = Instant::now();
        let ctxt_result11 = ctxt_a.ge(&ctxt_b);
        let op_duration11 = start_op.elapsed(); 

        // Déchiffrement
        //let start_decrypt = Instant::now();
        let result: u64 = ctxt_result.decrypt(&client_key);
        //let decrypt_duration = start_decrypt.elapsed();
        let result2: u64 = ctxt_result2.decrypt(&client_key);
        let result3: u64 = ctxt_result3.decrypt(&client_key);
        // let result4: u64 = ctxt_result4.decrypt(&client_key);
        let result5: u64 = ctxt_result5.decrypt(&client_key);
        let result6: u64 = ctxt_result6.decrypt(&client_key);
        let result7: u64 = ctxt_result7.decrypt(&client_key);
        let result8: bool = ctxt_result8.decrypt(&client_key);
        let result9: bool = ctxt_result9.decrypt(&client_key);
        let result10: bool = ctxt_result10.decrypt(&client_key);
        let result11: bool = ctxt_result11.decrypt(&client_key);


        // pour vérifier la validité
        assert_eq!(result, a + b); 
        assert_eq!(result2, a - b);
        assert_eq!(result3, a * b);
        // assert_eq!(result4, a / b);
        assert_eq!(result5, a & b);
        assert_eq!(result6, a | b);
        assert_eq!(result7, a ^ b);
        assert_eq!(result8, a == b);
        assert_eq!(result9, a != b);
        assert_eq!(result10, a > b);
        assert_eq!(result11, a >= b);


        //encrypt_times.push(encrypt_duration);
        op_times.push(op_duration);
        op2_times.push(op_duration2);
        op3_times.push(op_duration3);
        // op4_times.push(op_duration4);
        op5_times.push(op_duration5);
        op6_times.push(op_duration6);
        op7_times.push(op_duration7);
        op8_times.push(op_duration8);
        op9_times.push(op_duration9);
        op10_times.push(op_duration10);
        op11_times.push(op_duration11);
        //decrypt_times.push(decrypt_duration);
    }

    // Moyenne et écart-type
    //let encrypt_mean = mean(&encrypt_times);
    let op_mean = mean(&op_times);
    let op2_mean = mean(&op2_times);
    let op3_mean = mean(&op3_times);
    // let op4_mean = mean(&op4_times);
    let op5_mean = mean(&op5_times);
    let op6_mean = mean(&op6_times);
    let op7_mean = mean(&op7_times);
    let op8_mean = mean(&op8_times);
    let op9_mean = mean(&op9_times);
    let op10_mean = mean(&op10_times);
    let op11_mean = mean(&op11_times);
    //let decrypt_mean = mean(&decrypt_times);

    println!("--- Résultats sur {N} répétitions ---");
    println!(
        "Génération des clés: {:?}", gen_key_duration
    );
    println!(
        "Addition homomorphe : moyenne = {:?}",
        op_mean
    );
    println!(
        "Soustraction homomorphe : moyenne = {:?}",
        op2_mean 
    );
    println!(
        "Multiplication homomorphe : moyenne = {:?}",
        op3_mean 
    );
    // println!(
    //     "Division homomorphe : moyenne = {:?}",
    //     op4_mean 
    // );
    println!(
        "ET binaire : moyenne = {:?}",
        op5_mean 
    );
    println!(
        "OU binaire : moyenne = {:?}",
        op6_mean 
    );
    println!(
        "XOR binaire : moyenne = {:?}",
        op7_mean 
    );
    println!(
        "Egalité : moyenne = {:?}",
        op8_mean 
    );
    println!(
        "Inégalité : moyenne = {:?}",
        op9_mean 
    );
    println!(
        "Supérieur strictement : moyenne = {:?}",
        op10_mean 
    );
    println!(
        "Supérieur ou égal : moyenne = {:?}",
        op11_mean 
    );

    Ok(())
}
