mod generators;

use generators::PasswordGenerator;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let pg = PasswordGenerator::from_file("example.toml")?;

    let password = pg.generate();

    if let Some(password) = password {
        println!("{}", password);
    } else {
        println!("No se pudo generar la contraseña.");
    }

    Ok(())
}
