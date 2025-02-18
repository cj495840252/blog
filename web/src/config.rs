use std::fs::File;

use serde::Deserialize;

use crate::error::WebError;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub port: usize,
    pub base_dir: String,
    pub db_url: String,
}

pub fn load_config() -> Result<AppConfig, WebError> {
    // let s = include_str!("app.yaml");
    let file = File::open("app.yaml")?;
    let reader = std::io::BufReader::new(file);
    let config: AppConfig = serde_yaml::from_reader(reader)?;

    Ok(config)
}

#[cfg(test)]
mod test {
    use std::net::SocketAddr;

    use crate::config::load_config;

    #[test]
    fn test_load_config_should_work() {
        let config = load_config().unwrap();
        assert_eq!(config.server.port, 8080);
        let socket_addr: SocketAddr = format!("[::0]:{}", config.server.port).parse().unwrap();
        println!("socket_addr: {:?}", socket_addr);
    }
}
