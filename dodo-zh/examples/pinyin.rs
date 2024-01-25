use dodo_zh;

fn main() {
    let text = "wo3 xi3 huan1 ni3";

    let pinyin_tone_marker = dodo_zh::convert_pinyin_tone_number_to_tone_mark(text).unwrap();
    assert_eq!(pinyin_tone_marker, "wǒ xǐ huān nǐ");

    let zhuyin = dodo_zh::convert_pinyin_to_zhuyin(&pinyin_tone_marker).unwrap();
    assert_eq!(zhuyin, "ㄨㄛ̌ ㄒㄧ̌ ㄏㄨㄚ ㄋㄧ̌");

    let wade_giles = dodo_zh::convert_pinyin_to_wade_giles(pinyin_tone_marker).unwrap();
    assert_eq!(wade_giles, "wǒ hsǐ huān nǐ");

    let pinyin_accent = dodo_zh::convert_pinyin_accent_to_pinyin_number("xǐ huān").unwrap();
    assert_eq!(pinyin_accent, "xi3 huan1");
}
