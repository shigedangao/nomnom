use super::cedict::Item;
use crate::cedict::{Dictionary, KeyVariant};
use crate::Error;
use std::{collections::HashMap, path::PathBuf, sync::OnceLock};

static SIMPLIFIED: OnceLock<HashMap<String, Item>> = OnceLock::new();
static TRADTIONAL: OnceLock<HashMap<String, Item>> = OnceLock::new();

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

/// Convert a text to a desired variant e.g: simplified -> traditional
///
/// # Arguments
///
/// * `text` - S
/// * `input_variant` - KeyVariant
/// * `target_variant`- KeyVariant
pub(crate) fn convert_text_to_desired_variant<S: AsRef<str>>(
    text: S,
    input_variant: KeyVariant,
    target_variant: KeyVariant,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_transform_traditional_to_simplified() {
        let text = "她是我的最好摯友";

        let res = super::initialize_dictionaries(&PathBuf::from("../static/cedict_sample_ts.u8"));
        assert!(res.is_ok());

        let converted = super::convert_text_to_desired_variant(
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

        let converted = super::convert_text_to_desired_variant(
            text,
            KeyVariant::Simplified,
            KeyVariant::Traditional,
        )
        .unwrap();

        assert_eq!(converted, "她是我的最好摯友");
    }
}
