//! Dodo-zh is a crate which provide utilities method on pinyin and cedict file. It enables you to do the following operations
//! - Load cedict file from a given path with the keys either being in Simplified or Traditional Chinese
//!
//! Doing several operations on a given pinyin such as:
//! - convert a pinyin to a zhuyin
//! - convert a pinyin to a wade giles
//! - convert a pinyin which has number tones e.g: wo3 to a pinyin with tone markers wǒ
//! - convert a pinyin with tones markers to numbers
//! - convert a simplified <-> traditional text
//! - detect chinese variant of a text
use crate::error::Error;
use cedict::Dictionary;
use pinyin::accent::PinyinAccent;
use pinyin::numbers::PinyinNumber;
use std::path::PathBuf;
use variant::KeyVariant;
use wade_giles::WadeGiles;
use zhuyin::Zhuyin;

pub mod cedict;
pub(crate) mod error;
pub(crate) mod pinyin;
pub mod variant;
pub(crate) mod wade_giles;
pub(crate) mod zhuyin;

// Constant
const SEPARATOR: &str = " ";

/// Convert a sequence of pinyin with tone markers into zhuyin
/// <div class="warning">Pinyin with tone number</div>
///
/// If you have a pinyin with numbers. You may first convert the pinyin to a tone markers with the [`self::convert_pinyin_tone_number_to_tone_mark`]
///
/// # Arguments
///
/// * `text` - S
///
/// # Examples
///
/// ```
/// let zhuyin = dodo_zh::convert_pinyin_to_zhuyin("wǒ").unwrap();
/// ```
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
        .join(SEPARATOR);

    Ok(res)
}

/// Convert a sequence of pinyin into wade giles
/// <div class="warning">Pinyin with tone number</div>
///
/// If you have a pinyin with numbers. You may first convert the pinyin to a tone markers with the [`self::convert_pinyin_tone_number_to_tone_mark`]
/// # Arguments
///
/// * `text` - S
///
/// # Examples
///
/// ```
/// let wade = dodo_zh::convert_pinyin_to_wade_giles("wǒ").unwrap();
/// ```
pub fn convert_pinyin_to_wade_giles<S>(text: S) -> Result<String, Error>
where
    S: AsRef<str> + Clone,
{
    let splitted_text = text.as_ref().split_whitespace().collect::<Vec<_>>();

    let res = splitted_text
        .into_iter()
        .map(|content| WadeGiles(content).convert_pinyin_to_wade_giles())
        .collect::<Vec<_>>()
        .join(SEPARATOR);

    Ok(res)
}

/// Convert a sequence of pinyin with number to a pinyin tone mark
///
/// # Arguments
///
/// * `text` - S
///
/// # Examples
///
/// ```
/// let pinyin_tone = dodo_zh::convert_pinyin_tone_number_to_tone_mark("wo3").unwrap();
/// ```
pub fn convert_pinyin_tone_number_to_tone_mark<S>(text: S) -> Result<String, Error>
where
    S: AsRef<str> + Clone,
{
    let splitted_text = text.as_ref().split_whitespace().collect::<Vec<_>>();

    let res = splitted_text
        .into_iter()
        .filter_map(|content| PinyinAccent(content).replace_tone_numbers_with_tone_marks())
        .collect::<Vec<_>>()
        .join(SEPARATOR);

    Ok(res)
}

/// Convert a sequence of pinyin with accent into a pinyin with number
///
/// # Arguments
///
/// * `text` - S
///
/// # Examples
///
/// ```
/// let pinyin_number = dodo_zh::convert_pinyin_accent_to_pinyin_number("wǒ").unwrap();
/// ```
pub fn convert_pinyin_accent_to_pinyin_number<S>(text: S) -> Result<String, Error>
where
    S: AsRef<str> + Clone,
{
    let splitted_text = text.as_ref().split_whitespace().collect::<Vec<_>>();

    let res = splitted_text
        .into_iter()
        .map(|content| PinyinNumber(content.to_string()).into_number())
        .collect::<Vec<_>>()
        .join(SEPARATOR);

    Ok(res)
}

/// Load Cedict Dictionary
///
/// # Arguments
///
/// * `p` - PathBuf
/// * `key_variant` - KeyVariant
///
/// # Examples
///
/// ```
/// use dodo_zh::variant::KeyVariant;
/// use std::path::PathBuf;
///
/// let dict = dodo_zh::load_cedict_dictionary(PathBuf::new(), KeyVariant::Traditional);
/// ```
pub fn load_cedict_dictionary(p: PathBuf, key_variant: KeyVariant) -> Result<Dictionary, Error> {
    let dictionary = Dictionary::new(&p, key_variant)?;

    Ok(dictionary)
}

/// Convert a chinese text to a desired variant (simplified <-> tradtional)
///
/// # Arguments
///
/// * `p` - PathBuf
/// * `content` - S
/// * `input_variant` - KeyVariant
/// * `target_varaint` - KeyVariant
///
/// # Examples
///
/// ```
/// use dodo_zh::variant::KeyVariant;
/// use std::path::PathBuf;
///
/// let converted = dodo_zh::convert_text_to_desired_variant(PathBuf::new(), "大家好我是馬克的摯友", KeyVariant::Traditional, KeyVariant::Simplified);
/// ```
pub fn convert_text_to_desired_variant<S: AsRef<str>>(
    p: PathBuf,
    content: S,
    input_variant: KeyVariant,
    target_variant: KeyVariant,
) -> Result<String, Error> {
    variant::initialize_dictionaries(&p)?;

    variant::KeyVariant::convert_text_to_desired_variant(content, input_variant, target_variant)
        .ok_or_else(|| Error::Parse("Unable to convert content to target key variant".to_string()))
}

/// Detect which variant of chinese is the text. If the given path for the cedict dictionary is passed
/// the detection will use the cedict. Otherwise it'll try to do the detection through unicode.
/// ⚠️ Unicode detection isn't very accurate. It's recommended to use the cedict dictionary for a precise detection.
///
/// # Arguments
///
/// * `p` - PathBuf
/// * `content` - S
///
/// # Examples
///
/// ```
/// use dodo_zh::variant::KeyVariant;
///
/// let variant = dodo_zh::detect_which_variant(None, "今天是星期天. 我明天不要去公司工作");
/// ```
pub fn detect_which_variant<S: AsRef<str>>(
    path: Option<PathBuf>,
    content: S,
) -> Result<KeyVariant, Error> {
    match path {
        Some(p) => {
            variant::initialize_dictionaries(&p)?;
            variant::KeyVariant::which_variant(content)
                .ok_or_else(|| Error::Parse("Unable to detect chinese variant".to_string()))
        }
        None => Ok(variant::KeyVariant::detect_variant_with_unicode(content)),
    }
}
