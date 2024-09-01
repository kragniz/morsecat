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

#[derive(PartialEq, Debug)]
enum MorseElement {
    Dot,
    Dash,
    Gap,
    LetterGap,
    WordGap,
}

impl MorseElement {
    fn length(&self) -> u64 {
        match self {
            MorseElement::Dot => 1,
            MorseElement::Dash => 3,
            MorseElement::Gap => 1,
            MorseElement::LetterGap => 3,
            MorseElement::WordGap => 1,
        }
    }
}

#[derive(PartialEq, Debug)]
enum MorseValue {
    Space,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl MorseValue {
    fn to_morse_elements(&self) -> Vec<MorseElement> {
        let elements = match self {
            MorseValue::Space => vec![MorseElement::WordGap],
            MorseValue::A => vec![MorseElement::Dot, MorseElement::Dash],
            MorseValue::B => vec![
                MorseElement::Dash,
                MorseElement::Dot,
                MorseElement::Dot,
                MorseElement::Dot,
            ],
            MorseValue::C => vec![
                MorseElement::Dash,
                MorseElement::Dot,
                MorseElement::Dash,
                MorseElement::Dot,
            ],
            MorseValue::D => vec![MorseElement::Dash, MorseElement::Dot, MorseElement::Dot],
            MorseValue::E => vec![MorseElement::Dot],
            MorseValue::F => vec![
                MorseElement::Dot,
                MorseElement::Dot,
                MorseElement::Dash,
                MorseElement::Dot,
            ],
            MorseValue::G => vec![MorseElement::Dash, MorseElement::Dash, MorseElement::Dot],
            MorseValue::H => vec![
                MorseElement::Dot,
                MorseElement::Dot,
                MorseElement::Dot,
                MorseElement::Dot,
            ],
            MorseValue::I => vec![MorseElement::Dot, MorseElement::Dot],
            MorseValue::J => vec![
                MorseElement::Dot,
                MorseElement::Dash,
                MorseElement::Dash,
                MorseElement::Dash,
            ],
            MorseValue::K => vec![MorseElement::Dash, MorseElement::Dot, MorseElement::Dash],
            MorseValue::L => vec![
                MorseElement::Dot,
                MorseElement::Dash,
                MorseElement::Dot,
                MorseElement::Dot,
            ],
            MorseValue::M => vec![MorseElement::Dash, MorseElement::Dash],
            MorseValue::N => vec![MorseElement::Dash, MorseElement::Dot],
            MorseValue::O => vec![MorseElement::Dash, MorseElement::Dash, MorseElement::Dash],
            MorseValue::P => vec![
                MorseElement::Dot,
                MorseElement::Dash,
                MorseElement::Dash,
                MorseElement::Dot,
            ],
            MorseValue::Q => vec![
                MorseElement::Dash,
                MorseElement::Dash,
                MorseElement::Dot,
                MorseElement::Dash,
            ],
            MorseValue::R => vec![MorseElement::Dot, MorseElement::Dash, MorseElement::Dot],
            MorseValue::S => vec![MorseElement::Dot, MorseElement::Dot, MorseElement::Dot],
            MorseValue::T => vec![MorseElement::Dash],
            MorseValue::U => vec![MorseElement::Dot, MorseElement::Dot, MorseElement::Dash],
            MorseValue::V => vec![
                MorseElement::Dot,
                MorseElement::Dot,
                MorseElement::Dot,
                MorseElement::Dash,
            ],
            MorseValue::W => vec![MorseElement::Dot, MorseElement::Dash, MorseElement::Dash],
            MorseValue::X => vec![
                MorseElement::Dash,
                MorseElement::Dot,
                MorseElement::Dot,
                MorseElement::Dash,
            ],
            MorseValue::Y => vec![
                MorseElement::Dash,
                MorseElement::Dot,
                MorseElement::Dash,
                MorseElement::Dash,
            ],
            MorseValue::Z => vec![
                MorseElement::Dash,
                MorseElement::Dash,
                MorseElement::Dot,
                MorseElement::Dot,
            ],
        };
        let len = elements.len() * 2 - 1;
        elements
            .into_iter()
            .flat_map(|e| vec![e, MorseElement::Gap])
            .take(len)
            .collect::<Vec<MorseElement>>()
    }
}

fn encode_morse_message(values: Vec<MorseValue>) -> Vec<MorseElement> {
    let mut result = Vec::new();
    for value in values {
        result.extend(value.to_morse_elements());
        result.push(MorseElement::LetterGap);
    }
    result.pop();
    result
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_morse_message_single_letter() {
        assert_eq!(
            encode_morse_message(vec![MorseValue::A]),
            vec![MorseElement::Dot, MorseElement::Gap, MorseElement::Dash]
        );
    }

    #[test]
    fn test_encode_morse_message_two_letters() {
        use MorseElement::*;
        use MorseValue::*;
        assert_eq!(
            encode_morse_message(vec![H, I]),
            vec![Dot, Gap, Dot, Gap, Dot, Gap, Dot, LetterGap, Dot, Gap, Dot]
        );
    }

    #[test]
    fn test_encode_morse_message_two_words() {
        use MorseElement::*;
        use MorseValue::*;

        #[rustfmt::skip]
        assert_eq!(
            encode_morse_message(vec![H, I, Space, H, I]),
            vec![
                Dot, Gap, Dot, Gap, Dot, Gap, Dot, LetterGap,
                Dot, Gap, Dot, LetterGap,
                WordGap, LetterGap,
                Dot, Gap, Dot, Gap, Dot, Gap, Dot, LetterGap,
                Dot, Gap, Dot,
            ],
        );
    }
}
