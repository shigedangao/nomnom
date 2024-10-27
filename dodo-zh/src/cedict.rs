use crate::error::Error;
use crate::variant::KeyVariant;
use serde::Serialize;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

// Constant
const CEDICT_SLASH: &str = "/";
const CEDICT_BRACKET: [char; 2] = ['[', ']'];
const VALID_LINE_FILTER: [char; 2] = ['#', '%'];

#[derive(Debug)]
pub struct Dictionary {
    pub items: HashMap<String, Item>,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Item {
    pub traditional_character: String,
    pub simplified_character: String,
    pub pinyin_tone_number: Vec<String>,
    pub translations: Vec<String>,
}

impl Dictionary {
    /// Create a new cedict dictionnary
    ///
    /// # Arguments
    ///
    /// * `path` - PathBuf
    /// * `key_variant` - KeyVariant
    pub fn new(path: &PathBuf, key_variant: KeyVariant) -> Result<Dictionary, Error> {
        let file = File::open(path)?;
        let lines = BufReader::new(file).lines();
        let mut items = HashMap::new();

        for line in lines {
            let line = line?;

            if line.starts_with(VALID_LINE_FILTER) {
                continue;
            }

            // A cedict line is composed using the format below
            // <traditional_chinese> <simplified_chinese> <pinyin> <translations>
            let item = Item::try_from(line)?;
            match key_variant {
                KeyVariant::Simplified => items.insert(item.simplified_character.clone(), item),
                KeyVariant::Traditional => items.insert(item.traditional_character.clone(), item),
            };
        }

        Ok(Dictionary { items })
    }
}

impl TryFrom<String> for Item {
    type Error = Error;

    fn try_from(line: String) -> Result<Self, Self::Error> {
        let translations_split_parts = line.split(CEDICT_SLASH).collect::<Vec<&str>>();

        let rest = translations_split_parts
            .first()
            .ok_or_else(|| Error::Parse("Unable to found the rest".to_string()))?;

        let translations = translations_split_parts
            .get(1..)
            .ok_or_else(|| Error::Parse("Unable to found the translations".to_string()))?
            .iter()
            .filter_map(filter_empty_check)
            .collect::<Vec<_>>();

        let pinyin_split_parts = rest.split(CEDICT_BRACKET).collect::<Vec<_>>();

        let rest = pinyin_split_parts
            .first()
            .ok_or_else(|| Error::Parse("Unable to found the rest".to_string()))?;

        let pinyin = pinyin_split_parts
            .get(1)
            .ok_or_else(|| Error::Parse("Unable to found pinyin".to_string()))?
            .split_whitespace()
            .filter_map(filter_empty_check)
            .collect::<Vec<String>>();

        // Splitting the whitespace allow of the rest allow us to get the traditional & simplified chinese character
        let rest = rest.split_whitespace().collect::<Vec<_>>();

        let traditional_character = rest
            .first()
            .ok_or_else(|| Error::Parse("Unable to found the tradtional character".to_string()))?
            .to_string();

        let simplified_character = rest
            .last()
            .ok_or_else(|| Error::Parse("Unable to found the tradtional character".to_string()))?
            .to_string();

        Ok(Item {
            traditional_character,
            simplified_character,
            pinyin_tone_number: pinyin,
            translations,
        })
    }
}

impl Item {
    /// Get the character for the given key variant
    ///
    /// # Arguments
    ///
    /// * `self` - Item
    /// * `variant` - KeyVariant
    pub(crate) fn get_character_for_key_variant(&self, variant: &KeyVariant) -> String {
        match variant {
            KeyVariant::Simplified => self.simplified_character.clone(),
            KeyVariant::Traditional => self.traditional_character.clone(),
        }
    }
}

/// Filter empty string out and return a string value
///
/// # Arguments
///
/// * `s` - S
fn filter_empty_check<S>(s: S) -> Option<String>
where
    S: AsRef<str>,
{
    if s.as_ref().is_empty() {
        return None;
    }

    Some(s.as_ref().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_parse_line_item() {
        let line = r"一動不動 一动不动 [yi1 dong4 bu4 dong4] /motionless/";
        let item = Item::try_from(line.to_string());

        assert!(item.is_ok());

        let item = item.unwrap();
        assert_eq!(item.traditional_character, "一動不動");
        assert_eq!(item.simplified_character, "一动不动");
        assert_eq!(
            item.pinyin_tone_number,
            vec!["yi1", "dong4", "bu4", "dong4"]
        );
        assert_eq!(item.translations, vec!["motionless"]);
    }
}
