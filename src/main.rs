use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

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
    brightness_path: PathBuf,
    max_brightness_path: PathBuf,
}

impl Led {
    fn new(name: &str) -> Result<Led> {
        let base = Path::new("/sys/class/leds/");

        let led_dir = base.join(name);
        let brightness_path = led_dir.join("brightness");
        let max_brightness_path = led_dir.join("max_brightness");

        println!("Using {}", led_dir.display());

        let mut led = Led {
            max_brightness: 0,
            brightness_path,
            max_brightness_path,
        };

        led.set_max_brightness()?;
        Ok(led)
    }

    fn set_max_brightness(&mut self) -> Result<()> {
        let line = fs::read_to_string(self.max_brightness_path.clone())?;
        self.max_brightness = line.trim().parse()?;
        Ok(())
    }

    fn set_brightness(&self, brightness: u32) -> Result<()> {
        let mut file = File::create(self.brightness_path.clone())?;
        let brightness_str = brightness.to_string();
        file.write_all(brightness_str.as_bytes())?;
        Ok(())
    }

    fn set_value(&self, on: bool) -> Result<()> {
        self.set_brightness(if on { self.max_brightness } else { 0 })
    }
}

fn get_led_names() -> Result<Vec<String>> {
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

fn print_leds_available() -> Result<()> {
    eprintln!("Leds available:\n");
    for led in get_led_names()? {
        eprintln!("    {}", led);
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <led_name>\n", args.first().unwrap());
        print_leds_available().expect("Error listing leds");
        std::process::exit(1);
    }

    let led_name = args.get(1).unwrap();

    if !get_led_names()
        .expect("Error listing leds")
        .contains(led_name)
    {
        eprintln!("Error: led {} not found\n", led_name);
        print_leds_available().expect("Error listing leds");
        std::process::exit(1);
    }

    let led = Led::new(led_name).expect("Error creating Led");

    let mut value = false;
    loop {
        led.set_value(value).expect("Error setting brightness");
        value = !value;
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
