use super::{
    config::Config,
    utils::{parse_pattern, PatternElement, ValidationError},
};

struct Semver {
    mayor: u8,
    minor: u8,
    parct: u8,
}
impl Semver {
    fn from(version: String) -> Self {
        Semver {}
    }
}

pub fn validate_config(config: &Config) -> Result<(), ValidationError> {
    let props = &config.properties;

    if !(props.generation_type == "password") {
        return Err(ValidationError::InvalidFileType(
            props.generation_type.clone(),
        ));
    }

    Semver::from(props.version.clone());

    Ok(())
}

// if let Some(rules) = &config.rules {
//     if let Some(pattern_str) = &rules.pattern {
//         validate_pattern(pattern_str, config)?;
//     }
// }

// fn validate_pattern(pattern_str: &str, config: &Config) -> ValResult {
//     // Analizar el pattern: Parsear el patrón para extraer los bloques, conjuntos y cantidades.
//     let patterns = parse_pattern(pattern_str)
//         .ok_or(ValidationError::InvalidPattern(pattern_str.to_string()))?;
//     let languages_sets = &config.languages;
//     let requirements = &config.requirements;

//     // Calcular la longitud mínima y máxima posible del pattern
//     let mut pattern_min_length = 0;
//     let mut pattern_max_length = 0;
//     let mut pattern_has_wildcard = false;

//     for pattern in &patterns {
//         match pattern {
//             PatternElement::Set { name, min, max, .. } => {
//                 // Verificar que los conjuntos utilizados en el pattern existen en las secciones de idiomas.
//                 if !languages_sets
//                     .iter()
//                     .any(|(_, v)| v.sets.contains_key(name))
//                 {
//                     return Err(ValidationError::SetNotFound(name.to_string()));
//                 }

//                 if requirements.sets.contains_key(name) {
//                     let set_req = &requirements.sets[name];

//                     match set_req {
//                         super::config::SetRequirement::Range { min, max } => {
//                             if let Some(max) = max {
//                                 if min > max {
//                                     return Err(ValidationError::InvalidLengthBounds(
//                                         name.to_string(),
//                                     ));
//                                 }
//                             } else {

//                             }
//                         }
//                         super::config::SetRequirement::Exact(length) => {
//                             if *min != *length {
//                                 return Err(ValidationError::InvalidLengthBounds(name.to_string()));
//                             }
//                         }
//                     }
//                 }

//                 // Sumar las cantidades mínimas y máximas de cada bloque.
//                 pattern_min_length += min;
//                 pattern_max_length += max;
//             }
//             PatternElement::Wildcard => {
//                 pattern_has_wildcard = true;
//             }
//         }
//     }

//     Ok(())
// }
