use regex::Regex;
use thiserror::Error;

#[derive(Debug)]
pub enum PatternElement {
    Set {
        name: String,
        min: usize,
        max: usize,
        negate: bool,
    },
    Wildcard, // Representa el carácter '*' que indica el final del patrón con caracteres aleatorios
}

pub fn expand_unicode_sequences(sequences: &Vec<String>) -> Vec<char> {
    let mut chars = Vec::new();
    let range_re = Regex::new(r"U\+([0-9A-Fa-f]{4,6})-U\+([0-9A-Fa-f]{4,6})").unwrap();
    let single_re = Regex::new(r"U\+([0-9A-Fa-f]{4,6})").unwrap();

    for seq in sequences {
        if let Some(captures) = range_re.captures(seq) {
            let start = u32::from_str_radix(&captures[1], 16).unwrap();
            let end = u32::from_str_radix(&captures[2], 16).unwrap();
            for code in start..=end {
                if let Some(ch) = std::char::from_u32(code) {
                    chars.push(ch);
                }
            }
        } else if let Some(captures) = single_re.captures(seq) {
            let code = u32::from_str_radix(&captures[1], 16).unwrap();
            if let Some(ch) = std::char::from_u32(code) {
                chars.push(ch);
            }
        }
    }
    chars
}

pub fn parse_pattern(pattern_str: &str) -> Option<Vec<PatternElement>> {
    let mut elements = Vec::new();
    let mut remaining = pattern_str.trim();

    // Regex para capturar un bloque con conjuntos y cantidades
    let re = Regex::new(r"^\((\^?)([^\(\)\,\{\}\*]+)\)\{(\d+|\d+,|\d+,\d+)\}").unwrap();

    while !remaining.is_empty() {
        // Detecta si hay un '*' en cualquier parte del patrón
        if remaining.starts_with('*') {
            elements.push(PatternElement::Wildcard); // Wildcard para manejar el *
            remaining = &remaining[1..];
            continue;
        }

        // Captura un bloque con conjunto y cantidad
        if let Some(captures) = re.captures(remaining) {
            let negate = captures.get(1).map_or("", |m| m.as_str()) == "^";
            let set_name = captures.get(2)?.as_str().to_string();
            let quantity = captures.get(3)?.as_str();

            let (min, max) = if quantity.contains(',') {
                let parts: Vec<&str> = quantity.split(',').collect();
                let min = parts[0].parse::<usize>().ok()?;
                let max = if parts.len() > 1 && !parts[1].is_empty() {
                    parts[1].parse::<usize>().ok()?
                } else {
                    min
                };
                (min, max)
            } else {
                let num = quantity.parse::<usize>().ok()?;
                (num, num)
            };

            elements.push(PatternElement::Set {
                name: set_name,
                min,
                max,
                negate,
            });

            // Elimina el bloque ya procesado del string restante
            let match_str = captures.get(0)?.as_str();
            remaining = &remaining[match_str.len()..];
        } else {
            // Si no se puede parsear más, retorna None
            return None;
        }
    }

    Some(elements)
}

pub fn has_max_consecutive(chars: &Vec<char>, max_consecutive: usize) -> bool {
    let mut count = 1;
    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] {
            count += 1;
            if count > max_consecutive {
                return true;
            }
        } else {
            count = 1;
        }
    }
    false
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("El valor {0} en el archivo de configuracion no es de tipo `password`")]
    InvalidFileType(String),
}
