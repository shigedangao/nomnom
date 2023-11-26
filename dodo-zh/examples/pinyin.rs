use dodo_zh;

fn main() {
    let text = "wo3 xi3 huan1 ni3";

    let pinyin_tone_marker = dodo_zh::convert_pinyin_tone_number_to_tone_mark(text).unwrap();
    println!("{pinyin_tone_marker}");

    let zhuyin = dodo_zh::convert_pinyin_to_zhuyin(&pinyin_tone_marker).unwrap();
    println!("{zhuyin}");

    let wade_giles = dodo_zh::convert_pinyin_to_wade_giles(pinyin_tone_marker).unwrap();
    println!("{wade_giles}");
}
