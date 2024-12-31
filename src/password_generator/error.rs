use thiserror::Error;

#[derive(Error, Debug)]
pub enum PasswordGenError {
    #[error("An error occurred while reading the TOML file.")]
    IOError(#[from] std::io::Error),

    #[error("An error occurred while deserializing the TOML file.")]
    TomlDeserializeError(#[from] toml::de::Error),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("File version ({0}) is incompatible. Use the supported version ({1}) to proceed.")]
    IncompatibleVersion(String, String),
}

pub type Result<T> = std::result::Result<T, PasswordGenError>;
