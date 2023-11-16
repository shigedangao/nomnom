// Constant
const MEDIAL_VOWEL: [char; 2] = ['i', 'u'];
const VOWEL: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const TONES: [&str; 4] = ["\u{0304}", "\u{0301}", "\u{030c}", "\u{0300}"];
const TONES_U: [&str; 4] = ["ǖ", "ǘ", "ǚ", "ǜ"];

/// Create & manipulate a pinyin to convert into an accent one.
pub struct PinyinAccent(pub String);

impl PinyinAccent {
    /// Replace the tone numberes i.e: xi1 with the tone marks
    /// - Tones are only placed in a vowel
    ///     - There are some rules. Please refer to readme for the link of the rules but in short:
    ///         - 1 vowel only -> add the marker tone to the vowel
    ///         - More than 2 vowels
    ///             -> If first vowel is a medial vowel then the next letter (vowel) should have the tone marker
    ///             -> Otherwise, the first vowel has the tone marker
    ///         - Vowel that are marked for the 'u' character are usually prceded by a ':'. These are replaced by a special 'u' character
    ///
    /// # Arguments
    ///
    /// * `self` - Self
    pub fn replace_tone_numbers_with_tone_marks(&self) -> Option<String> {
        // Get a vector of char and filter out the ':' used to define the tone for word that has the u like nu:3
        let mut chars = self
            .0
            .chars()
            .filter(|c| !c.eq(&':'))
            .collect::<Vec<char>>();

        let tone_marker = chars.pop().filter(|c| c.is_alphanumeric());

        let (tone_index, tone_mark) = match tone_marker {
            Some(t) => {
                let index = t.to_digit(10).unwrap_or_default().overflowing_sub(1).0 as usize;
                (index, TONES.get(index).unwrap_or(&""))
            }
            None => return Some(self.0.clone()),
        };

        let vowel_position = self.get_vowel_position(&chars);
        if vowel_position.is_none() {
            return Some(self.0.clone());
        }

        let vowel_position = vowel_position.unwrap();
        let res = chars
            .into_iter()
            .enumerate()
            .map(|(idx, c)| {
                if idx == vowel_position {
                    match c {
                        'u' => {
                            if let Some(t_u) = TONES_U.get(tone_index) {
                                return t_u.to_string();
                            }
                        }
                        _ => return format!("{c}{tone_mark}"),
                    }
                }

                c.to_string()
            })
            .collect::<String>();

        Some(res)
    }

    /// Get the vowel position on the slice of char
    ///
    /// # Arguments
    ///
    /// * `self` - Self
    /// * `chars` - &[char]
    fn get_vowel_position(&self, chars: &[char]) -> Option<usize> {
        let mut iterator = chars.iter().enumerate();

        while let Some((idx, c)) = iterator.next() {
            // cases where there are more than 1 vowel
            // If the first vowel is a MEDIAL Vowel, then the next vowel which is the next letter
            // is the one who has the marker tone
            if MEDIAL_VOWEL.contains(c) {
                if let Some((n_idx, next)) = iterator.next() {
                    if VOWEL.contains(next) {
                        return Some(n_idx);
                    }
                }

                // Otherwise this mean that we're at the end word meaning that the
                // tone accent is at the last character
                return Some(idx);
            } else if VOWEL.contains(c) {
                // otherwise it's the first vowel that we need to take into account
                // only the first vowel
                return Some(idx);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_convert_numeri_pinyin_to_accent() {
        let word = PinyinAccent("xian1".to_string()).replace_tone_numbers_with_tone_marks();

        assert_eq!(word.unwrap(), "xiān");
    }

    #[test]
    fn expect_to_convert_simple_pinyin_to_accent() {
        let word = PinyinAccent("chi1".to_string()).replace_tone_numbers_with_tone_marks();

        assert_eq!(word.unwrap(), "chī");
    }

    #[test]
    fn expect_to_convert_special_tone_marker() {
        let word: Option<String> =
            PinyinAccent("nu:3".to_string()).replace_tone_numbers_with_tone_marks();

        assert_eq!(word.unwrap(), "nǚ");
    }

    #[test]
    fn expect_to_convert_pinyin_number_not_end() {
        let word: Option<String> =
            PinyinAccent("ping4".to_string()).replace_tone_numbers_with_tone_marks();

        assert_eq!(word.unwrap(), "pìng");
    }
}
