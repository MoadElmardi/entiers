use tfhe::shortint::{parameters::v1_0::{V1_0_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128}, prelude::*};

fn main() {
    // We generate a set of client/server keys
    let (client_key, server_key) = gen_keys(V1_0_PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_GAUSSIAN_2M128);

    let msg1 = 1;
    let msg2 = 0;

    let modulus = client_key.parameters.message_modulus().0;

    // We use the client key to encrypt two messages:
    let ct_1 = client_key.encrypt(msg1);
    let ct_2 = client_key.encrypt(msg2);

    // We use the server public key to execute an integer circuit:
    let ct_3 = server_key.add(&ct_1, &ct_2);

    // We use the client key to decrypt the output of the circuit:
    let output = client_key.decrypt(&ct_3);
    assert_eq!(output, (msg1 + msg2) % modulus);
}