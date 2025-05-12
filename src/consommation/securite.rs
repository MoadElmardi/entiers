use tfhe::prelude::*;
use tfhe::shortint::parameters::v1_1::*;
use tfhe::{ConfigBuilder, generate_keys, set_server_key, FheUint16};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config 
        = ConfigBuilder::with_custom_parameters(V1_1_PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M128)
            .build();
    let (client_key, server_keys) = generate_keys(config);
    set_server_key(server_keys);

    let ctxt_a = FheUint16::try_encrypt(400u16, &client_key)?;
    let ctxt_b = FheUint16::try_encrypt(1200u16, &client_key)?;

    let ctxt_result = &ctxt_a + &ctxt_b;

    let _result: u16 = ctxt_result.decrypt(&client_key);
    Ok(())
}