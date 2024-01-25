use unicode_normalization::UnicodeNormalization;

/// PinyinNumber is handler which allows to convert a pinyin with accent to a pinyin with a number
pub struct PinyinNumber(pub String);

impl PinyinNumber {
    /// Into Number convert the accent to numberss
    pub fn into_number(self) -> String {
        let chars: Vec<char> = self.0.nfd().collect();
        let mut accent = char::default();

        let mut pinyin: Vec<char> = chars
            .into_iter()
            .filter_map(|item| {
                if let Some(acc) = get_char(item) {
                    accent = acc;

                    return None
                }

                Some(item)
            })
            .collect();

        // Pinyin accent number is usually put at the end.
        pinyin.push(accent);

        pinyin.iter().collect()
    }
}

/// Transform the accent into their number representation
///
/// # Arguments
///
/// * `ch` - char
fn get_char(ch: char) -> Option<char> {
    match ch {
        '\u{0304}' => Some('1'),
        '\u{0301}' => Some('2'),
        '\u{030c}' => Some('3'),
        '\u{0300}' => Some('4'),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::PinyinNumber;

    #[test]
    fn expect_to_generate_pinyin_number_from_pinyin_accent() {
        let p = PinyinNumber("wǒ".into()).into_number();

        assert_eq!(p, "wo3");
    }

    #[test]
    fn expect_to_generate_pinyin_number_from_accent_middle() {
        let p = PinyinNumber("huān".into()).into_number();

        assert_eq!(p, "huan1");
    }
}
