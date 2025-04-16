mod deux_entiers;

use std::time::Instant;
use deux_entiers::main as test_main;

fn main() {
    
    let start = Instant::now();
    let _ = test_main();
    let duration = start.elapsed();
    println!("\nDuration: {:?}", duration);
}
