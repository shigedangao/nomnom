use crate::Error;
use crate::cedict::{Dictionary, Item};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::OnceLock;

// Static variable to handle the different versions of the dictionaries.
pub(crate) static SIMPLIFIED: OnceLock<HashMap<String, Item>> = OnceLock::new();
pub(crate) static TRADTIONAL: OnceLock<HashMap<String, Item>> = OnceLock::new();

/// KeyVariant handle the different supported version of chinese.
#[derive(Debug, PartialEq, Clone)]
pub enum KeyVariant {
    Simplified,
    Traditional,
}

impl Default for KeyVariant {
    fn default() -> Self {
        Self::Simplified
    }
}

/// Initialize Dictionaries for simplified & traditional chinese based on the given cedict file path
///
/// # Arguments
///
/// * `path` - &PathBuf
pub(crate) fn initialize_dictionaries(path: &PathBuf) -> Result<(), Error> {
    let simplified = Dictionary::new(path, KeyVariant::Simplified)?;
    let traditional = Dictionary::new(path, KeyVariant::Traditional)?;

    SIMPLIFIED.get_or_init(|| simplified.items);
    TRADTIONAL.get_or_init(|| traditional.items);

    Ok(())
}

impl KeyVariant {
    /// Convert a text to a desired variant e.g: simplified -> traditional
    ///
    /// # Arguments
    ///
    /// * `text` - S
    /// * `input_variant` - KeyVariant
    /// * `target_variant`- KeyVariant
    pub(crate) fn convert_text_to_desired_variant<S: AsRef<str>>(
        text: S,
        input_variant: Self,
        target_variant: Self,
    ) -> Option<String> {
        // Split the content as a list of character
        let chars = text.as_ref().chars();

        // Use the variant that needed for the conversion
        let dictionary = match input_variant {
            KeyVariant::Simplified => SIMPLIFIED.get()?,
            KeyVariant::Traditional => TRADTIONAL.get()?,
        };

        let mut rebuild_content: Vec<String> = Vec::new();
        // Rebuild the list of character into the target variant
        for c in chars {
            match dictionary.get(&c.to_string()) {
                Some(character) => {
                    rebuild_content.push(character.get_character_for_key_variant(&target_variant))
                }
                // We assume that this may be a non chinese character e.g: space or number.
                None => rebuild_content.push(c.to_string()),
            }
        }

        Some(rebuild_content.join(""))
    }

    /// Which variant returns the variant of chinese that the text has been written on
    ///
    /// # Arguments
    ///
    /// * `text` - S
    pub(crate) fn which_variant<S: AsRef<str>>(text: S) -> Option<Self> {
        // Get the simplified dictionary
        let (simplified_dict, traditional_dict) = SIMPLIFIED.get().zip(TRADTIONAL.get())?;

        let characters = text.as_ref().chars();
        for ch in characters {
            let str_char = ch.to_string();
            // Once we found that the variant is traditional. We directly returns the new variant.
            if simplified_dict.get(&str_char).is_none() && traditional_dict.get(&str_char).is_some()
            {
                return Some(Self::Traditional);
            }
        }

        Some(Self::Simplified)
    }

    /// Detect the chinese character variant by using Unicode
    ///
    /// # Arguments
    ///
    /// * `content` - S
    pub(crate) fn detect_variant_with_unicode<S: AsRef<str>>(content: S) -> Self {
        let characters = content.as_ref().chars();

        for c in characters {
            match c {
                '\u{3400}'..='\u{4DBF}' => return Self::Traditional,
                '\u{20000}'..='\u{2A6DF}' => return Self::Traditional,
                '\u{2A700}'..='\u{2B73F}' => return Self::Traditional,
                '\u{2B740}'..='\u{2B81F}' => return Self::Traditional,
                '\u{2B820}'..='\u{2CEAF}' => return Self::Traditional,
                '\u{2CEB0}'..='\u{2EBEF}' => return Self::Traditional,
                _ => {}
            }
        }

        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn expect_to_transform_traditional_to_simplified() {
        let text = "她是我的最好摯友";

        let res = super::initialize_dictionaries(&PathBuf::from("../static/cedict_sample_ts.u8"));
        assert!(res.is_ok());

        let converted = super::KeyVariant::convert_text_to_desired_variant(
            text,
            KeyVariant::Traditional,
            KeyVariant::Simplified,
        )
        .unwrap();

        assert_eq!(converted, "她是我的最好挚友");
    }

    #[test]
    fn expect_to_convert_simplified_into_traditional() {
        let text = "她是我的最好挚友";

        let res = super::initialize_dictionaries(&PathBuf::from("../static/cedict_sample_ts.u8"));
        assert!(res.is_ok());

        let converted = super::KeyVariant::convert_text_to_desired_variant(
            text,
            KeyVariant::Simplified,
            KeyVariant::Traditional,
        )
        .unwrap();

        assert_eq!(converted, "她是我的最好摯友");
    }

    #[test]
    fn expect_to_detect_traditional() {
        let res = initialize_dictionaries(&PathBuf::from("../static/cedict_sample_ts.u8"));
        assert!(res.is_ok());

        let res = super::KeyVariant::which_variant("她是我的最好摯友");
        assert_eq!(res.unwrap(), KeyVariant::Traditional);
    }

    #[test]
    fn expect_to_detect_simplified() {
        let res = initialize_dictionaries(&PathBuf::from("../static/cedict_sample_ts.u8"));
        assert!(res.is_ok());

        let res = super::KeyVariant::which_variant("她是我的最好挚友");
        assert_eq!(res.unwrap(), KeyVariant::Simplified);
    }

    #[test]
    fn expect_to_detect_traditional_with_unicode() {
        let res = super::KeyVariant::detect_variant_with_unicode("這個牛肉的顏色是𫞩的");
        assert_eq!(res, KeyVariant::Traditional);
    }
}
