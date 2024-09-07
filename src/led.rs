use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub type Result<T> = std::result::Result<T, LedError>;

#[derive(PartialEq, Debug)]
pub enum LedError {
    IoError(()),
    ParseIntError(std::num::ParseIntError),
}

impl From<io::Error> for LedError {
    fn from(_error: io::Error) -> Self {
        LedError::IoError(())
    }
}

impl From<std::num::ParseIntError> for LedError {
    fn from(error: std::num::ParseIntError) -> Self {
        LedError::ParseIntError(error)
    }
}

pub struct Led {
    max_brightness: u32,
    brightness_path: PathBuf,
    max_brightness_path: PathBuf,
}

impl Led {
    pub fn new(name: &str) -> Result<Led> {
        let base = Path::new("/sys/class/leds/");

        let led_dir = base.join(name);
        let brightness_path = led_dir.join("brightness");
        let max_brightness_path = led_dir.join("max_brightness");

        let mut led = Led {
            max_brightness: 0,
            brightness_path,
            max_brightness_path,
        };

        led.set_max_brightness()?;
        Ok(led)
    }

    pub fn set_max_brightness(&mut self) -> Result<()> {
        let line = fs::read_to_string(self.max_brightness_path.clone())?;
        self.max_brightness = line.trim().parse()?;
        Ok(())
    }

    pub fn set_brightness(&self, brightness: u32) -> Result<()> {
        let mut file = File::create(self.brightness_path.clone())?;
        let brightness_str = brightness.to_string();
        file.write_all(brightness_str.as_bytes())?;
        Ok(())
    }

    pub fn set_value(&self, on: bool) -> Result<()> {
        self.set_brightness(if on { self.max_brightness } else { 0 })
    }
}

pub fn get_led_names() -> Result<Vec<String>> {
    let mut result = Vec::new();
    let paths = fs::read_dir("/sys/class/leds/")?;
    for path in paths {
        let path = path?.file_name();
        let path_str = path.to_str().unwrap();
        result.push(path_str.to_owned());
    }
    result.sort();
    Ok(result)
}

pub fn print_leds_available() -> Result<()> {
    eprintln!("Leds available:\n");
    for led in get_led_names()? {
        eprintln!("    {}", led);
    }
    Ok(())
}
