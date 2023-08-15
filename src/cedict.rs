use crate::error::Error;
use crate::hsk::HSKLevel;
use crate::log::Logger;
use serde::Serialize;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    sync::OnceLock,
};

// constant
const VALID_LINE_FILTER: [char; 2] = ['#', '%'];
const BRACKET: [char; 2] = ['[', ']'];
const SPACE_SEPARATOR: &str = " ";

// constant for vowels
const VOWEL: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const MEDIAL_VOWEL: [char; 2] = ['i', 'u'];

// constant for special cedict :<number> char
const TONE_SPECIAL_COLON_MARKER: &str = ":";
const NEUTRAL_TONE_U: &str = "ü";

// Special tone with the different 'u' value
static TONES_U: OnceLock<HashMap<&str, &str>> = OnceLock::new();
static TONES_ACCENT: OnceLock<HashMap<char, &str>> = OnceLock::new();

#[derive(Debug, Default, Serialize)]
pub struct Cedict {
    traditional_chinese: String,
    simplified_chinese: String,
    pinyin: String,
    pinyin_accent: String,
    translations: String,
    level: Option<HSKLevel>,
}

/// Parse and process each line of the cedict file
///
/// # Arguments
///
/// * `path` - &str
/// * `hsk` - HashMap<String, HSKLevel>
pub fn parse_cedict_file(path: &str, hsk: HashMap<String, HSKLevel>) -> Result<Vec<Cedict>, Error> {
    let file = File::open(path)?;
    let buffer = BufReader::new(file);
    let mut items = Vec::new();

    // Initialize the tones lock
    prepare_tones();

    Logger::info("🈷️ Processing Cedic file");

    for line in buffer.lines() {
        let unprocessed_line = line?;
        if !unprocessed_line.starts_with(VALID_LINE_FILTER) {
            let item = Cedict::parse(unprocessed_line, &hsk)?;
            items.push(item);
        }
    }

    Ok(items)
}

impl Cedict {
    /// Parse a line of the cedict into a Cedict struct
    ///
    /// # Arguments
    ///
    /// * `line` - String
    /// * `hsk` - &HashMap<String, HSKLevel>
    pub fn parse(content: String, hsk: &HashMap<String, HSKLevel>) -> Result<Self, Error> {
        let splitted_whitespace_res = content.split_whitespace().collect::<Vec<&str>>();
        let Some(tw_char) = splitted_whitespace_res.first() else {
            return Err(Error::Process("Unable to get the traditional chinese character".to_string()));
        };

        let Some(cn_char) = splitted_whitespace_res.get(1) else {
            return Err(Error::Process("Unable to get the simplified chinese character".to_string()));
        };

        let Some(pinyin) = splitted_whitespace_res.get(2)
            .map(|v| v.replace(BRACKET, "")) else {
                return Err(Error::Process("Unable to found the pinyin".to_string()));
            };

        let reminder = splitted_whitespace_res
            .get(3..)
            .unwrap_or_default()
            .join(SPACE_SEPARATOR);

        let mut item = Cedict {
            traditional_chinese: tw_char.to_string(),
            simplified_chinese: cn_char.to_string(),
            pinyin,
            translations: reminder,
            ..Default::default()
        };

        match hsk.get(&item.simplified_chinese) {
            Some(level) => item.level = Some(level.clone()),
            None => Logger::warn(format!(
                "{} has not been founded in the hsk dictionnary",
                item.simplified_chinese
            )),
        }

        // convert a pinyin w/o accent to a pinyin with accent
        item.convert_pinyin_to_acccent()?;

        Ok(item)
    }

    /// Convert a pinyin with numeric value to a pinyin with accents
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Self
    fn convert_pinyin_to_acccent(&mut self) -> Result<(), Error> {
        // get a list of pinyin (cedict can provide many pinyin for a single character)
        let words = self.pinyin.split_whitespace();
        let mut pinyin_list_accent = Vec::new();
        for word in words {
            // loop through each character of the pinyin word to find if it has any numeric value
            // if yes, then we need to find the vowel and replace the vowel with the proper tone character
            let has_numeric = word.chars().any(char::is_numeric);
            if has_numeric {
                // Check if it's a special case of ':<number>'
                let processed_word = match word.contains(TONE_SPECIAL_COLON_MARKER) {
                    true => replace_collon_tone_with_accent(word)?,
                    false => replace_vowel_with_accent(word)?,
                };

                pinyin_list_accent.push(processed_word);
            } else {
                pinyin_list_accent.push(word.to_string());
            }
        }

        self.pinyin_accent = pinyin_list_accent.join(SPACE_SEPARATOR);

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
    let numeric = chars_vec.pop().ok_or(Error::Numeral)?;

    // count number of vowel in a sentence
    let vowel_count = chars_vec.iter().filter(|c| VOWEL.contains(c)).count();
    let Some(tones) = TONES_ACCENT.get() else {
        return Err(Error::Process("Unable to retrieve the tones accent".to_string()))
    };

    let tone = tones.get(&numeric).unwrap_or(&"");

    // get the position of the vowel we want to edit
    let vowel_position = get_vowel_position(vowel_count, &chars_vec)?;
    // loop through the chars_vec to edit the char and then create a string
    for (idx, ch) in chars_vec.into_iter().enumerate() {
        if idx == vowel_position {
            pinyin_vec.push(format!("{ch}{tone}"));
        } else {
            pinyin_vec.push(ch.to_string());
        }
    }

    Ok(pinyin_vec.join(""))
}

/// Special case where a pinyin has the following tone marker
///     - :1 -> ǖ
///     - :2 -> ǘ
///     - :3 -> ǚ
///     - :4 -> ǜ
///
/// # Arguments
///
/// * `word` - &str
fn replace_collon_tone_with_accent(word: &str) -> Result<String, Error> {
    // Change the two last character in order to get the tone marker and the value
    let (part, tone_marker) = word.split_at(word.len() - 3);

    let Some(tones) = TONES_U.get() else {
        return Err(Error::Process("Unable to get the tones".to_string()))
    };

    let char_with_tone_marker = *tones.get(tone_marker).unwrap_or(&NEUTRAL_TONE_U);

    Ok(format!("{part}{char_with_tone_marker}"))
}

/// Get the vowel position which will be use to add the tone marker
///
/// # Arguments
///
/// * `vowel_count` - usize
/// * `mut vowels` - &[char]
fn get_vowel_position(vowel_count: usize, vowels: &[char]) -> Result<usize, Error> {
    match vowel_count {
        1 => match vowels.iter().position(|c| VOWEL.contains(c)) {
            Some(vp) => Ok(vp),
            None => Err(Error::Process("Vowel does not has one vowel".to_string())),
        },
        _ => {
            for (idx, c) in vowels.iter().enumerate() {
                // cases where there are more than 1 vowel
                // If the first vowel is a MEDIAL Vowel, then the next vowel (should be the next letter)
                // is the one who has the marker tone
                if MEDIAL_VOWEL.contains(c) {
                    return Ok(idx + 1);
                } else if VOWEL.contains(c) {
                    // otherwise it's the first vowel that we need to take into account
                    // only the first vowel
                    return Ok(idx);
                }
            }

            Ok(0)
        }
    }
}

// Prepare the tones Hashmap
// - special cases for u value
// - special case for pinyin tones which use numerical values
fn prepare_tones() {
    TONES_U.get_or_init(|| {
        let mut tones = HashMap::new();
        tones.insert("u:1", "ǖ");
        tones.insert("u:2", "ǘ");
        tones.insert("u:3", "ǚ");
        tones.insert("u:4", "ǜ");

        tones
    });

    TONES_ACCENT.get_or_init(|| {
        let mut tones = HashMap::new();
        tones.insert('1', "\u{0304}");
        tones.insert('2', "\u{0301}");
        tones.insert('3', "\u{030c}");
        tones.insert('4', "\u{0300}");

        tones
    });
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn expect_to_parse_cedict_line() {
        let line = r"一動不動 一动不动 [yi1 dong4 bu4 dong4] /motionless/";
        let mm = HashMap::new();

        // Initialize the tones
        super::prepare_tones();

        let cedict = super::Cedict::parse(line.to_string(), &mm);
        assert!(cedict.is_ok());

        let res = cedict.unwrap();

        assert_eq!(res.traditional_chinese, "一動不動");
        assert_eq!(res.simplified_chinese, "一动不动");
        assert_eq!(res.pinyin, "yi1");
        assert_eq!(res.pinyin_accent, "yi\u{304}");
    }

    #[test]
    fn expect_to_convert_numeri_pinyin_to_accent() {
        let word = "xian1";

        super::prepare_tones();
        let expected_word = super::replace_vowel_with_accent(word).unwrap();
        assert_eq!(expected_word, "xiān");
    }

    #[test]
    fn expect_to_convert_simple_pinyin_to_accent() {
        let word = "chi1";

        super::prepare_tones();
        let expected_word = super::replace_vowel_with_accent(word).unwrap();
        assert_eq!(expected_word, "chī");
    }

    #[test]
    fn expect_to_convert_special_tone_marker() {
        let word = "nu:3";

        super::prepare_tones();
        let expected_word = super::replace_collon_tone_with_accent(word);
        assert_eq!(expected_word.unwrap(), "nǚ");
    }
}
