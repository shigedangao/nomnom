use crate::error::Error;
use crate::hsk::HSKLevel;
use crate::log::Logger;
use crate::pinyin;
use crate::wade_giles;
use crate::zhuyin;
use crate::zhuyin::initials;
use serde::Serialize;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

// constant
const VALID_LINE_FILTER: [char; 2] = ['#', '%'];
const BRACKET: [char; 2] = ['[', ']'];
const SPACE_SEPARATOR: &str = " ";

// constant for special cedict :<number> char
const TONE_SPECIAL_COLON_MARKER: &str = ":";

#[derive(Debug, Default, Serialize)]
pub struct Cedict {
    traditional_chinese: String,
    simplified_chinese: String,
    pinyin: String,
    pinyin_accent: String,
    translations: String,
    zhuyin: String,
    wade: String,
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

    // Load the zhuyin files
    Logger::info("🈶 Load zhuyin character files");
    let zh = zhuyin::initials::load_zhuyin_accents_files()?;

    Logger::info("🈷️ Processing Cedic file");

    for line in buffer.lines() {
        let unprocessed_line = line?;
        if !unprocessed_line.starts_with(VALID_LINE_FILTER) {
            let item = Cedict::parse(unprocessed_line, &hsk, &zh)?;
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
    pub fn parse(
        content: String,
        hsk: &HashMap<String, HSKLevel>,
        zh: &initials::ZhuyinConfig,
    ) -> Result<Self, Error> {
        let splitted_whitespace_res = content.split_whitespace().collect::<Vec<&str>>();
        let Some(tw_char) = splitted_whitespace_res.first() else {
            return Err(Error::Process(
                "Unable to get the traditional chinese character".to_string(),
            ));
        };

        let Some(cn_char) = splitted_whitespace_res.get(1) else {
            return Err(Error::Process(
                "Unable to get the simplified chinese character".to_string(),
            ));
        };

        // assuming that the pinyin start with the brackets..
        // join the reminder
        let rest = splitted_whitespace_res
            .get(2..)
            .unwrap_or_default()
            .join(SPACE_SEPARATOR);

        let remainder = rest.split(BRACKET).collect::<Vec<&str>>();

        let Some(pinyin) = remainder.get(1) else {
            return Err(Error::Process("Unable to found the pinyin".to_string()));
        };

        let translations = remainder.get(2..).unwrap_or_default().join(SPACE_SEPARATOR);

        let mut item = Cedict {
            traditional_chinese: tw_char.to_string(),
            simplified_chinese: cn_char.to_string(),
            pinyin: pinyin.to_string(),
            translations: translations.trim().to_string(),
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
        // convert the pinyin to zhuyin
        item.convert_pinyin_to_zhuyin(zh)?;
        // convert a regular pinyin to a wade giles
        item.convert_pinyin_to_wades()?;

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
                    true => pinyin::replace_u_tone_with_accent(word)?,
                    false => pinyin::replace_vowel_with_accent(word)?,
                };

                pinyin_list_accent.push(processed_word);
            } else {
                pinyin_list_accent.push(word.to_string());
            }
        }

        self.pinyin_accent = pinyin_list_accent.join(SPACE_SEPARATOR);

        Ok(())
    }

    /// Convert the generated piyin with accent into a bopomofo representation
    ///
    /// # Arguments
    ///
    /// * `&mut Self`
    fn convert_pinyin_to_zhuyin(&mut self, zh: &initials::ZhuyinConfig) -> Result<(), Error> {
        let pinyin_accents_vec = self
            .pinyin_accent
            .split(SPACE_SEPARATOR)
            .collect::<Vec<_>>();

        let mut bopomofo = Vec::new();

        for p in pinyin_accents_vec {
            let res = zhuyin::get_zhuyin_from_pinyin(p, zh)?;
            bopomofo.push(res);
        }

        let bopomofo = bopomofo.join(SPACE_SEPARATOR);
        self.zhuyin = bopomofo.trim().to_string();

        Ok(())
    }

    /// Converting a pinyin to a wade value
    ///
    /// # Arguments
    ///
    /// * `&mut self`
    fn convert_pinyin_to_wades(&mut self) -> Result<(), Error> {
        let wade = wade_giles::convert_cedict_pinyin_sentence_to_wade(&self.pinyin)?;
        self.wade = wade;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::hsk::HSKLevel;
    use std::collections::HashMap;

    #[test]
    fn expect_to_parse_cedict_line() {
        let line = r"一動不動 一动不动 [yi1 dong4 bu4 dong4] /motionless/";
        let mut mm = HashMap::new();
        mm.insert("一动不动".to_string(), HSKLevel::HSK1);

        let zh = super::zhuyin::initials::load_zhuyin_accents_files().unwrap();

        let cedict = super::Cedict::parse(line.to_string(), &mm, &zh);
        assert!(cedict.is_ok());

        let res = cedict.unwrap();

        assert_eq!(res.traditional_chinese, "一動不動");
        assert_eq!(res.simplified_chinese, "一动不动");
        assert_eq!(res.pinyin, "yi1 dong4 bu4 dong4");
        assert_eq!(
            res.pinyin_accent,
            "yi\u{304} do\u{300}ng bu\u{300} do\u{300}ng"
        );
        assert_eq!(res.translations, "/motionless/");
        assert_eq!(res.zhuyin, "ㄧ ㄉㄨㄥ\u{300} ㄅㄨ\u{300} ㄉㄨㄥ\u{300}");
        assert_eq!(res.wade, "i1 tung4 pu4 tung4");
    }
}
