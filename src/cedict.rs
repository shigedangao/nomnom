use std::{
    fs::File,
    io::BufReader
};
use serde::Serialize;
use crate::error::Error;
use std::io::BufRead;

// constant
const NB_SIGN_CHARACTER_CEDICT: char = '#';
const PERCENT_CHARACTER_CEDICT: char = '%';
const EMPTY_SPACE_CHARACTER: char = ' ';
const LEFT_BRACKET_CHARACTER: char = '[';
const RIGHT_BRACKET_CHARACTER: char = ']';

// const for chinese pinyin accent
const FIRST_TONE: &str = "\u{0304}";
const SECOND_TONE: &str = "\u{0301}";
const THIRD_TONE: &str = "\u{030c}";
const FOURTH_TONE: &str = "\u{0300}";

// constant for vowels
const VOWEL: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const MEDIAL_VOWEL: [char; 2] = ['i', 'u'];

#[derive(Debug, Default, Serialize)]
pub struct Cedict {
    traditional_chinese: String,
    simplified_chinese: String,
    pinyin: String,
    pinyin_accent: String,
    translations: String
}

impl Cedict {
    /// Parse the cc-cedict into a list of Cedict item
    /// 
    /// # Arguments
    /// 
    /// * `cedict_path` - &str
    pub fn parse(cedict_path: &str) -> Result<Vec<Cedict>, Error> {
        let file = File::open(cedict_path)?;
        // read the cedict line by line
        let buffer = BufReader::new(file);
        let mut items = Vec::new();

        for line in buffer.lines() {
            if let Some(content) = Self::skip_line(line) {
                let mut reminder = "";
                let mut item = Cedict::default();

                if let Some((tw_character, rest)) = content.split_once(EMPTY_SPACE_CHARACTER) {
                    item.traditional_chinese = tw_character.to_owned();
                    reminder = rest;
                }

                if let Some((sf_character, rest)) = reminder.split_once(EMPTY_SPACE_CHARACTER) {
                    item.simplified_chinese = sf_character.to_owned();
                    reminder = rest;
                }

                if let Some((pinyin, rest)) = reminder.split_once(RIGHT_BRACKET_CHARACTER) {
                    item.pinyin = pinyin.to_owned().replace(LEFT_BRACKET_CHARACTER, "");
                    reminder = rest.trim();
                }

                item.translations = reminder.to_string();
                // convert a pinyin w/o accent to a pinyin with accent
                item.convert_pinyin_to_acccent()?;
                items.push(item);
            }
        }

        Ok(items)
    }

    /// Check if a line contains character which we want to avoid. If we match those. Then we skip these lines
    /// 
    /// # Arguments
    /// 
    /// * `line` - Result<String, std::io::Error>
    fn skip_line(line: Result<String, std::io::Error>) -> Option<String> {
        if let Ok(content) = line {
            if content.starts_with(NB_SIGN_CHARACTER_CEDICT) || content.starts_with(PERCENT_CHARACTER_CEDICT) {
                return None;
            }

            return Some(content);
        }

        None
    }

    /// Convert a pinyin with numeric value to a pinyin with accents
    /// 
    /// # Arguments
    /// 
    /// * `&mut self` - Self
    fn convert_pinyin_to_acccent(&mut self) -> Result<(), Error> {
        // get a list of pinyin (cedict can provide many pinyin for a single character)
        let words = self.pinyin.split(' ');
        let mut pinyin_list_accent = Vec::new();
        for word in words {
            // loop through each character of the pinyin word to find if it has any numeric value
            // if yes, then we need to find the vowel and replace the vowel with the proper tone character
            let has_numeric = word.chars().any(char::is_numeric);
            if has_numeric {
                let word = replace_vowel_with_accent(word)?;
                pinyin_list_accent.push(word);
            } else {
                pinyin_list_accent.push(word.to_string());
            }
        }

        self.pinyin_accent = pinyin_list_accent.join(" ");

        Ok(())
    }
}

/// Replace the vowel with the tone accent
///     - Tones are only placed in a vowel
///     - There are some rules. Please refer to readme for the link of the rules but in short:
///         - 1 vowel only -> add the marker tone to the vowel
///         - More than 2 vowels
///             -> If first vowel is a medial vowel then the next letter (vowel) should have the tone marker
///             -> Otherwise, the first vowel has the tone marker
/// 
/// # Arguments
/// 
/// * `word` - &str 
fn replace_vowel_with_accent(word: &str) -> Result<String, Error> {
    let mut chars_vec: Vec<char> = word.chars().collect();
    let mut pinyin_vec = Vec::new();

    // indication of the tone is located at the end of the word
    let numeric = chars_vec
        .pop()
        .ok_or(Error::Numeral)?;

    // count number of vowel in a sentence
    let vowel_count = chars_vec.iter().filter(|c| VOWEL.contains(c)).count();
    let tone = match numeric {
        '1' => FIRST_TONE,
        '2' => SECOND_TONE,
        '3' => THIRD_TONE,
        '4' => FOURTH_TONE,
        _ => ""
    };

    // get the position of the vowel we want to edit
    let vowel_position = get_vowel_position(vowel_count, &chars_vec);
    // loop through the chars_vec to edit the char and then create a string
    for (idx, ch) in chars_vec.into_iter().enumerate() {
        if idx == vowel_position {
            pinyin_vec.push(format!("{}{}", ch, tone));
        } else {
            pinyin_vec.push(ch.to_string());
        }
    }

    Ok(pinyin_vec.join(""))
}

/// Get the vowel position which will be use to add the tone marker
/// 
/// # Arguments
/// 
/// * `vowel_count` - usize
/// * `mut vowels` - Chars
fn get_vowel_position(vowel_count: usize, vowels: &[char]) -> usize {
    let mut vowel_position = 0;

    if vowel_count == 1 {
        // Using expect as we should have at least 1 as we have previously count that we have 1 vowel
        vowel_position = vowels.iter()
            .position(|c| VOWEL.contains(c))
            .expect("Expect to found a vowel position");
    } else {
        for (idx, c) in vowels.iter().enumerate() {
            // cases where there are more than 1 vowel
            // If the first vowel is a MEDIAL Vowel, then the next vowel (should be the next letter)
            // is the one who has the marker tone
            if MEDIAL_VOWEL.contains(c) {
                vowel_position = idx + 1;
                // break to not take into account other vowels
                break;
            } else if VOWEL.contains(c) {
                // otherwise it's the first vowel that we need to take into account
                // only the first vowel
                vowel_position = idx;
                // break to not take into account other vowels
                break;
            }
        }
    }

    vowel_position
}

#[cfg(test)]
mod tests {

    #[test]
    fn expect_to_convert_numeri_pinyin_to_accent() {
        let word = "xian1";

        let expected_word = super::replace_vowel_with_accent(word).unwrap();
        assert_eq!(expected_word, "xia??n");
    }

    #[test]
    fn expect_to_convert_simple_pinyin_to_accent() {
        let word = "chi1";

        let expected_word = super::replace_vowel_with_accent(word).unwrap();
        assert_eq!(expected_word, "chi??");
    }
}
