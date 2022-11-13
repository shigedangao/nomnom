use std::collections::HashMap;
use serde::Serialize;
use crate::error::Error;

const CHINESE_LEFT_PEN: &str = "（";
const CHINESE_RIGHT_PEN: &str = "）";

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub enum HSKLevel {
    #[default]
    HSK1,
    HSK2,
    HSK3,
    HSK4,
    HSK5,
    HSK6
}

impl HSKLevel {
    /// Get the HSKLevel based on the given string
    /// 
    /// # Arguments
    /// 
    /// * `level` - &str
    fn from_string(level: &str) -> Option<HSKLevel> {
        // clean the level string of parentheses
        let cleaned_level = level
            .replace(CHINESE_LEFT_PEN, "")
            .replace(CHINESE_RIGHT_PEN, "");

        match cleaned_level.trim() {
            "一级" => Some(HSKLevel::HSK1),
            "二级" => Some(HSKLevel::HSK2),
            "三级" => Some(HSKLevel::HSK3),
            "四级" => Some(HSKLevel::HSK4),
            "五级" => Some(HSKLevel::HSK5),
            "六级" => Some(HSKLevel::HSK6),
            &_ => None
        }
    }
}

/// Get a list of chinese character and it's associated HSK level based on the HSK-2012.csv
/// 
/// # Arguments
/// 
/// * `path` - &str
pub fn from_csv(path: &str) -> Result<HashMap<String, Option<HSKLevel>>, Error> {
    let file = std::fs::read_to_string(path)?;
    let mut reader = csv::Reader::from_reader(file.as_bytes());
    let mut hsk = HashMap::new();

    for res in reader.records() {
        let record = res?;
        if let Some(content) = record.get(0) {
            // split the content in two by space
            let splitted: Vec<&str> = content.split(CHINESE_LEFT_PEN).collect();
            // The hsk.csv is formatted as follow
            // "<char> (level)"
            // "愛 （一级）"
            let zipped = splitted
                .first()
                .zip(splitted.get(1));

            if let Some((character, level)) = zipped {
                hsk.insert(character.to_string(), HSKLevel::from_string(level));
            }
        }
    }

    Ok(hsk)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_get_hsk_level() {
        let vec = from_csv("./HSK-2012.csv");

        assert!(vec.is_ok());

        let hsk = vec.unwrap();

        let hsk1 = hsk.get("水果").unwrap();
        assert_eq!(hsk1.as_ref().unwrap(), &HSKLevel::HSK1);

        let hsk6 = hsk.get("包装").unwrap();
        assert_eq!(hsk6.as_ref().unwrap(), &HSKLevel::HSK6);
    }
}
