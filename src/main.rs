use std::fs::File;
use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, LedError>;

#[derive(Debug)]
enum LedError {
    IoError(io::Error),
    ParseIntError(std::num::ParseIntError),
}

impl From<io::Error> for LedError {
    fn from(error: io::Error) -> Self {
        LedError::IoError(error)
    }
}

impl From<std::num::ParseIntError> for LedError {
    fn from(error: std::num::ParseIntError) -> Self {
        LedError::ParseIntError(error)
    }
}

fn read_max_brightness() -> Result<u32> {
    let file = File::open("/sys/class/leds/tpacpi::lid_logo_dot/max_brightness")?;
    let mut reader = io::BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let max_brightness: u32 = line.trim().parse()?;
    Ok(max_brightness)
}

fn main() {
    println!("Hello, world!");

    let max_brightness = match read_max_brightness() {
        Ok(max_brightness) => max_brightness,
        Err(err) => panic!("Error reading max brightness: {:?}", err),
    };
    println!("Max brightness: {}", max_brightness);
}