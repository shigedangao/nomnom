use std::sync::OnceLock;
use std::collections::{HashMap, HashSet};

// Provided by
// @link https://www.omniglot.com/chinese/zhuyin.htm
pub(crate) static INITIALS: OnceLock<HashMap<&str, &str>> = OnceLock::new();
pub(crate) static FINALS: OnceLock<HashMap<&str, &str>> = OnceLock::new();
pub(crate) static ACCENTS: OnceLock<HashSet<&str>> = OnceLock::new();

/// Initialize the static initials, medials & finals static value
pub fn initialize_initials_tables() {
    initialize_initials();
    initialize_medials();
    initialize_accents();
}

/// Return an initialized list of initials
fn initialize_initials() {
    INITIALS.get_or_init(|| {
        let mut initials = HashMap::new();
        initials.insert("b", "ㄅ");
        initials.insert("p", "ㄆ");
        initials.insert("m", "ㄇ");
        initials.insert("f", "ㄈ");
        initials.insert("d", "ㄉ");
        initials.insert("t", "ㄊ");
        initials.insert("n", "ㄋ");
        initials.insert("l", "ㄌ");
        initials.insert("g", "ㄍ");
        initials.insert("k", "ㄎ");
        initials.insert("h", "ㄏ");
        initials.insert("j", "ㄐ");
        initials.insert("q", "ㄑ");
        initials.insert("x", "ㄒ");
        initials.insert("zh", "ㄓ");
        initials.insert("zhi", "ㄓ");
        initials.insert("ch", "ㄔ");
        initials.insert("chi", "ㄔ");
        initials.insert("sh", "ㄕ");
        initials.insert("shi", "ㄕ");
        initials.insert("r", "ㄖ");
        initials.insert("z", "ㄗ");
        initials.insert("c", "ㄘ");
        initials.insert("s", "ㄙ");

        initials
    });
}

fn initialize_medials() {
    FINALS.get_or_init(|| {
        let mut finals = HashMap::new();
        finals.insert("a", "ㄚ");
        finals.insert("o", "ㄛ");
        finals.insert("e", "ㄜ");
        finals.insert("ê", "ㄝ");
        finals.insert("ai", "ㄞ");
        finals.insert("ei", "ㄟ");
        finals.insert("ao", "ㄠ");
        finals.insert("ou", "ㄡ");
        finals.insert("an", "ㄢ");
        finals.insert("en", "ㄣ");
        finals.insert("ang", "ㄤ");
        finals.insert("eng", "ㄥ");
        finals.insert("ong", "ㄨㄥ");
        
        finals.insert("yi", "ㄧ");
        finals.insert("i", "ㄧ");
        finals.insert("ya", "ㄧㄚ");
        finals.insert("ia", "ㄧㄚ");
        finals.insert("yo", "ㄧㄛ");
        finals.insert("ye", "ㄧㄝ");
        finals.insert("ie", "ㄧㄝ");
        finals.insert("yai", "ㄧㄞ");
        finals.insert("yao", "ㄧㄠ");
        finals.insert("iao", "ㄧㄠ");
        finals.insert("you", "ㄧㄡ");
        finals.insert("yan", "ㄧㄢ");
        finals.insert("ian", "ㄧㄢ");
        finals.insert("yin", "ㄧㄣ");
        finals.insert("in", "ㄧㄣ");
        finals.insert("yang", "ㄧㄤ");
        finals.insert("iang", "ㄧㄤ");
        finals.insert("ying", "ㄧㄥ");
        finals.insert("ing", "ㄧㄥ");

        finals.insert("u", "ㄨ");
        finals.insert("wu", "ㄨ");
        finals.insert("ua", "ㄨㄚ");
        finals.insert("wa", "ㄨㄚ");
        finals.insert("uo", "ㄨㄛ");
        finals.insert("wo", "ㄨㄛ");
        finals.insert("uai", "ㄨㄞ");
        finals.insert("wai", "ㄨㄞ");
        finals.insert("ui", "ㄨㄟ");
        finals.insert("wei", "ㄨㄟ");
        finals.insert("uan", "ㄨㄢ");
        finals.insert("wan", "ㄨㄢ");
        finals.insert("un", "ㄨㄣ");
        finals.insert("wen", "ㄨㄣ");
        finals.insert("uang", "ㄨㄤ");
        finals.insert("wang", "ㄨㄤ");
        finals.insert("weng", "ㄨㄥ");
        finals.insert("yu", "ㄩ");
        finals.insert("ü", "ㄩ");
        finals.insert("yüe", "ㄩㄝ");
        finals.insert("üe", "ㄩㄝ");
        finals.insert("üan", "ㄩㄢ");
        finals.insert("yüan", "ㄩㄢ");
        finals.insert("ün", "ㄩㄣ");
        finals.insert("yün", "ㄩㄣ");
        finals.insert("er", "ㄦ");

        finals
    });
}


fn initialize_accents() {
    ACCENTS.get_or_init(|| {
        let mut accents = HashSet::new();
        accents.insert("\u{0301}");
        accents.insert("\u{030c}");
        accents.insert("\u{0300}");

        accents
    });
}

