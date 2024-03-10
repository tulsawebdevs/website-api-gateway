use std::net::SocketAddr;

use config::load_config;

mod config;

#[tokio::main]
async fn main() {
    let config = load_config("gateway_config.toml");
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("config:{:?}, addr:{:?}", config, addr);
}
