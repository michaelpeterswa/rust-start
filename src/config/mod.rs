use std::env;

pub struct Config {
    pub log_level: String,
    pub api_host: String,
    pub api_port: u16,
    pub metrics_host: String,
    pub metrics_port: u16,
}

impl Config {
    pub fn new(
        log_level: String,
        api_host: String,
        api_port: u16,
        metrics_host: String,
        metrics_port: u16,
    ) -> Self {
        Self {
            log_level,
            api_host,
            api_port,
            metrics_host,
            metrics_port,
        }
    }
}

pub fn load() -> Config {
    let log_level = get("LOG_LEVEL");
    let api_host = get("API_HOST");
    let api_port = u16(get("API_PORT"));
    let metrics_host = get("METRICS_HOST");
    let metrics_port = u16(get("METRICS_PORT"));

    Config::new(log_level, api_host, api_port, metrics_host, metrics_port)
}

fn get(key: &str) -> String {
    env::var(key)
        .unwrap_or_else(|_| panic!("{} is not set", key))
        .to_string()
}

fn u16(key: String) -> u16 {
    key.parse::<u16>()
        .unwrap_or_else(|_| panic!("{} is not a valid u16", key))
}
