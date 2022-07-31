use std::{
    fs::File,
    io::BufReader,
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

// chinese pinyin accent
const FIRST_TONE: &str = "\u{0304}";
const SECOND_TONE: &str = "\u{0301}";
const THIRD_TONE: &str = "\u{030c}";
const FOURTH_TONE: &str = "\u{0300}";

const VOWEL: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const MEDAL_VOWEL: [char; 2] = ['i', 'u'];

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
                item.convert_pinyin_to_acccent();
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

    fn convert_pinyin_to_acccent(&mut self) {
        // get a list of pinyin (cedict can provide many pinyin for a single character)
        let words = self.pinyin.split(" ");
        let mut pinyin_list_accent = Vec::new();
        for word in words {
            // loop through each character of the pinyin word to find if it has any numeric value
            // if yes, then we need to find the vowel and replace the vowel with the proper tone character
            let has_numeric = word.chars().all(char::is_numeric);
            if has_numeric {
                replace_vowel_with_accent(word);
            } else {
                pinyin_list_accent.push(word);
            }
        }

        self.pinyin_accent = pinyin_list_accent.join("");
    }
}

fn replace_vowel_with_accent(word: &str) -> String {
    let chars = word.chars();
    let mut chars_vec: Vec<char> = chars.to_owned().collect();
    // indication of the tone is located at the end of the word
    let numeric = chars_vec.pop()
        .and_then(|c| Some(c as u8));

    if numeric.is_none() {
        return chars.as_str().to_owned();
    }

    let tone = match numeric.unwrap() {
        1 => FIRST_TONE,
        2 => SECOND_TONE,
        3 => THIRD_TONE,
        4 => FOURTH_TONE,
        _ => ""
    };

    // store the position where we found the vowel
    let mut vowel_position = 0;
    let mut previous_vowel = '\0';
    for (idx, c) in chars.enumerate() {
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            // check if the vowel is a medal vowel
            previous_vowel = c;
            vowel_position = idx;
        }
    }

    if let Some(vowel) = chars_vec.get(vowel_position) {
        
    }

    return "".to_string()
}
