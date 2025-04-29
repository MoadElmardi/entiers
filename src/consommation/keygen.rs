use tfhe::{ConfigBuilder, generate_keys};

fn main() {
    let config = ConfigBuilder::default().build();
    let (_client_key, _server_key) = generate_keys(config);
    // Keep keys alive to simulate real usage
}