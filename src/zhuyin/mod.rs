use std::collections::HashSet;
use unicode_normalization::UnicodeNormalization;
use crate::error::Error;
use self::initials::{INITIALS, FINALS, ACCENTS};

pub(crate) mod initials;

/// Compute the zhuyin from the given pinyin character value
/// 
/// # Arguments
/// 
/// * `pinyin` - &str
pub fn get_zhuyin_from_pinyin(pinyin: &str) -> Result<String, Error> {
    let mut normalized = pinyin
        .nfd()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();

    let (
        i, f, a
    ) = (INITIALS.get(), FINALS.get() , ACCENTS.get());

    let (
        Some(initials),
        Some(finals),
        Some(accents),
    ) = (i, f, a) else {
        return Err(Error::Process("Unable to retrieve zhuyin parameters".to_string()));
    };

    swap_tones_to_end(&mut normalized, &accents);

    let mut zhuyin_acc = Vec::new();
    let mut buf = String::new();

    let mut iter = normalized.iter().enumerate();
    while let Some((idx, item)) = iter.next() {
        if item.trim().is_empty() {
            continue;
        }

        buf.push_str(&item);
        let buf_str = buf.as_str();

        // Handle special cases for the initials.
        match buf_str {
            // Match whether the pinyin would begin by "zh, ch, sh"
            "z" | "c" | "s" => {
                if let Some(next) = normalized.get(idx + 1) {
                    if next == "h" {
                        continue;
                    }
                }
            },
            // Used to match the "zhi, chi, shi" bopomofo sound
            "zh" | "ch" | "sh" => {
                if let Some(next) = normalized.get(idx + 1) {
                    if next == "i" {
                        continue;
                    }
                }
            }
            _ => {}
        }

        // Otherwise match the initials
        if let Some(zhuyin) = initials.get(buf.as_str()) {
            zhuyin_acc.push(zhuyin);
            buf.clear();
            continue;
        }

        // Used to handle special cases for the finals
        match buf.as_str() {
            // Check whether "a, e, ya" can be match with the sound "ai, ao, yao...". Cases below
            // also handle other specific bopomofo rules.
            "a" | "e" | "ya" => {
                if let Some(next) = normalized.get(idx + 1) {
                    if next == "i" || (next == "o" && (buf.as_str() == "a" || buf.as_str() == "ya")) || next == "n" {
                        continue;
                    }
                }
            },
            "o" => {
                if let Some(next) = normalized.get(idx + 1) {
                    if next == "u" || next == "n" {
                        continue;
                    }
                }
            },
            "an" | "en" | "on" | "yan" | "ian" | "yin" | "in" | "wan" | "wen" | "uan" => {
                if let Some(next) = normalized.get(idx + 1) {
                    if next == "g" {
                        continue;
                    }
                }
            },
            "u" => {
                if let Some(next) = normalized.get(idx + 1) {
                    if next == "a" || next == "o" || next == "i" || next == "n" {
                        continue;
                    }
                }
            },
            "w" => {
                if let Some(next) = normalized.get(idx + 1) {
                    if next == "u" || next == "a" || next == "o" || next == "e" {
                        continue;
                    }
                }
            }
            _ => {}
        }

        if let Some(zhuyin) = finals.get(buf.as_str()).or(accents.get(buf.as_str())) {
            zhuyin_acc.push(zhuyin);
            buf.clear();
        }  
    }

    let zhuyin: String = zhuyin_acc.into_iter().map(|c| c.to_string()).collect();

    Ok(zhuyin)
}

/// Swap the founded the tone at the end of the word. This is because zhuyin put the tones
/// at the end.
/// 
/// # Arguments
/// 
/// * `words` - &mut Vec<String>
/// * `accents` - &HashSet<&str>
fn swap_tones_to_end(words: &mut Vec<String>, accents: &HashSet<&str>) {
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
    
    #[test]
    fn expect_to_generate_zhuyin() {
        super::initials::initialize_initials_tables();

        let pinyin_wo_accent = "néng";

        let zhuyin = super::get_zhuyin_from_pinyin(pinyin_wo_accent).unwrap();
        assert_eq!(zhuyin, "ㄋㄥ\u{301}");
    }

    #[test]
    fn expect_to_generate_zhuyin_two() {
        super::initials::initialize_initials_tables();

        let pinyin_wo_accent = "wǒ";

        let zhuyin = super::get_zhuyin_from_pinyin(pinyin_wo_accent).unwrap();
        assert_eq!(zhuyin, "ㄨㄛ\u{30c}");
    }
}
