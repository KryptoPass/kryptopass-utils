pub mod config;
pub mod error;

use config::Config;

pub struct GenPassword {
    config: Config,
}

impl GenPassword {
    pub fn new(config: Config) -> Self {
        GenPassword { config }
    }

    pub fn generate(&self) -> String {
        let charset = self.config.get_charset().unwrap();

        println!("{:?}", charset);

        String::new()
    }
}
