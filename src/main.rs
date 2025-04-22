mod precision_temps {
    pub mod tri_liste;
}

use std::time::Instant;
use precision_temps::tri_liste::main as test_main;

fn main() {
    
    let start = Instant::now();
    let _ = test_main();
    let duration = start.elapsed();
    println!("\nDuration: {:?}", duration);
}
