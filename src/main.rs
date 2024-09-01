use std::fs::File;
use std::io::{self, BufRead, Write};

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

struct Led {
    max_brightness: u32,
    path: String,
}

impl Led {
    fn new(path: &str) -> Result<Led> {
        let mut led = Led {
            max_brightness: 0,
            path: path.to_owned(),
        };

        led.set_max_brightness()?;
        println!("Max brightness: {}", led.max_brightness);
        Ok(led)
    }

    fn set_max_brightness(&mut self) -> Result<()> {
        let file = File::open(self.path.to_owned() + "max_brightness")?;
        let mut reader = io::BufReader::new(file);
        let mut line = String::new();
        reader.read_line(&mut line)?;
        self.max_brightness = line.trim().parse()?;
        Ok(())
    }

    fn set_brightness(&self, brightness: u32) -> Result<()> {
        let mut file = File::create(self.path.clone() + "brightness")?;
        let brightness_str = brightness.to_string();
        file.write_all(brightness_str.as_bytes())?;
        Ok(())
    }

    fn set_value(&self, on: bool) -> Result<()> {
        self.set_brightness(if on { self.max_brightness } else { 0 })
    }
}

fn main() {
    // let led_path = "/sys/class/leds/tpacpi::lid_logo_dot/";
    let led_path = "/sys/class/leds/input0::capslock/";
    let led = Led::new(led_path).expect("Error creating Led");

    let mut value = false;
    loop {
        led.set_value(value).expect("Error setting brightness");
        value = !value;
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
