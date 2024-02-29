/// 8 not yet compressed letters
pub(crate) type NormalArray = [Letter; 8];

/// A Compressed array of 8 letters
pub(crate) type CompressedArray = [u8; 5];

/// Represents 8 letters each has 5 bits
/// total 5 bytes
#[derive(Debug,Clone, Copy, PartialEq)]
#[repr(C)]
pub(crate) struct CompressedLetters(pub(crate) CompressedArray);


/// Array representing the parsed letters
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(C)]
pub(crate) struct ParsedLetters(pub(crate) NormalArray);

/// byte conversion to more easily manipulate the bytes
#[repr(C)]
union NumArr {
    num: u64,
    arr: CompressedArray,
}

impl CompressedLetters {
    /// Compresses the already parsed array 
    pub(crate) fn from_parsed_words(value: &ParsedLetters) -> Self {
        let mut num_repr: u64 = 0;

        for (i, letter) in value.0.iter().enumerate() {
            let letter_u8 = unsafe { std::mem::transmute::<Letter, u8>(*letter) };
            let letter_u64 = (letter_u8 as u64) << ((7 - i) as u8 * 5);
            num_repr += letter_u64;
        }

        assert!(num_repr < 2_u64.pow(40), "num_repr Is too large");

        let union_repr = NumArr { num: num_repr };

        unsafe { Self(union_repr.arr) }
    }
}

impl ParsedLetters {
    /// Decompression
    pub(crate) fn from_compressed_words(value: &CompressedLetters) -> Self {
        let mut num_repr = unsafe { NumArr { arr: value.0 }.num };

        let mut decoded = Self([Letter::default(); 8]);

        for letter in decoded.0.iter_mut().rev() {
            let letter_u8 = (num_repr % 32) as u8; // 32 == 2**5
            num_repr /= 32;
            *letter = unsafe { std::mem::transmute::<u8, Letter>(letter_u8) }; // Letter as u8 <3
        }

        decoded
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Default)]
pub(crate) enum Letter {
    #[default] Space,
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
    NextIsWeird, // s -> š || a -> á
    Chord,
    Comma, // Dot represented as two Commas
    Enter,
}

impl Letter {
    pub(crate) fn new(character: char) -> Vec<Self> {
        match character.to_uppercase().to_string().as_str() {
            "A" => vec![Letter::A],
            "B" => vec![Letter::B],
            "C" => vec![Letter::C],
            "D" => vec![Letter::D],
            "E" => vec![Letter::E],
            "F" => vec![Letter::F],
            "G" => vec![Letter::G],
            "H" => vec![Letter::H],
            "I" => vec![Letter::I],
            "J" => vec![Letter::J],
            "K" => vec![Letter::K],
            "L" => vec![Letter::L],
            "M" => vec![Letter::M],
            "N" => vec![Letter::N],
            "O" => vec![Letter::O],
            "P" => vec![Letter::P],
            "Q" => vec![Letter::Q],
            "R" => vec![Letter::R],
            "S" => vec![Letter::S],
            "T" => vec![Letter::T],
            "U" => vec![Letter::U],
            "V" => vec![Letter::V],
            "W" => vec![Letter::W],
            "X" => vec![Letter::X],
            "Y" => vec![Letter::Y],
            "Z" => vec![Letter::Z],
            "Ě" => vec![Letter::NextIsWeird, Letter::E],
            "Š" => vec![Letter::NextIsWeird, Letter::S],
            "Č" => vec![Letter::NextIsWeird, Letter::C],
            "Ř" => vec![Letter::NextIsWeird, Letter::R],
            "Ž" => vec![Letter::NextIsWeird, Letter::Z],
            "Ď" => vec![Letter::NextIsWeird, Letter::D],
            "Ý" => vec![Letter::NextIsWeird, Letter::Y],
            "Á" => vec![Letter::NextIsWeird, Letter::A],
            "Í" => vec![Letter::NextIsWeird, Letter::I],
            "É" => vec![Letter::NextIsWeird, Letter::NextIsWeird, Letter::E],
            "Ó" => vec![Letter::NextIsWeird, Letter::O],
            "Ú" | "Ů" => vec![Letter::NextIsWeird, Letter::U],
            "#" => vec![Letter::Chord],
            "," => vec![Letter::Comma],
            "." => vec![Letter::Comma, Letter::Comma],
            " " => vec![Letter::Space],
            "\n" => vec![Letter::Enter],
            e => panic!("Unknown Letter: '{e}'"),
        }
    }
}
impl std::fmt::Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        let letter = match self {
            Letter::Space => " ",
            Letter::A => "a",
            Letter::B => "b",
            Letter::C => "c",
            Letter::D => "d",
            Letter::E => "e",
            Letter::F => "f",
            Letter::G => "g",
            Letter::H => "h",
            Letter::I => "i",
            Letter::J => "j",
            Letter::K => "k",
            Letter::L => "l",
            Letter::M => "m",
            Letter::N => "n",
            Letter::O => "o",
            Letter::P => "p",
            Letter::Q => "q",
            Letter::R => "r",
            Letter::S => "s",
            Letter::T => "t",
            Letter::U => "u",
            Letter::V => "v",
            Letter::W => "w",
            Letter::X => "x",
            Letter::Y => "y",
            Letter::Z => "z",
            Letter::Comma => ",",
            Letter::Enter => "\n",
            l => panic!("{:#?} cannot be parsed as a String", l)
        };

        write!(f, "{}", letter)
    }
}