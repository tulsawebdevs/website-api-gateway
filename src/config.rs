use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{fs::File, io::Read};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceConfig {
    pub host: String,
    pub port: u32,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_config() -> Result<(), std::io::Error> {
        let config = load_config("./tests/test_config.toml");
        let test_service = config
            .services
            .get("test-service")
            .expect("test service not found");

        assert_eq!(config.auth_url, "https://example.com");
        assert_eq!(test_service.path, "/testservice");
        assert_eq!(
            test_service.target_host,
            "https://my-service.default.svc.cluster.local"
        );
        assert_eq!(test_service.target_port, 8080);

        Ok(())
    }
}
