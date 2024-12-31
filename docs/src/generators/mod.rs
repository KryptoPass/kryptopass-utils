mod config;
mod utils;
mod validator;

use std::error::Error;
use std::fs;

pub use config::Config;
use validator::validate_config;

pub struct PasswordGenerator {
    config: Config,
}

impl PasswordGenerator {
    pub fn from_config(config: Config) -> Self {
        PasswordGenerator { config }
    }

    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;

        validate_config(&config)?;

        Ok(PasswordGenerator { config })
    }

    pub fn generate(&self) -> Option<String> {
        println!("Generator Name: {}", self.config.properties.name);

        Some("Hello, world!".to_string())
    }
}
