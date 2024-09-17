use core::fmt;

pub struct WadeGiles<S>(pub S)
where
    S: AsRef<str> + Clone;

impl<S> WadeGiles<S>
where
    S: AsRef<str> + Clone + fmt::Display,
{
    /// Convert a single pinyin to a wade giles value
    ///     1. The first rules will be based at matching the prefix
    ///     2. Change the suffix
    ///
    /// # Arguments
    ///
    /// * `pinyin` - S
    pub fn convert_pinyin_to_wade_giles(&mut self) -> String {
        let last_char_numeric_check = self.0.as_ref().chars().last().filter(|v| v.is_numeric());

        let (working_str, last_char) = match last_char_numeric_check {
            Some(v) => (
                self.0
                    .as_ref()
                    .trim_end_matches(char::is_numeric)
                    .to_string(),
                v.to_string(),
            ),
            None => (self.0.as_ref().to_owned(), String::default()),
        };

        let mut handler = WadeGilesWorker(working_str);
        let res = handler
            .process_first_character_set()
            .process_end_characters(2)
            .process_end_characters(3);

        format!("{}{}", res.0, last_char)
    }
}

/// WadeGilesWorker implement the logic to convert a pinyin to a wade giles.
pub struct WadeGilesWorker(pub String);

impl WadeGilesWorker {
    /// Process the first character of the pinyin and some special cases related to this first character
    ///
    /// # Arguments
    ///
    /// * `&mut Self` - Self
    /// * `input` - &str
    fn process_first_character_set(&mut self) -> &mut Self {
        let input = self.0.to_owned();
        let first_character = self.0.get(0..1).unwrap_or_default();

        self.0 = match first_character {
            "b" => input.replace('b', "p"),
            "c" => match input.get(0..2).unwrap_or_default() {
                "ch" => input.replace("ch", "ch'"),
                "ci" => input.replace("ci", "tz'u"),
                "co" | "cu" => input.replace('c', "t").replace(['o', 'u'], "s'"),
                _ => input.replace('c', "ts'"),
            },
            "d" => input.replace('d', "t"),
            "g" => input.replace('g', "k"),
            "j" => input.replace('j', "ch"),
            "k" => input.replace('k', "k'"),
            "p" => input.replace('p', "p'"),
            "q" => input.replace('q', "ch'"),
            "r" => input.replace('r', "j"),
            "t" => input.replace('t', "t'"),
            "x" => input.replace('x', "hs"),
            "z" => match input.get(0..2).unwrap_or_default() {
                "zh" => input.replace("zh", "ch"),
                "zi" => input.replace("zi", "tzu"),
                _ => input.replace('z', "ts"),
            },
            "y" => match input.get(0..2).unwrap_or_default() {
                "yi" => input.replace("yi", "i"),
                _ => input.to_string(),
            },
            _ => input.to_string(),
        };

        self
    }

    /// Process the end characters of the pinyin based on the lookback index
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Self
    /// * `lookback_index` - usize
    fn process_end_characters(&mut self, lookback_index: usize) -> &mut Self {
        let lookback = self.0.len().overflowing_sub(lookback_index).0;
        let end = self.0.get(lookback..).unwrap_or_default();

        self.0 = match end {
            "ie" => self.0.replace("ie", "ieh"),
            "üe" => self.0.replace("üe", "üeh"),
            "uo" => self.0.replace("uo", "o"),
            "en" => self.0.replace("en", "un"),
            "er" => self.0.replace("er", "erh"),
            "ui" => self.0.replace("ui", "uei"),
            "ye" => self.0.replace("ye", "yeh"),
            "ian" => self.0.replace("ian", "ien"),
            "ong" => self.0.replace("ong", "ung"),
            _ => self.0.to_string(),
        };

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_parse_pinyin_to_yade() {
        let wade = WadeGiles("rong").convert_pinyin_to_wade_giles();

        assert_eq!(wade, "jung");
    }

    #[test]
    fn expect_to_parse_other_pinyin_to_yade() {
        let wade = WadeGiles("zong").convert_pinyin_to_wade_giles();

        assert_eq!(wade, "tsung");
    }

    #[test]
    fn expect_to_keep_the_tones() {
        let wade = WadeGiles("chong4").convert_pinyin_to_wade_giles();

        assert_eq!(wade, "ch'ung4");
    }

    #[test]
    fn expect_to_parse_cedict_pinyin_to_wades() {
        let wade = WadeGiles("xia4".to_string()).convert_pinyin_to_wade_giles();

        assert_eq!(wade, "hsia4");
    }
}
