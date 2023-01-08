use dotenv::dotenv;
use std::fs::File;
use std::io::{self, Write};
use std::error::Error;

pub fn set_base_path (mut path: String)  -> Result<(), Box<dyn Error>> {
    if &path.chars().last().unwrap().to_string() != "/" {
        path = [&path, "/"].concat();
    };

    let mut file = File::create(".env")?;
    write!(file, "BASE_PATH={}", path)?;
    file.flush()?;
    Ok(())
}

pub fn reset_base_path ()  -> Result<(), Box<dyn Error>> {
    let mut file = File::create(".env")?;
    write!(file, "")?;
    file.flush()?;
    Ok(())
}