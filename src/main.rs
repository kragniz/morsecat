use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use MorseElement::*;
use MorseSignal::*;
use MorseValue::*;

type Result<T> = std::result::Result<T, LedError>;

#[derive(PartialEq, Debug)]
enum LedError {
    IoError(()),
    ParseIntError(std::num::ParseIntError),
    MorseParseError(String),
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
    fn to_signal(&self) -> MorseSignal {
        match self {
            Dot => On(1),
            Dash => On(3),
            Gap => Off(1),
            LetterGap => Off(3),
            WordGap => Off(1),
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
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl MorseValue {
    fn from(c: char) -> Result<Self> {
        match c.to_ascii_uppercase() {
            ' ' => Ok(Space),
            'A' => Ok(A),
            'B' => Ok(B),
            'C' => Ok(C),
            'D' => Ok(D),
            'E' => Ok(E),
            'F' => Ok(F),
            'G' => Ok(G),
            'H' => Ok(H),
            'I' => Ok(I),
            'J' => Ok(J),
            'K' => Ok(K),
            'L' => Ok(L),
            'M' => Ok(M),
            'N' => Ok(N),
            'O' => Ok(O),
            'P' => Ok(P),
            'Q' => Ok(Q),
            'R' => Ok(R),
            'S' => Ok(S),
            'T' => Ok(T),
            'U' => Ok(U),
            'V' => Ok(V),
            'W' => Ok(W),
            'X' => Ok(X),
            'Y' => Ok(Y),
            'Z' => Ok(Z),
            '0' => Ok(Zero),
            '1' => Ok(One),
            '2' => Ok(Two),
            '3' => Ok(Three),
            '4' => Ok(Four),
            '5' => Ok(Five),
            '6' => Ok(Six),
            '7' => Ok(Seven),
            '8' => Ok(Eight),
            '9' => Ok(Nine),
            _ => Err(LedError::MorseParseError(format!(
                "Character not allowed: {}",
                c
            ))),
        }
    }

    fn to_morse_elements(&self) -> Vec<MorseElement> {
        let elements = match self {
            Space => vec![WordGap],
            A => vec![Dot, Dash],
            B => vec![Dash, Dot, Dot, Dot],
            C => vec![Dash, Dot, Dash, Dot],
            D => vec![Dash, Dot, Dot],
            E => vec![Dot],
            F => vec![Dot, Dot, Dash, Dot],
            G => vec![Dash, Dash, Dot],
            H => vec![Dot, Dot, Dot, Dot],
            I => vec![Dot, Dot],
            J => vec![Dot, Dash, Dash, Dash],
            K => vec![Dash, Dot, Dash],
            L => vec![Dot, Dash, Dot, Dot],
            M => vec![Dash, Dash],
            N => vec![Dash, Dot],
            O => vec![Dash, Dash, Dash],
            P => vec![Dot, Dash, Dash, Dot],
            Q => vec![Dash, Dash, Dot, Dash],
            R => vec![Dot, Dash, Dot],
            S => vec![Dot, Dot, Dot],
            T => vec![Dash],
            U => vec![Dot, Dot, Dash],
            V => vec![Dot, Dot, Dot, Dash],
            W => vec![Dot, Dash, Dash],
            X => vec![Dash, Dot, Dot, Dash],
            Y => vec![Dash, Dot, Dash, Dash],
            Z => vec![Dash, Dash, Dot, Dot],
            Zero => vec![Dash, Dash, Dash, Dash, Dash],
            One => vec![Dot, Dash, Dash, Dash, Dash],
            Two => vec![Dot, Dot, Dash, Dash, Dash],
            Three => vec![Dot, Dot, Dot, Dash, Dash],
            Four => vec![Dot, Dot, Dot, Dot, Dash],
            Five => vec![Dot, Dot, Dot, Dot, Dot],
            Six => vec![Dash, Dot, Dot, Dot, Dot],
            Seven => vec![Dash, Dash, Dot, Dot, Dot],
            Eight => vec![Dash, Dash, Dash, Dot, Dot],
            Nine => vec![Dash, Dash, Dash, Dash, Dot],
        };
        let len = elements.len() * 2 - 1;
        elements
            .into_iter()
            .flat_map(|e| vec![e, MorseElement::Gap])
            .take(len)
            .collect::<Vec<MorseElement>>()
    }
}

fn morse_values_to_elements(values: Vec<MorseValue>) -> Vec<MorseElement> {
    let mut result = Vec::new();
    for value in values {
        result.extend(value.to_morse_elements());
        result.push(MorseElement::LetterGap);
    }
    result.pop();
    result
}

#[derive(PartialEq, Clone, Debug)]
enum MorseSignal {
    On(u64),
    Off(u64),
}

fn morse_elements_to_signals(elements: Vec<MorseElement>) -> Vec<MorseSignal> {
    let mut signals = Vec::new();

    let mut current_signal = On(0);
    for e in &elements {
        let next_signal = e.to_signal();

        // Push the current signal if it's different from the new one, otherwise compact it
        match (&current_signal, &next_signal) {
            (On(_), Off(_)) | (Off(_), On(_)) => {
                signals.push(current_signal.clone());
                current_signal = next_signal;
            }
            (On(n), On(m)) => {
                current_signal = On(n + m);
            }
            (Off(n), Off(m)) => {
                current_signal = Off(n + m);
            }
        }
    }
    signals.push(current_signal);
    signals.push(Off(7));
    signals
}

fn string_to_morse_values(s: &str) -> Result<Vec<MorseValue>> {
    s.chars().map(MorseValue::from).collect()
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

    let values = string_to_morse_values("SOS").expect("Error parsing message");
    let elements = morse_values_to_elements(values);
    let signals = morse_elements_to_signals(elements);

    for signal in signals {
        let (on, duration) = match signal {
            On(d) => (true, d),
            Off(d) => (false, d),
        };
        led.set_value(on).expect("Error setting brightness");
        std::thread::sleep(std::time::Duration::from_millis(duration * 300));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_morse_message_single_letter() {
        assert_eq!(
            morse_values_to_elements(vec![MorseValue::A]),
            vec![MorseElement::Dot, MorseElement::Gap, MorseElement::Dash]
        );
    }

    #[test]
    fn test_encode_morse_message_two_letters() {
        assert_eq!(
            morse_values_to_elements(vec![H, I]),
            vec![Dot, Gap, Dot, Gap, Dot, Gap, Dot, LetterGap, Dot, Gap, Dot]
        );
    }

    #[test]
    fn test_encode_morse_message_two_words() {
        #[rustfmt::skip]
        assert_eq!(
            morse_values_to_elements(vec![H, I, Space, H, I]),
            vec![
                Dot, Gap, Dot, Gap, Dot, Gap, Dot, LetterGap,
                Dot, Gap, Dot, LetterGap,
                WordGap, LetterGap,
                Dot, Gap, Dot, Gap, Dot, Gap, Dot, LetterGap,
                Dot, Gap, Dot,
            ],
        );
    }

    #[test]
    fn test_encode_morse_message_two_letters_with_space() {
        assert_eq!(
            morse_values_to_elements(vec![E, Space, T]),
            vec![Dot, LetterGap, WordGap, LetterGap, Dash]
        );
    }

    #[test]
    fn test_morse_elements_to_signals() {
        assert_eq!(
            morse_elements_to_signals(vec![Dot, Gap, Dash]),
            vec![On(1), Off(1), On(3), Off(7)]
        );
    }

    #[test]
    fn test_morse_elements_to_signals_compact() {
        assert_eq!(
            morse_elements_to_signals(vec![Dot, LetterGap, WordGap, LetterGap, Dash]),
            vec![On(1), Off(7), On(3), Off(7)]
        );
    }

    #[test]
    fn test_morse_elements_to_signals_multiple_words() {
        #[rustfmt::skip]
        assert_eq!(
            morse_elements_to_signals(
                vec![
                    Dot, Gap, Dot, Gap, Dot, Gap, Dot, LetterGap,
                    Dot, Gap, Dot, LetterGap,
                    WordGap, LetterGap,
                    Dot, Gap, Dot, Gap, Dot, Gap, Dot, LetterGap,
                    Dot, Gap, Dot,
                ]
            ),
            vec![
                On(1), Off(1), On(1), Off(1), On(1), Off(1), On(1), Off(3),
                On(1), Off(1), On(1),
                Off(7),
                On(1), Off(1), On(1), Off(1), On(1), Off(1), On(1), Off(3),
                On(1), Off(1), On(1),
                Off(7),
            ]
        );
    }

    #[test]
    fn test_string_to_morse_values() {
        assert_eq!(string_to_morse_values("SOS"), Ok(vec![S, O, S]));
    }

    #[test]
    fn test_string_to_morse_values_lower() {
        assert_eq!(
            string_to_morse_values("sos sos"),
            Ok(vec![S, O, S, Space, S, O, S])
        );
    }

    #[test]
    fn test_string_to_morse_values_numbers() {
        assert_eq!(
            string_to_morse_values("123 456"),
            Ok(vec![One, Two, Three, Space, Four, Five, Six])
        );
    }

    #[test]
    fn test_string_to_morse_values_error() {
        assert_eq!(
            string_to_morse_values("SoS£"),
            Err(LedError::MorseParseError(
                "Character not allowed: £".to_string()
            ))
        );
    }
}
