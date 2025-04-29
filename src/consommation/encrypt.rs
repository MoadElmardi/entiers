use tfhe::prelude::*;
use tfhe::{ConfigBuilder, generate_keys, FheUint8};

fn main() {
    let config = ConfigBuilder::default().build();
    let (client_key, _) = generate_keys(config);
    let _ctxt = FheUint8::try_encrypt(42u8, &client_key);
}