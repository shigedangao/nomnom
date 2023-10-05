use crate::error::Error;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

// Constant
const ACCENTS: [&str; 3] = ["\u{0301}", "\u{030c}", "\u{0300}"];

pub type ZhuyinConfig = (
    HashMap<String, String>,
    HashMap<String, String>,
    HashSet<String>,
);

#[derive(Debug, Deserialize)]
struct Zhuyin {
    initials: Vec<Data>,
    finals: Vec<Data>,
}

#[derive(Debug, Deserialize)]
struct Data {
    sound: String,
    value: String,
}

// Provided by
// @link https://www.omniglot.com/chinese/zhuyin.htm
/// Load zhuyin accents and return the related initials, finals and accents needed to build the zhuyin from pinyin
pub fn load_zhuyin_accents_files() -> Result<ZhuyinConfig, Error> {
    let data = include_bytes!("../static/zhuyin.json");
    let parsed_zhuyin: Zhuyin = serde_json::from_slice(data)?;

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

    Ok((initials, finals, accents))
}
