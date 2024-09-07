mod led;
mod morse;

use std::io::{stdin, BufRead};

use morse::MorseSignal;

fn show_message(message: &str, led: &led::Led) {
    let values = morse::string_to_values(message).expect("Error parsing message");
    let elements = morse::values_to_elements(values);

    println!(
        "{}",
        &elements
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join("")
    );

    let signals = morse::elements_to_signals(elements);

    for signal in signals {
        let (on, duration) = match signal {
            MorseSignal::On(d) => (true, d),
            MorseSignal::Off(d) => (false, d),
        };
        led.set_value(on).expect("Error setting brightness");
        std::thread::sleep(std::time::Duration::from_millis(duration * 120));
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <led_name>\n", args.first().unwrap());
        led::print_leds_available().expect("Error listing leds");
        std::process::exit(1);
    }

    let led_name = args.get(1).unwrap();

    if !led::get_led_names()
        .expect("Error listing leds")
        .contains(led_name)
    {
        eprintln!("Error: led {} not found\n", led_name);
        led::print_leds_available().expect("Error listing leds");
        std::process::exit(1);
    }

    let led = led::Led::new(led_name).expect("Error creating Led");

    let mut stdin_handle = stdin().lock();
    let mut buf = String::new();

    loop {
        buf.clear();
        let line = stdin_handle.read_line(&mut buf);
        match line {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                show_message(buf.trim(), &led);
            }
            Err(s) => {
                println!("Error getting line: {:?}", s);
            }
        }
    }
}
