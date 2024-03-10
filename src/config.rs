use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{fs::File, io::Read};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceConfig {
    pub path: String,
    pub target_host: String,
    pub target_port: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GatewayConfig {
    pub auth_url: String,
    pub services: HashMap<String, ServiceConfig>,
}

pub fn load_config(path: &str) -> GatewayConfig {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    toml::from_str(&contents).unwrap()
}
