use crate::error::Error;

// Constant
const VOWEL: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const TONES: [&str; 4] = ["\u{0304}", "\u{0301}", "\u{030c}", "\u{0300}"];
const TONES_U: [&str; 4] = ["ǖ", "ǘ", "ǚ", "ǜ"];
const MEDIAL_VOWEL: [char; 2] = ['i', 'u'];

/// Replace the vowel with the tone accent
///     - Tones are only placed in a vowel
///     - There are some rules. Please refer to readme for the link of the rules but in short:
///         - 1 vowel only -> add the marker tone to the vowel
///         - More than 2 vowels
///             -> If first vowel is a medial vowel then the next letter (vowel) should have the tone marker
///             -> Otherwise, the first vowel has the tone marker
///
/// # Arguments
///
/// * `word` - S
pub fn replace_vowel_with_accent<S: AsRef<str>>(word: S) -> Result<String, Error> {
    let mut chars = word.as_ref().chars().collect::<Vec<char>>();
    let tone_marker = chars.pop().and_then(|c| {
        if c.is_numeric() {
            return Some(c);
        }

        None
    });

    let tone = match tone_marker {
        Some(t) => t
            .to_digit(10)
            .and_then(|index| TONES.get(index as usize - 1))
            .unwrap_or(&""),
        None => return Ok(chars.iter().map(|c| c.to_string()).collect::<String>()),
    };

    // Count the number of vowel which will be used to determine the position of the tone
    let vowel_count = chars.iter().filter(|c| VOWEL.contains(c)).count();
    let vowel_position = get_vowel_position(vowel_count, &chars)?;

    let res = chars
        .into_iter()
        .enumerate()
        .map(|(idx, c)| {
            if idx == vowel_position {
                return format!("{c}{tone}");
            }

            c.to_string()
        })
        .collect::<String>();

    Ok(res)
}

/// Special case where a pinyin has the following tone marker
///     - :1 -> ǖ
///     - :2 -> ǘ
///     - :3 -> ǚ
///     - :4 -> ǜ
///
/// # Arguments
///
/// * `word` - S
pub fn replace_u_tone_with_accent<S: AsRef<str>>(word: S) -> Result<String, Error> {
    // Change the two last character in order to get the tone marker and the value
    let w_ref = word.as_ref();
    if w_ref.len() < 2 {
        return Err(Error::Process(
            "Unable to get tone for vowel 'u'".to_string(),
        ));
    }

    let splitted_word = w_ref.chars().collect::<Vec<char>>();
    let Some(tone_number) = splitted_word.last() else {
        return Err(Error::Process("Unable to get the tone number".to_string()));
    };

    let tone = tone_number
        .to_digit(10)
        .and_then(|index| TONES_U.get(index as usize - 1))
        .unwrap_or(&"");

    let Some(first_char) = splitted_word.first() else {
        return Err(Error::Process(
            "Unable to found the first character of the word".to_string(),
        ));
    };

    Ok(format!("{first_char}{tone}"))
}

/// Get the vowel position which will be use to add the tone marker
///
/// # Arguments
///
/// * `vowel_count` - usize
/// * `mut vowels` - &[char]
fn get_vowel_position(vowel_count: usize, chars: &[char]) -> Result<usize, Error> {
    if vowel_count == 1 {
        return match chars.iter().position(|c| VOWEL.contains(c)) {
            Some(vp) => Ok(vp),
            None => Err(Error::Process("Vowel does not has one vowel".to_string())),
        };
    }

    for (idx, c) in chars.iter().enumerate() {
        // cases where there are more than 1 vowel
        // If the first vowel is a MEDIAL Vowel, then the next vowel which is the next letter
        // is the one who has the marker tone
        if MEDIAL_VOWEL.contains(c) {
            if let Some(next) = chars.get(idx + 1) {
                if VOWEL.contains(next) {
                    return Ok(idx + 1);
                }
            }
        } else if VOWEL.contains(c) {
            // otherwise it's the first vowel that we need to take into account
            // only the first vowel
            return Ok(idx);
        }
    }

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_convert_numeri_pinyin_to_accent() {
        let word = "xian1";

        let expected_word = replace_vowel_with_accent(word).unwrap();
        assert_eq!(expected_word, "xiān");
    }

    #[test]
    fn expect_to_convert_simple_pinyin_to_accent() {
        let word = "chi1";

        let expected_word = replace_vowel_with_accent(word).unwrap();
        assert_eq!(expected_word, "chī");
    }

    #[test]
    fn expect_to_convert_special_tone_marker() {
        let word = "nu:3";

        let expected_word = replace_u_tone_with_accent(word);
        assert_eq!(expected_word.unwrap(), "nǚ");
    }
}
