use std::time::Instant;
use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint16};
use rand::{rng, Rng};


fn cmp_swap_plain<T: Ord + Copy>(a: T, b: T, ascending: bool) -> (T, T) {
    if (a > b) == ascending {
        (b, a)
    } else {
        (a, b)
    }
}

fn bitonic_merge_plain<T: Ord + Copy>(slice: &mut [T], ascending: bool) {
    let len = slice.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;
    for i in 0..mid {
        let (a, b) = cmp_swap_plain(slice[i], slice[i + mid], ascending);
        slice[i] = a;
        slice[i + mid] = b;
    }
    bitonic_merge_plain(&mut slice[..mid], ascending);
    bitonic_merge_plain(&mut slice[mid..], ascending);
}

fn bitonic_sort_plain<T: Ord + Copy>(slice: &mut [T], ascending: bool) {
    let len = slice.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;
    bitonic_sort_plain(&mut slice[..mid], true);
    bitonic_sort_plain(&mut slice[mid..], false);
    bitonic_merge_plain(slice, ascending);
}

fn cmp_swap(a: &FheUint16, b: &FheUint16) -> (FheUint16, FheUint16) {
    // FheBool
    let cond = a.gt(b);
    
    let a_new = cond.if_then_else(b, a);
    let b_new = cond.if_then_else(a, b);
    (a_new, b_new)
}

fn bitonic_sort(slice: &mut [FheUint16], ascending: bool) {
    let len = slice.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;
    bitonic_sort(&mut slice[..mid], true);
    bitonic_sort(&mut slice[mid..], false);
    bitonic_merge(slice, ascending);
}

fn bitonic_merge(slice: &mut [FheUint16], ascending: bool) {
    let len = slice.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;
    for i in 0..mid{
        let (a, b) = if ascending {
            cmp_swap(&slice[i], &slice[i + mid])
        } else {
            let (b, a) = cmp_swap(&slice[i], &slice[i + mid]);
            (a, b)
        };
        slice[i] = a;
        slice[i + mid] = b;
    }

    bitonic_merge(&mut slice[..mid], ascending);
    bitonic_merge(&mut slice[mid..], ascending);
}

fn generate_sample_vector(size: usize) -> Vec<u16> {
    let mut rng = rng();
    (0..size)
        .map(|_| rng.random_range(100..10_000)) // valeurs réalistes (3 à 4 chiffres)
        .collect()
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigBuilder::default().build();
    // Génération des clés
    let (client_key, server_keys) = generate_keys(config);
    set_server_key(server_keys);

    for i in 0..5 {
        println!("Itération {:?}", i+1);
        let plaintexts = generate_sample_vector(256);
        println!("Liste: {:?}", plaintexts);
        let mut list: Vec<FheUint16> = plaintexts
            .iter()
            .map(|&x| FheUint16::try_encrypt(x, &client_key).unwrap())
            .collect();

        // let start_sort = Instant::now();
        // bitonic_sort_plain(&mut plaintexts, true);
        // let duration_sort_plain = start_sort.elapsed();

        let start_sort = Instant::now();
        bitonic_sort(&mut list, true);
        let duration_sort = start_sort.elapsed();

        let _dechiffre: Vec<u16> = list
            .iter()
            .map(|x| x.decrypt(&client_key))
            .collect();

        // println!("Tri: {:?}", dechiffre);
        // println!("Duration de tri: {:?}", duration_sort_plain);
        println!("Duration de tri chiffré: {:?}", duration_sort);
    }

    

    Ok(())
}

