use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint8};
use std::time::{Duration, Instant};
use rayon::prelude::*;
use std::sync::Arc;
use std::cell::Cell;

const N: usize = 100;

fn mean(durations: &[Duration]) -> Duration {
    durations.iter().sum::<Duration>() / (durations.len() as u32)
}

thread_local! {
    static INITIALIZED: Cell<bool> = Cell::new(false);
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigBuilder::default().build();

    // Génération des clés (hors de la boucle)
    let start_gen_key = Instant::now();
    let (client_key, server_keys) = generate_keys(config);
    let gen_key_duration = start_gen_key.elapsed();
    //set_server_key(server_keys);
    let client_key= Arc::new(client_key);
    let server_keys = Arc::new(server_keys);

    // Valeurs à chiffrer
    let a: u8 = 19;
    let b: u8 = 10;

    // let pool: ThreadPool = ThreadPoolBuilder::new()
    //     .num_threads(8)
    //     .spawn_handler({
    //         let server_key = server_keys.clone();
    //         move |thread| {
    //             set_server_key(server_key.as_ref().clone());
    //             std::thread::Builder::new().spawn(move || thread.run())?;
    //             Ok(())
    //         }
    //     })
    //     .build()?;

    // Chiffrement une seule fois (et cloné dans chaque thread)
    // let ctxt_a = FheUint8::try_encrypt(a, &*client_key)?;
    // let ctxt_b = FheUint8::try_encrypt(b, &*client_key)?;

    // Clone la clé une seule fois (si nécessaire dans chaque thread)
    //let client_key = &client_key;

    // Parallélisation avec rayon
    let results = (0..N).into_par_iter().map(|_| {
            INITIALIZED.with(|flag| {
                if !flag.get() {
                    set_server_key(server_keys.as_ref().clone());
                    flag.set(true);
                }
            });
            // set_server_key(server_keys.as_ref().clone());
    
            let ctxt_a = FheUint8::try_encrypt(a, &*client_key).unwrap();
            let ctxt_b = FheUint8::try_encrypt(b, &*client_key).unwrap();    
            
            let mut times = Vec::new();
    
            macro_rules! time {
                ($code:block) => {{
                    let start = Instant::now();
                    let result = $code;
                    let dur = start.elapsed();
                    times.push(dur);
                    result
                }};
            }
    
            let r1 = time!({ &ctxt_a + &ctxt_b });
            let r2 = time!({ &ctxt_a - &ctxt_b });
            let r3 = time!({ &ctxt_a * &ctxt_b });
            let r4 = time!({ &ctxt_a / &ctxt_b });
    
            let r5 = time!({ &ctxt_a & &ctxt_b });
            let r6 = time!({ &ctxt_a | &ctxt_b });
            let r7 = time!({ &ctxt_a ^ &ctxt_b });
    
            let r8 = time!({ ctxt_a.eq(&ctxt_b) });
            let r9 = time!({ ctxt_a.ne(&ctxt_b) });
            let r10 = time!({ ctxt_a.gt(&ctxt_b) });
            let r11 = time!({ ctxt_a.ge(&ctxt_b) });
    
            // Déchiffrement (non chronométré ici)
            let res: Vec<u8> = vec![
                r1.decrypt(&client_key),
                r2.decrypt(&client_key),
                r3.decrypt(&client_key),
                r4.decrypt(&client_key),
                r5.decrypt(&client_key),
                r6.decrypt(&client_key),
                r7.decrypt(&client_key),
            ];
            let bools = vec![
                r8.decrypt(&client_key),
                r9.decrypt(&client_key),
                r10.decrypt(&client_key),
                r11.decrypt(&client_key),
            ];
    
            // Vérification
            assert_eq!(res[0], a + b);
            assert_eq!(res[1], a - b);
            assert_eq!(res[2], a * b);
            assert_eq!(res[3], a / b);
            assert_eq!(res[4], a & b);
            assert_eq!(res[5], a | b);
            assert_eq!(res[6], a ^ b);
            assert_eq!(bools[0], a == b);
            assert_eq!(bools[1], a != b);
            assert_eq!(bools[2], a > b);
            assert_eq!(bools[3], a >= b);
    
            times
        }).collect::<Vec<_>>();

    // Regrouper les durées
    let mut op_durations = vec![Duration::ZERO; 11];
    for timings in &results {
        for (i, dur) in timings.iter().enumerate() {
            op_durations[i] += *dur;
        }
    }

    println!("--- Résultats sur {N} répétitions ---");
    println!("Génération des clés: {:?}", gen_key_duration);
    println!("Addition homomorphe           : moyenne = {:?}", op_durations[0] / (N as u32));
    println!("Soustraction homomorphe       : moyenne = {:?}", op_durations[1] / (N as u32));
    println!("Multiplication homomorphe     : moyenne = {:?}", op_durations[2] / (N as u32));
    println!("Division homomorphe           : moyenne = {:?}", op_durations[3] / (N as u32));
    println!("ET binaire                    : moyenne = {:?}", op_durations[4] / (N as u32));
    println!("OU binaire                    : moyenne = {:?}", op_durations[5] / (N as u32));
    println!("XOR binaire                   : moyenne = {:?}", op_durations[6] / (N as u32));
    println!("Égalité                       : moyenne = {:?}", op_durations[7] / (N as u32));
    println!("Inégalité                     : moyenne = {:?}", op_durations[8] / (N as u32));
    println!("Supérieur strictement         : moyenne = {:?}", op_durations[9] / (N as u32));
    println!("Supérieur ou égal             : moyenne = {:?}", op_durations[10] / (N as u32));

    Ok(())
}
