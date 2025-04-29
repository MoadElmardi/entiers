mod consommation {
    //pub mod addition2;
    // pub mod addition;
     pub mod multiplication;
    // pub mod division;
}
use consommation::multiplication::main as test_main;

fn main() {
    let _ = test_main();
}
