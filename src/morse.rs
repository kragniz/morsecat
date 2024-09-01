use MorseElement::*;
use MorseSignal::*;
use MorseValue::*;

#[derive(PartialEq, Debug)]
pub enum MorseElement {
    Dot,
    Dash,
    Gap,
    LetterGap,
    WordGap,
}

impl MorseElement {
    pub fn to_signal(&self) -> MorseSignal {
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
pub enum MorseValue {
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
    Period,
    Comma,
    Question,
    Apostrophe,
    Quotation,
    Hyphen,
    Slash,
    OpenParenthesis,
    CloseParenthesis,
    At,
    Colon,
}

impl MorseValue {
    pub fn from(c: char) -> Result<Self, String> {
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
            '.' => Ok(Period),
            ',' => Ok(Comma),
            '?' => Ok(Question),
            '\'' => Ok(Apostrophe),
            '"' => Ok(Quotation),
            '-' => Ok(Hyphen),
            '/' => Ok(Slash),
            '(' => Ok(OpenParenthesis),
            ')' => Ok(CloseParenthesis),
            '@' => Ok(At),
            ':' => Ok(Colon),
            _ => Err(format!("Character not allowed: {}", c)),
        }
    }

    pub fn to_morse_elements(&self) -> Vec<MorseElement> {
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
            Period => vec![Dot, Dash, Dot, Dash, Dot, Dash],
            Comma => vec![Dash, Dash, Dot, Dot, Dash, Dash],
            Question => vec![Dot, Dot, Dash, Dash, Dot, Dot],
            Apostrophe => vec![Dot, Dash, Dash, Dash, Dash, Dot],
            Quotation => vec![Dot, Dash, Dot, Dot, Dash, Dot],
            Hyphen => vec![Dash, Dot, Dot, Dot, Dot, Dash],
            Slash => vec![Dash, Dot, Dot, Dash, Dot],
            OpenParenthesis => vec![Dash, Dot, Dash, Dash, Dot],
            CloseParenthesis => vec![Dash, Dot, Dash, Dash, Dot, Dash],
            At => vec![Dot, Dash, Dash, Dot, Dash, Dot],
            Colon => vec![Dash, Dash, Dash, Dot, Dot, Dot],
        };
        let len = elements.len() * 2 - 1;
        elements
            .into_iter()
            .flat_map(|e| vec![e, MorseElement::Gap])
            .take(len)
            .collect::<Vec<MorseElement>>()
    }
}

pub fn values_to_elements(values: Vec<MorseValue>) -> Vec<MorseElement> {
    let mut result = Vec::new();
    for value in values {
        result.extend(value.to_morse_elements());
        result.push(MorseElement::LetterGap);
    }
    result.pop();
    result
}

#[derive(PartialEq, Clone, Debug)]
pub enum MorseSignal {
    On(u64),
    Off(u64),
}

pub fn elements_to_signals(elements: Vec<MorseElement>) -> Vec<MorseSignal> {
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

pub fn string_to_values(s: &str) -> Result<Vec<MorseValue>, String> {
    s.chars().map(MorseValue::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_morse_message_single_letter() {
        assert_eq!(
            values_to_elements(vec![MorseValue::A]),
            vec![MorseElement::Dot, MorseElement::Gap, MorseElement::Dash]
        );
    }

    #[test]
    fn test_encode_morse_message_two_letters() {
        assert_eq!(
            values_to_elements(vec![H, I]),
            vec![Dot, Gap, Dot, Gap, Dot, Gap, Dot, LetterGap, Dot, Gap, Dot]
        );
    }

    #[test]
    fn test_encode_morse_message_two_words() {
        #[rustfmt::skip]
        assert_eq!(
            values_to_elements(vec![H, I, Space, H, I]),
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
            values_to_elements(vec![E, Space, T]),
            vec![Dot, LetterGap, WordGap, LetterGap, Dash]
        );
    }

    #[test]
    fn test_morse_elements_to_signals() {
        assert_eq!(
            elements_to_signals(vec![Dot, Gap, Dash]),
            vec![On(1), Off(1), On(3), Off(7)]
        );
    }

    #[test]
    fn test_morse_elements_to_signals_compact() {
        assert_eq!(
            elements_to_signals(vec![Dot, LetterGap, WordGap, LetterGap, Dash]),
            vec![On(1), Off(7), On(3), Off(7)]
        );
    }

    #[test]
    fn test_morse_elements_to_signals_multiple_words() {
        #[rustfmt::skip]
        assert_eq!(
            elements_to_signals(
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
        assert_eq!(string_to_values("SOS"), Ok(vec![S, O, S]));
    }

    #[test]
    fn test_string_to_morse_values_lower() {
        assert_eq!(
            string_to_values("sos sos"),
            Ok(vec![S, O, S, Space, S, O, S])
        );
    }

    #[test]
    fn test_string_to_morse_values_numbers() {
        assert_eq!(
            string_to_values("123 456"),
            Ok(vec![One, Two, Three, Space, Four, Five, Six])
        );
    }

    #[test]
    fn test_string_to_morse_symbols() {
        assert_eq!(
            string_to_values("hi('@')"),
            Ok(vec![
                H,
                I,
                OpenParenthesis,
                Apostrophe,
                At,
                Apostrophe,
                CloseParenthesis
            ])
        );
    }

    #[test]
    fn test_string_to_morse_values_error() {
        assert_eq!(
            string_to_values("SoS£"),
            Err("Character not allowed: £".to_string())
        );
    }
}
