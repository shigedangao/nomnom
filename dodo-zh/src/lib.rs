use crate::error::Error;
use cedict::{Dictionary, KeyVariant};
use pinyin::accent::PinyinAccent;
use pinyin::numbers::PinyinNumber;
use std::path::PathBuf;
use wade_giles::WadeGiles;
use zhuyin::Zhuyin;

pub mod cedict;
pub(crate) mod error;
pub(crate) mod pinyin;
pub(crate) mod wade_giles;
pub(crate) mod zhuyin;

/// Convert a word or a sentence of pinyin into zhuyin
///
/// # Arguments
///
/// * `text` - S
pub fn convert_pinyin_to_zhuyin<S>(text: S) -> Result<String, Error>
where
    S: AsRef<str> + Clone,
{
    let splitted_text = text.as_ref().split_whitespace().collect::<Vec<_>>();

    let zh = Zhuyin::new()?;

    let res = splitted_text
        .into_iter()
        .map(|content| zh.get_zhuyin_from_pinyin(content).into_owned())
        .collect::<Vec<_>>()
        .join(" ");

    Ok(res)
}

/// Convert a word or a sentence of pinyin into wade giles
///
/// # Arguments
///
/// * `text` - S
pub fn convert_pinyin_to_wade_giles<S>(text: S) -> Result<String, Error>
where
    S: AsRef<str> + Clone,
{
    let splitted_text = text.as_ref().split_whitespace().collect::<Vec<_>>();

    let res = splitted_text
        .into_iter()
        .map(|content| WadeGiles(content.to_string()).convert_pinyin_to_wade_giles())
        .collect::<Vec<_>>()
        .join(" ");

    Ok(res)
}

/// Convert a word or a sentence of pinyin with number to a pinyin tone mark
///
/// # Arguments
///
/// * `text` - S
pub fn convert_pinyin_tone_number_to_tone_mark<S>(text: S) -> Result<String, Error>
where
    S: AsRef<str> + Clone,
{
    let splitted_text = text.as_ref().split_whitespace().collect::<Vec<_>>();

    let res = splitted_text
        .into_iter()
        .filter_map(|content| {
            PinyinAccent(content.to_string()).replace_tone_numbers_with_tone_marks()
        })
        .collect::<Vec<_>>()
        .join(" ");

    Ok(res)
}

/// Convert a pinyin word or sentence with accent into a pinyin with number
///
/// # Arguments
///
/// * `text` - S
pub fn convert_pinyin_accent_to_pinyin_number<S>(text: S) -> Result<String, Error>
where
    S: AsRef<str> + Clone,
{
    let splitted_text = text.as_ref().split_whitespace().collect::<Vec<_>>();

    let res = splitted_text
        .into_iter()
        .map(|content| PinyinNumber(content.to_string()).into_number())
        .collect::<Vec<_>>()
        .join(" ");

    Ok(res)
}

/// Load Cedict Dictionary
///
/// # Arguments
///
/// * `p` - PathBuf
/// * `key_variant` - KeyVariant
pub fn load_cedict_dictionary(p: PathBuf, key_variant: KeyVariant) -> Result<Dictionary, Error> {
    let dictionary = Dictionary::new(p, key_variant)?;

    Ok(dictionary)
}
