use crate::error::Error;
use crate::log::Logger;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum HSKLevel {
    #[default]
    HSK1,
    HSK2,
    HSK3,
    HSK4,
    HSK5,
    HSK6,
    HSK7_9,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HskData {
    level: String,
    #[serde(rename(deserialize = "character"))]
    s_character: String,
}

impl HSKLevel {
    /// Get the HSKLevel based on the given string
    ///
    /// # Arguments
    ///
    /// * `level` - S
    fn from_string<S>(level: S) -> HSKLevel
    where
        S: AsRef<str>,
    {
        match level.as_ref() {
            "hsk1" => HSKLevel::HSK1,
            "hsk2" => HSKLevel::HSK2,
            "hsk3" => HSKLevel::HSK3,
            "hsk4" => HSKLevel::HSK4,
            "hsk5" => HSKLevel::HSK5,
            "hsk6" => HSKLevel::HSK6,
            "hsk7-9" => HSKLevel::HSK7_9,
            _ => HSKLevel::HSK1,
        }
    }
}

/// Get a list of chinese character and it's associated HSK level based on the generated hsk.csv.
/// If no hsk.csv is founded then we return an empty hashmap.
///
/// # Arguments
///
/// * `path` - S
pub fn from_csv<S>(path: S) -> Result<HashMap<String, HSKLevel>, Error>
where
    S: AsRef<str>,
{
    let file = match std::fs::read_to_string(path.as_ref()) {
        Ok(file) => file,
        Err(err) => {
            Logger::warn(format!("Unable to found the cedict file, {}", err));

            return Ok(HashMap::new());
        }
    };

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(file.as_bytes());

    let mut hsk = HashMap::new();

    println!("🀄️ Processing HSK file");

    for res in reader.deserialize() {
        let record: HskData = res?;
        hsk.insert(record.s_character, HSKLevel::from_string(record.level));
    }

    Ok(hsk)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_get_hsk_level() {
        let vec: Result<HashMap<String, HSKLevel>, Error> = from_csv("./static/hsk_test.csv");

        assert!(vec.is_ok());

        let hsk = vec.unwrap();

        let hsk1 = hsk.get("水果").unwrap();
        assert_eq!(hsk1, &HSKLevel::HSK1);

        let hsk6 = hsk.get("包装").unwrap();
        assert_eq!(hsk6, &HSKLevel::HSK5);
    }
}
