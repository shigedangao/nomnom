use crate::error::Error;
use crate::Logger;

/// Convert a single pinyin to a wade giles value
///     1. The first rules will be based at matching the prefix
///     2. Change the suffix
///
/// # Arguments
///
/// * `pinyin` - S
fn convert_single_pinyin_to_wade_giles<S: AsRef<str>>(pinyin: S) -> Result<String, Error> {
    let mut working_str = pinyin.as_ref();
    let mut last_char = String::new();

    if let Some(last) = working_str.chars().last() {
        if last.is_numeric() {
            working_str = working_str.trim_end_matches(char::is_numeric);
            last_char = last.to_string();
        }
    }

    let Some(first_char) = working_str.get(0..1) else {
        Logger::info("Skipping {working_str}");

        return Ok("".to_string());
    };

    let res_first_step = match first_char {
        "b" => working_str.replace('b', "p"),
        "c" => {
            let two_sets_char = working_str.get(0..2).unwrap_or("");
            match two_sets_char {
                "ch" => working_str.replace("ch", "ch'"),
                "ci" => working_str.replace("ci", "tz'u"),
                "co" | "cu" => working_str.replace('c', "t").replace(['o', 'u'], "s'"),
                _ => working_str.replace('c', "ts'"),
            }
        }
        "d" => working_str.replace('d', "t"),
        "g" => working_str.replace('g', "k"),
        "j" => working_str.replace('j', "ch"),
        "k" => working_str.replace('k', "k'"),
        "p" => working_str.replace('p', "p'"),
        "q" => working_str.replace('q', "ch'"),
        "r" => working_str.replace('r', "j"),
        "t" => working_str.replace('t', "t'"),
        "x" => working_str.replace('x', "hs"),
        "z" => match working_str.get(0..2).unwrap_or("") {
            "zh" => working_str.replace("zh", "ch"),
            "zi" => working_str.replace("zi", "tzu"),
            _ => working_str.replace('z', "ts"),
        },
        "y" => match working_str.get(0..2).unwrap_or("") {
            "yi" => working_str.replace("yi", "i"),
            _ => working_str.to_string(),
        },
        _ => working_str.to_string(),
    };

    // Replace the end of the character with the other one
    let end = match res_first_step.len() {
        0..=2 => "",
        _ => res_first_step
            .get(res_first_step.len() - 2..res_first_step.len())
            .unwrap_or(""),
    };

    let res_second_step = match end {
        "ie" => res_first_step.replace("ie", "ieh"),
        "üe" => res_first_step.replace("üe", "üeh"),
        "uo" => res_first_step.replace("uo", "o"),
        "en" => res_first_step.replace("en", "un"),
        "er" => res_first_step.replace("er", "erh"),
        "ui" => res_first_step.replace("ui", "uei"),
        "ye" => res_first_step.replace("ye", "yeh"),
        _ => res_first_step.to_string(),
    };

    // Handling case where we need to match more than 2 characters
    let end_three = match res_second_step.len() {
        0..=3 => "",
        _ => res_second_step
            .get(res_second_step.len() - 3..res_second_step.len())
            .unwrap_or(""),
    };

    let final_word = match end_three {
        "ian" => res_second_step.replace("ian", "ien"),
        "ong" => res_second_step.replace("ong", "ung"),
        _ => res_second_step.to_string(),
    };

    Ok(format!("{final_word}{last_char}"))
}

/// Convert a cedict pinyin to a wade translation
///
/// # Arguments
///
/// * `pinyin` - S
pub fn convert_cedict_pinyin_sentence_to_wade<S: AsRef<str>>(pinyin: S) -> Result<String, Error> {
    let pinyins = pinyin.as_ref().split_whitespace().collect::<Vec<&str>>();
    let mut wades = Vec::new();

    for p in pinyins {
        let wade = convert_single_pinyin_to_wade_giles(p)?;
        wades.push(wade);
    }

    Ok(wades.join(" "))
}

#[cfg(test)]
mod tests {

    #[test]
    fn expect_to_parse_pinyin_to_yade() {
        let pinyin = "rong";
        let wade = super::convert_single_pinyin_to_wade_giles(pinyin).unwrap();

        assert_eq!(wade, "jung");
    }

    #[test]
    fn expect_to_parse_other_pinyin_to_yade() {
        let pinyin = "zong";
        let wade = super::convert_single_pinyin_to_wade_giles(pinyin).unwrap();

        assert_eq!(wade, "tsung");
    }

    #[test]
    fn expect_to_keep_the_tones() {
        let pinyin = "chong4";
        let wade = super::convert_single_pinyin_to_wade_giles(pinyin).unwrap();

        assert_eq!(wade, "ch'ung4");
    }

    #[test]
    fn expect_to_parse_cedict_pinyin_to_wades() {
        let pinyin = "xia4 zhu4";
        let wade = super::convert_cedict_pinyin_sentence_to_wade(pinyin).unwrap();

        assert_eq!(wade, "hsia4 chu4");
    }
}
