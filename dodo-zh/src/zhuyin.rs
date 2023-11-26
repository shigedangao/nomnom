use crate::error::Error;
use serde::Deserialize;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use unicode_normalization::UnicodeNormalization;

// Constant
const ACCENTS: [&str; 3] = ["\u{0301}", "\u{030c}", "\u{0300}"];

#[derive(Debug)]
pub struct Zhuyin {
    initials: HashMap<String, String>,
    finals: HashMap<String, String>,
    accents: HashSet<String>,
}

#[derive(Debug, Deserialize)]
struct ZhuyinDataWrapper {
    initials: Vec<Data>,
    finals: Vec<Data>,
}

#[derive(Debug, Deserialize)]
struct Data {
    sound: String,
    value: String,
}

impl Zhuyin {
    // Provided by
    // @link https://www.omniglot.com/chinese/zhuyin.htm
    /// Load zhuyin accents and return the related initials, finals and accents needed to build the zhuyin from pinyin
    pub fn new() -> Result<Self, Error> {
        let data = include_bytes!("./static/zhuyin.json");
        let parsed_zhuyin: ZhuyinDataWrapper = serde_json::from_slice(data)?;

        // Create the hashmap for the targeted type
        let initials = parsed_zhuyin
            .initials
            .into_iter()
            .map(|d| (d.sound, d.value))
            .collect::<HashMap<_, _>>();

        let finals: HashMap<String, String> = parsed_zhuyin
            .finals
            .into_iter()
            .map(|d| (d.sound, d.value))
            .collect::<HashMap<_, _>>();

        let accents: HashSet<String> = ACCENTS.into_iter().map(|a| a.to_string()).collect();

        Ok(Zhuyin {
            initials,
            finals,
            accents,
        })
    }

    /// Get a zhuyin character from a standard pinyin
    ///
    /// # Arguments
    ///
    /// * `&self` - Self
    /// * `pinyin` - S
    pub fn get_zhuyin_from_pinyin<S>(&self, pinyin: S) -> Cow<str>
    where
        S: AsRef<str>,
    {
        let mut normalized = pinyin
            .as_ref()
            .nfd()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();

        swap_tones_to_end(&mut normalized, &self.accents);

        let mut zhuyin_acc = Vec::new();
        let mut buf = String::new();

        let iter = normalized.iter().enumerate();
        for (idx, item) in iter {
            if item.trim().is_empty() {
                continue;
            }

            buf.push_str(item);
            let buf_str = buf.as_str();

            let next = match normalized.get(idx + 1) {
                Some(v) => v.clone(),
                None => String::new(),
            };

            // Handle special cases for the initials.
            match buf_str {
                // Match whether the pinyin would begin by "zh, ch, sh"
                "z" | "c" | "s" => {
                    if next == "h" {
                        continue;
                    }
                }
                // Used to match the "zhi, chi, shi" bopomofo sound
                "zh" | "ch" | "sh" => {
                    if next == "i" {
                        continue;
                    }
                }
                _ => {}
            }

            // Otherwise match the initials
            if let Some(zhuyin) = self.initials.get(buf.as_str()) {
                zhuyin_acc.push(zhuyin);
                buf.clear();
                continue;
            }

            // Used to handle special cases for the finals
            match buf_str {
                // Check whether "a, e, ya" can be match with the sound "ai, ao, yao...". Cases below
                // also handle other specific bopomofo rules.
                "a" | "e" | "ya" => {
                    if next == "i"
                        || (next == "o" && (buf_str == "a" || buf_str == "ya"))
                        || next == "n"
                        || (next == "r" && buf_str == "e")
                    {
                        continue;
                    }
                }
                "o" => {
                    if next == "u" || next == "n" {
                        continue;
                    }
                }
                "an" | "en" | "on" | "yan" | "ian" | "yin" | "in" | "wan" | "wen" | "uan" => {
                    if next == "g" {
                        continue;
                    }
                }
                "u" => {
                    if next == "a" || next == "o" || next == "i" || next == "n" {
                        continue;
                    }
                }
                "w" => {
                    if next == "u" || next == "a" || next == "o" || next == "e" {
                        continue;
                    }
                }
                _ => {}
            }

            if let Some(zhuyin) = self
                .finals
                .get(buf.as_str())
                .or(self.accents.get(buf.as_str()))
            {
                zhuyin_acc.push(zhuyin);
                buf.clear();
            }
        }

        let zhuyin: String = zhuyin_acc.into_iter().map(|c| c.to_string()).collect();

        Cow::from(zhuyin)
    }
}

/// Swap the founded the tone at the end of the word. This is because zhuyin put the tones
/// at the end.
///
/// # Arguments
///
/// * `words` - &mut Vec<String>
/// * `accents` - &HashSet<&str>
fn swap_tones_to_end(words: &mut Vec<String>, accents: &HashSet<String>) {
    for (idx, c) in words.to_owned().iter().enumerate() {
        if accents.contains(c.as_str()) {
            let accent = words.remove(idx);
            words.push(accent);

            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_generate_zhuyin() {
        let pinyin_wo_accent = "néng";

        let zhuyin = Zhuyin::new().unwrap();
        let zhuyin = zhuyin.get_zhuyin_from_pinyin(pinyin_wo_accent);
        assert_eq!(zhuyin, "ㄋㄥ\u{301}");
    }

    #[test]
    fn expect_to_generate_zhuyin_two() {
        let pinyin_wo_accent = "wǒ";

        let zhuyin = Zhuyin::new().unwrap();
        let zhuyin = zhuyin.get_zhuyin_from_pinyin(pinyin_wo_accent);
        assert_eq!(zhuyin, "ㄨㄛ\u{30c}");
    }

    #[test]
    fn expect_to_handle_er() {
        let pinyin = "ér";

        let zhuyin = Zhuyin::new().unwrap();
        let zhuyin = zhuyin.get_zhuyin_from_pinyin(pinyin);
        assert_eq!(zhuyin, "ㄦ\u{301}");
    }

    #[test]
    fn expect_to_generate_pinyin_to_zhuyin_for_sentences() {
        let pinyin = "dà jiā hǎo xiàn zài wǒ hǎo léi";
        let pinyins: Vec<&str> = pinyin.split_whitespace().collect();

        let handle = Zhuyin::new().unwrap();

        let mut res = Vec::new();
        for pin in pinyins {
            let zhuyin = handle.get_zhuyin_from_pinyin(pin);
            res.push(zhuyin);
        }

        let zhuyin_res = res.join("  ");
        assert_eq!(
            zhuyin_res,
            "ㄉㄚ̀  ㄐㄧㄚ  ㄏㄠ̌  ㄒㄧㄢ̀  ㄗㄞ̀  ㄨㄛ̌  ㄏㄠ̌  ㄌㄟ́"
        );
    }
}
