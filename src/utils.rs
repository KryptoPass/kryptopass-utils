use thiserror::Error;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Error, Debug)]
pub enum UtilsError {
    #[error("Formato inválido: {0}")]
    InvalidFormat(String),

    #[error("Rango inválido (start > end): {0}")]
    InvalidRange(String),

    #[error("Código Unicode inválido: {0}")]
    InvalidUnicodeCode(String),
}

pub type Result<T> = std::result::Result<T, UtilsError>;

/// Función principal para parsear rangos o cadenas.
pub fn parse_unicode(input: &str) -> Result<Vec<char>> {
    // 1. Si contiene un guion (-), podrían ser 2 casos:
    //    a) Rango de Unicode: U+XXXX-U+YYYY
    //    b) Rango simple (como "a-z" o "0-9"), pero solo si ambos lados tienen exactamente 1 grafema
    //    c) Caso "AA-B" => se trata como literal (A, A, -, B)
    if input.contains('-') {
        // Intentar parsear rango Unicode "U+...-U+..."
        if is_unicode_hex_range(input) {
            return parse_unicode_hex_range(input);
        } else if is_single_grapheme_range(input) {
            // Rango tipo "a-z", "0-9", "Ñ-ß", etc.
            return parse_single_char_range(input);
        } else {
            // No cumple con las reglas de rango => tomar como literal
            return Ok(input.chars().collect());
        }
    } else {
        // 2. No contiene guion:
        //    a) podría ser un único codepoint en formato U+XXXX
        //    b) podría ser una cadena "ABC" / "Ñ" / "🙂"
        //    c) podría ser algo no válido
        if is_single_unicode_hex(input) {
            // Parsear algo tipo "U+1F64F"
            let code = parse_single_unicode_hex(input)?;
            let ch = char::from_u32(code).ok_or(UtilsError::InvalidUnicodeCode(input.to_string()))?;
            return Ok(vec![ch]);
        } else {
            // Devolver cada char/grafema como un char
            // (si prefieres separar por grafemas, lo hacemos con graphemes)
            // Ej. "ABC" => ['A','B','C'], "Ñ" => ['Ñ']
            // Si quisieras tratar grafemas compuestos, deberías recoger graphemes.
            let mut chars = Vec::new();
            for g in input.graphemes(true) {
                // OJO: un grapheme puede tener > 1 char real
                // Si deseas unirlos, tendrías que decidir cómo.
                // Aquí simplificamos tomando el primer char del grapheme.
                // O, si quieres cada "grapheme" como un solo `char`,
                // lamentablemente en Rust no existe un `char` que contenga
                // un grapheme multicodepoint. Toca convertirlo a String.
                for c in g.chars() {
                    chars.push(c);
                }
            }
            return Ok(chars);
        }
    }
}

/// Verifica si la cadena es un rango Unicode en formato "U+XXXX-U+YYYY"
fn is_unicode_hex_range(input: &str) -> bool {
    let parts: Vec<&str> = input.split('-').collect();
    if parts.len() != 2 {
        return false;
    }
    // Ambas partes deberían empezar con "U+"
    parts[0].starts_with("U+") && parts[1].starts_with("U+")
}

/// Parsea rango Unicode "U+XXXX-U+YYYY"
fn parse_unicode_hex_range(range: &str) -> Result<Vec<char>> {
    let parts: Vec<&str> = range.split('-').collect();
    let start_code = parse_single_unicode_hex(parts[0])?;
    let end_code = parse_single_unicode_hex(parts[1])?;

    if start_code > end_code {
        return Err(UtilsError::InvalidRange(range.to_string()));
    }

    let mut chars = Vec::new();
    for code in start_code..=end_code {
        if let Some(c) = char::from_u32(code) {
            chars.push(c);
        }
    }
    Ok(chars)
}

/// Verifica si es un rango de un solo grafema a un solo grafema, como "a-z", "0-9", "Ñ-ß", etc.
fn is_single_grapheme_range(input: &str) -> bool {
    let parts: Vec<&str> = input.split('-').collect();
    if parts.len() != 2 {
        return false;
    }
    // Cada parte debe tener exactamente 1 grapheme
    parts[0].graphemes(true).count() == 1 && parts[1].graphemes(true).count() == 1
}

/// Parsea un rango de un solo grafema, p. ej. "a-z", "0-9"
fn parse_single_char_range(range: &str) -> Result<Vec<char>> {
    let parts: Vec<&str> = range.split('-').collect();
    // Podemos tomar el primer char de cada grapheme
    let start_char = parts[0].chars().next().unwrap();
    let end_char = parts[1].chars().next().unwrap();

    let start = start_char as u32;
    let end = end_char as u32;

    if start > end {
        return Err(UtilsError::InvalidRange(range.to_string()));
    }

    let mut chars = Vec::new();
    for code in start..=end {
        if let Some(c) = char::from_u32(code) {
            chars.push(c);
        }
    }
    Ok(chars)
}

/// Verifica si la cadena es un único codepoint en formato "U+XXXX"
fn is_single_unicode_hex(input: &str) -> bool {
    input.starts_with("U+") && !input.contains('-')
}

/// Parsea un único codepoint "U+XXXX" a u32
fn parse_single_unicode_hex(input: &str) -> Result<u32> {
    if !input.starts_with("U+") {
        return Err(UtilsError::InvalidFormat(input.to_string()));
    }
    // Quitar "U+" y parsear hex
    let hex_part = &input[2..];
    let code_u32 = u32::from_str_radix(hex_part, 16).map_err(|_| UtilsError::InvalidFormat(input.to_string()))?;
    Ok(code_u32)
}
