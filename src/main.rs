mod tri_liste;

use std::time::Instant;
use tri_liste::main as test_main;

fn main() {
    
    let start = Instant::now();
    let _ = test_main();
    let duration = start.elapsed();
    println!("\nDuration: {:?}", duration);
}
