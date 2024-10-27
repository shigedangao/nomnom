use super::converter::SIMPLIFIED;
use crate::{cedict::KeyVariant, converter::TRADTIONAL};

/// Which variant returns the variant of chinese that the text has been written on
///
/// # Arguments
///
/// * `text` - S
pub(crate) fn which_variant<S: AsRef<str>>(text: S) -> Option<KeyVariant> {
    // Assuming that the default is simplified chinese.
    let mut variant = KeyVariant::default();

    // Get the simplified dictionary
    let (simplified_dict, traditional_dict) = SIMPLIFIED.get().zip(TRADTIONAL.get())?;

    let characters = text.as_ref().chars();
    for ch in characters {
        let str_char = ch.to_string();
        // Once we found that the variant is traditional. We directly returns the new variant.
        if simplified_dict.get(&str_char).is_none() && traditional_dict.get(&str_char).is_some() {
            variant = KeyVariant::Traditional;

            return Some(variant);
        }
    }

    Some(variant)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::converter;
    use std::path::PathBuf;

    #[test]
    fn expect_to_detect_traditional() {
        let res =
            converter::initialize_dictionaries(&PathBuf::from("../static/cedict_sample_ts.u8"));
        assert!(res.is_ok());

        let res = super::which_variant("她是我的最好摯友");
        assert_eq!(res.unwrap(), KeyVariant::Traditional);
    }

    #[test]
    fn expect_to_detect_simplified() {
        let res =
            converter::initialize_dictionaries(&PathBuf::from("../static/cedict_sample_ts.u8"));
        assert!(res.is_ok());

        let res = super::which_variant("她是我的最好挚友");
        assert_eq!(res.unwrap(), KeyVariant::Simplified);
    }
}
