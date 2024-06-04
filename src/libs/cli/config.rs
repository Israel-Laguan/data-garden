use serde_yml;
use std::fs;

pub fn read_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string(file_path)?;
    let config: Config = serde_yml::from_str(&config_content)?;
    Ok(config)
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub environment: String,
    pub is_verbose: bool,
    pub server_port: u16,
    pub preferred_output_template: String,
}
