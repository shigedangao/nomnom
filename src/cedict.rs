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

#[derive(Debug, Default, Serialize)]
pub struct Cedict {
    traditional_chinese: String,
    simplified_chinese: String,
    pinyin: String,
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
}
