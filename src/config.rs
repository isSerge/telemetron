#[derive(Debug)]
pub struct Config {
    pub http_port: u16,
    pub http_host: String,
}

// TODO: read from environment variables or a config file
impl Config {
    pub fn new() -> Self {
        Config { http_port: 3000, http_host: "0.0.0.0".to_string() }
    }
}
