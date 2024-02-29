mod letters;

use std::{collections::VecDeque, vec};

use crate::letters::{ParsedLetters, CompressedLetters, Letter, NormalArray};

/// Compressed array of letters
#[derive(PartialEq, Debug)]
pub struct CompressedArr(Vec<CompressedLetters>);

impl CompressedArr {
    /// Parses and Compresses the string
    pub fn new(src: &str) -> Self {
        let decompressed = ParsedArr::new(src);

        let mut compressed = Self(vec![]);
        for octet in decompressed.0 {
            compressed.0.push(CompressedLetters::from_parsed_words(&octet))
        }

        compressed
    }
}
impl std::fmt::Display for CompressedArr {
    /// Converts the array back to the text form
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut final_text = String::default();

        let compressed_arr = &self.0;

        let compressed_arr_len = compressed_arr.len();
        let mut index = 0;

        let mut encoded_arr = ParsedLetters::from_compressed_words(&(compressed_arr[index]))
            .0
            .to_vec();
        let mut letter_que: VecDeque<Letter> = encoded_arr.into();
        index += 1;

        encoded_arr = ParsedLetters::from_compressed_words(&(compressed_arr[index]))
            .0
            .to_vec();
        letter_que.append(&mut encoded_arr.into());
        index += 1;

        while !letter_que.is_empty() {
            let letter = letter_que.pop_front().unwrap();

            // encoded as two characters
            match letter {
                Letter::Comma => {
                    if letter_que[0] == Letter::Comma {
                        letter_que.pop_front();
                        final_text += "."
                    } else {
                        final_text += ","
                    }
                }
                Letter::Chord => {
                    let chord = letter_que.pop_front().unwrap();

                    if cfg!(collored) {
                        final_text += "\x1b[31m";
                        final_text += &chord.to_string();
                        final_text += "\x33[0m";
                    } else {
                        final_text += &chord.to_string();
                    }
                }
                Letter::NextIsWeird => {
                    let weird_letter = letter_que.pop_front().unwrap();
                    final_text += match weird_letter {
                        Letter::E => {
                            if letter_que[0] == Letter::NextIsWeird {
                                letter_que.pop_front();
                                "é"
                            } else {
                                "ě"
                            }
                        }
                        Letter::S => "š",
                        Letter::C => "č",
                        Letter::R => "ř",
                        Letter::Z => "ž",
                        Letter::D => "ď",
                        Letter::Y => "ý",
                        Letter::I => "í",
                        Letter::O => "ó",
                        Letter::U => "ú",
                        letter => panic!("Cannot convert '{:#?}' to signed", letter),
                    }
                }
                // single letter
                letter => final_text += &letter.to_string(),
            }

            if letter_que.len() < 8 && index < compressed_arr_len {
                let encoded_arr = ParsedLetters::from_compressed_words(&(compressed_arr[index])).0;
                letter_que.append(&mut encoded_arr.into());
                index += 1;
            }
        }

        write!(f, "{}", final_text)
    }
}

/// Array of parsed words
#[derive(PartialEq, Debug)]
pub struct ParsedArr(Vec<ParsedLetters>);

impl ParsedArr {
    /// Parses the string into `ParsedArr`
    pub fn new(src: &str) -> Self {
        let mut letter_buffer: Vec<Letter> = vec![];

        for character in src.chars() {
            let mut letters = Letter::new(character);

            letter_buffer.append(&mut letters);
        }
        letter_buffer.push(Letter::Enter);

        dbg!(&letter_buffer);

        let song_size = (letter_buffer.len() / 8) + 1;
        let mut song = Self(Vec::with_capacity(song_size));

        while letter_buffer.len() > 8 {
            let buffer_vec: Vec<Letter> = letter_buffer.drain(..8).collect();

            let letter_arr: NormalArray = buffer_vec.try_into().unwrap();

            song.0.push(ParsedLetters(letter_arr));
        }

        let mut last_arr = [Letter::default(); 8];
        for (i, letter) in letter_buffer.iter().enumerate() {
            last_arr[i] = *letter;
        }
        song.0.push(ParsedLetters(last_arr));

        song
    }
}

#[cfg(test)]
mod tests {
    use crate::letters::{ParsedLetters, CompressedLetters, Letter};
    use crate::{CompressedArr, ParsedArr};

    #[test]
    fn decompression() {
        let encoded = CompressedLetters([0b01110011, 0b10010111, 0b00100100, 0b11011110, 0b00010011]);
        let decoded: ParsedLetters = ParsedLetters::from_compressed_words(&encoded);

        assert_eq!(
            decoded,
            ParsedLetters([
                Letter::B,
                Letter::O,
                Letter::O,
                Letter::B,
                Letter::I,
                Letter::E,
                Letter::NextIsWeird,
                Letter::S,
            ])
        )
    }
    #[test]
    fn compression() {
        let decoded = ParsedLetters([
            Letter::B,
            Letter::O,
            Letter::O,
            Letter::B,
            Letter::I,
            Letter::E,
            Letter::NextIsWeird,
            Letter::S,
        ]);
        let encoded: CompressedLetters = CompressedLetters::from_parsed_words(&decoded);

        assert_eq!(
            encoded,
            CompressedLetters([0b01110011, 0b10010111, 0b00100100, 0b11011110, 0b00010011,])
        );
    }
    #[test]
    fn compress_decompress() {
        let decoded = ParsedLetters([
            Letter::B,
            Letter::O,
            Letter::O,
            Letter::B,
            Letter::I,
            Letter::E,
            Letter::NextIsWeird,
            Letter::S,
        ]);
        let encoded: CompressedLetters = CompressedLetters::from_parsed_words(&decoded);

        let decoded_2: ParsedLetters = ParsedLetters::from_compressed_words(&encoded);

        assert_eq!(decoded, decoded_2);
    }

    #[test]
    fn decopressed_song_new() {
        let src = "Boobies Really do be SŠ M";

        let song = ParsedArr::new(src);

        assert_eq!(
            song,
            ParsedArr(vec![
                ParsedLetters([
                    Letter::B,
                    Letter::O,
                    Letter::O,
                    Letter::B,
                    Letter::I,
                    Letter::E,
                    Letter::S,
                    Letter::Space,
                ]),
                ParsedLetters([
                    Letter::R,
                    Letter::E,
                    Letter::A,
                    Letter::L,
                    Letter::L,
                    Letter::Y,
                    Letter::Space,
                    Letter::D,
                ]),
                ParsedLetters([
                    Letter::O,
                    Letter::Space,
                    Letter::B,
                    Letter::E,
                    Letter::Space,
                    Letter::S,
                    Letter::NextIsWeird,
                    Letter::S,
                ]),
                ParsedLetters([
                    Letter::Space,
                    Letter::M,
                    Letter::Enter,
                    Letter::Space,
                    Letter::Space,
                    Letter::Space,
                    Letter::Space,
                    Letter::Space,
                ]),
            ])
        )
    }

    #[test]
    fn everything() {
        let og = "i am very proud of my country and its citizens";
        let compressed = CompressedArr::new(og);

        assert_eq!(og.to_string(), compressed.to_string().trim_end())
    }
    #[test]
    fn actually_decrases_size() {
        let og = "i am very proud of my country and its citizens";
        let compressed = CompressedArr::new(og);

        use std::mem::size_of_val;
        dbg!(size_of_val(og));
        dbg!(size_of_val(&compressed));
        assert!(size_of_val(og) > size_of_val(&compressed))
    }
}
