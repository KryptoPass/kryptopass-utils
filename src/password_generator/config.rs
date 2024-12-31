use std::{collections::HashMap, fs, path::Path};

use serde::Deserialize;
use uuid::Uuid;

use super::error::{PasswordGenError, Result};
use crate::utils::parse_unicode;

/// Define un trait para validaciones dentro de la configuración.
/// Cada struct que deba validar sus propios campos implementará este trait.
pub trait Validator {
    /// Valida los datos de la estructura. Retorna `Ok(())` si son válidos.
    fn validate(&mut self) -> Result<()>;
}

/// Representa la configuración general, cargada desde un archivo TOML.
#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub version: semver::Version,
    pub profile: Profile,
    pub rules: Rules,
    pub charset: Charset,
}

impl Config {
    /// Carga la configuración desde un archivo en `path` y la valida.
    /// Retorna un `Result<Self>` o un `PasswordGenError`.
    pub fn from_file<T: AsRef<Path>>(path: T) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        let mut config = toml::from_str::<Config>(&contents)?;
        config.validate()?;
        Ok(config)
    }

    pub fn get_charset(&self) -> Result<Vec<char>> {
        if let Some(charsets_rules) = &self.rules.charsets_rules {
            for (name, charset_rule) in charsets_rules {
                let charset = &self.charset.charsets.get(name).unwrap();

                let charset = match charset {
                    CharsetConstraint::Multiple(v) => {
                        let mut valid_chars = Vec::new();

                        for x in v {
                            valid_chars.extend(parse_unicode(x).unwrap());
                        }

                        valid_chars
                    }
                    CharsetConstraint::One(v) => Vec::from(parse_unicode(v).unwrap()),
                };

                println!("{:?}", charset);
            }
        }

        Ok(Vec::new())
    }
}

/// Para la validación de `Config`, se comprueba la versión y se delega la validación
/// a los subcomponentes `profile`, `rules` y `charset`.
impl Validator for Config {
    fn validate(&mut self) -> Result<()> {
        const COMPATIBLE_VERSIONS: &str = ">=0.1.0, <1.0.0";

        // Se parsea la expresión de versiones esperada.
        // `expect` se usa en lugar de `unwrap` para explicar mejor la razón de fallo.
        let req =
            semver::VersionReq::parse(COMPATIBLE_VERSIONS).expect("Expresión semver inválida en COMPATIBLE_VERSIONS");

        if !req.matches(&self.version) {
            return Err(PasswordGenError::IncompatibleVersion(
                self.version.to_string(),
                COMPATIBLE_VERSIONS.into(),
            ));
        }

        self.profile.validate()?;
        self.charset.validate()?;
        self.rules.validate()?;

        Ok(())
    }
}

/// Contiene información de perfil, como un identificador único (`id`) y un `name`.
#[derive(Deserialize, Clone, Debug)]
pub struct Profile {
    id: Uuid,
    name: String,
}

/// Valida la estructura `Profile` revisando que el ID no sea nulo y el nombre cumpla criterios.
impl Validator for Profile {
    fn validate(&mut self) -> Result<()> {
        if self.id.is_nil() {
            return Err(PasswordGenError::InvalidConfig(
                "Profile ID cannot be the nil UUID".into(),
            ));
        }

        if self.name.trim().is_empty() {
            return Err(PasswordGenError::InvalidConfig("Profile name cannot be empty".into()));
        }

        const MIN_NAME_LENGTH: usize = 1;
        const MAX_NAME_LENGTH: usize = 255;

        let name_length = self.name.len();
        if !(MIN_NAME_LENGTH..=MAX_NAME_LENGTH).contains(&name_length) {
            return Err(PasswordGenError::InvalidConfig(format!(
                "Profile name must be between {} and {} characters",
                MIN_NAME_LENGTH, MAX_NAME_LENGTH
            )));
        }

        Ok(())
    }
}

/// Define reglas para la generación de contraseñas,
/// incluyendo restricciones de longitud, caracteres permitidos y otras propiedades.
#[derive(Deserialize, Clone, Debug)]
pub struct Rules {
    length: Constraint,
    include: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
    #[serde(rename = "max-consecutive")]
    max_consecutive: Option<u8>,
    #[serde(rename = "min-entropy-bits")]
    min_entropy_bits: Option<u8>,
    pattern: Option<String>,
    #[serde(flatten)]
    charsets_rules: Option<HashMap<String, RulesConstraint>>,
}

impl Rules {
    /// Función auxiliar para validar las listas de caracteres incluidos/excluidos.
    /// Usa la función `parse_unicode` para transformar las cadenas en chars.
    fn validate_char_list(&self, list: &Option<Vec<String>>, list_name: &str) -> Result<Vec<char>> {
        if let Some(items) = list {
            if !items.is_empty() {
                let mut valid_chars = Vec::new();

                for item in items {
                    let chars = parse_unicode(item).map_err(|e| {
                        PasswordGenError::InvalidConfig(format!("Error procesando '{}' en {}: {}", item, list_name, e))
                    })?;
                    valid_chars.extend(chars);
                }

                if valid_chars.is_empty() {
                    return Err(PasswordGenError::InvalidConfig(format!(
                        "No se generaron caracteres Unicode válidos en {}",
                        list_name
                    )));
                }

                return Ok(valid_chars);
            }
        }
        Ok(Vec::new())
    }
}

/// Valida `Rules`, principalmente ajustando la restricción de longitud y revisando las listas
/// de caracteres incluidos/excluidos.
impl Validator for Rules {
    fn validate(&mut self) -> Result<()> {
        // 1. Validar la restricción de longitud.
        match self.length {
            Constraint::Range { min, max } => {
                if min > max {
                    return Err(PasswordGenError::InvalidConfig(
                        "Minimum length cannot be greater than maximum length".into(),
                    ));
                } else if min == 0 || max == 0 {
                    return Err(PasswordGenError::InvalidConfig(
                        "Length constraints must be greater than zero".into(),
                    ));
                }

                // Si min == max, lo convertimos a un valor exacto para simplificar.
                if min == max {
                    self.length = Constraint::Exact(min)
                }
            }
            Constraint::Exact(value) => {
                if value == 0 {
                    return Err(PasswordGenError::InvalidConfig(
                        "Length must be greater than zero".into(),
                    ));
                }
            }
        }

        // 2. Validar caracteres incluidos/excluidos.
        let valid_includes = self.validate_char_list(&self.include, "include")?;
        if valid_includes.is_empty() {
            self.include = None;
        }

        let valid_excludes = self.validate_char_list(&self.exclude, "exclude")?;
        if valid_excludes.is_empty() {
            self.exclude = None;
        }

        // 3. Verificar colisiones entre include/exclude.

        if !valid_includes.is_empty() && !valid_excludes.is_empty() {
            let include_set: std::collections::HashSet<_> = valid_includes.iter().collect();
            let exclude_set: std::collections::HashSet<_> = valid_excludes.iter().collect();

            let collisions: Vec<_> = include_set.intersection(&exclude_set).collect();

            if !collisions.is_empty() {
                let colliding_chars: String = collisions.iter().map(|&&c| c).collect();
                // In the future
                // pub enum ValidationResult {
                //     Ok,
                //     Warnings(Vec<String>),
                //     Error(PasswordGenError),
                // }
                println!(
                    "[WARN] Characters {} were excluded but reintroduced via include.",
                    colliding_chars
                );
            }
        }

        // 4. Ajustar restricciones opcionales a `None` si no son válidas.
        if let Some(value) = self.max_consecutive {
            if value == 0 {
                self.max_consecutive = None
            }
        }

        if let Some(value) = self.min_entropy_bits {
            if value == 0 {
                self.min_entropy_bits = None
            }
        }

        Ok(())
    }
}

/// Define la restricción de longitud, que puede ser un rango (con min y max) o un valor exacto.
#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Constraint {
    Range { min: usize, max: usize },
    Exact(usize),
}

/// Define la restricción de longitud, que puede ser un rango (con min y max) o un valor exacto.
#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum RulesConstraint {
    Range { min: usize, max: Option<usize> },
    Exact(usize),
}

/// Representa el conjunto de caracteres disponibles para la generación de contraseñas.
/// El mapeo es flexible gracias al flatten del HashMap, que asocia nombres de charset
/// con su tipo de restricción (`CharsetConstraint`).
#[derive(Deserialize, Clone, Debug)]
pub struct Charset {
    #[serde(flatten)]
    pub charsets: HashMap<String, CharsetConstraint>,
}

impl Validator for Charset {
    fn validate(&mut self) -> Result<()> {
        if self.charsets.is_empty() {
            return Err(PasswordGenError::InvalidConfig(
                "No character set has been specified.".into(),
            ));
        }

        for (name, charset) in &self.charsets {
            match charset {
                CharsetConstraint::Multiple(chars) => {}
                CharsetConstraint::One(chars) => {}
            }
        }

        Ok(())
    }
}

/// Define restricciones para un charset particular, que puede ser un único String o una lista.
#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum CharsetConstraint {
    Multiple(Vec<String>),
    One(String),
}
