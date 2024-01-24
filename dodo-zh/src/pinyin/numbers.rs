use unicode_normalization::UnicodeNormalization;

/// PinyinNumber is handler which allows to convert a pinyin with accent to a pinyin with a number
pub struct PinyinNumber(pub String);

impl PinyinNumber {
    /// Into Number convert the accent to numberss
    pub fn into_number(self) -> String {
        let chars: Vec<char> = self.0.nfd().collect();

        let pinyin: Vec<char> = chars.into_iter().map(get_char).collect();

        pinyin.iter().collect()
    }
}

/// Transform the accent into their number representation
///
/// # Arguments
///
/// * `ch` - char
fn get_char(ch: char) -> char {
    match ch {
        '\u{0304}' => '1',
        '\u{0301}' => '2',
        '\u{030c}' => '3',
        '\u{0300}' => '4',
        _ => ch,
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
}
