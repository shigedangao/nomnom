# Nomnom 🥘

<p align="center">
  <img src="./nomnom.gif" />
</p>

## Nado - CLI

Just a small util tool to convert the cedict_ts.u8 into a JSON or CSV file. Additionals features are:

- Add pinyin with accent based on these [rules](https://web.mit.edu/jinzhang/www/pinyin/spellingrules/index.html#:~:text=(i)%20If%20the%20first%20vowel,letter%20immediately%20following%20the%20medial.&text=(ii)%20If%20the%20first%20vowel,on%20the%20first%20vowel%20letter.&text=(iii)%20If%20the%20tone%20mark,%22%2C%20the%20dot%20is%20omitted.)
- Add HSK level character based fetched on [mandarinbean](https://mandarinbean.com). The HSK7-9 level is parsed from a different website by [wohok](https://wohok.com/hsk-7-9-vocabulary-list-for-hsk-3-0/)
- Add zhuyin support based on this conversion rules [link](https://www.omniglot.com/chinese/zhuyin.htm)
- Add wade-giles support based on this conversion rules [link](https://www.eastasianlib.org/ctp/RomTable/Chipinyintowade.pdf)

### Usage

Clone this project and run one of the cargo command below. If needed I could provided the generate json & csv file.

#### Json

```sh
cargo run -- generate -e ../cedict_ts.u8 -o ../cedict.json -f json
```

#### Csv

```sh
cargo run -- generate -e ../cedict_ts.u8 -o ../cedict.csv -f csv
```

## Dodo - Lib

A small crates is available which provided a list of utility method to interact with the cedict and doing some pinyin conversion. Below is how you can use the crate to load the cedict

```rust
use dodo_zh;
use dodo_zh::KeyVariant;

fn main() {
    // The KeyVariant can either be Traditional or Simplified chinese
    let cedict = dodo_zh::load_cedict_dictionary(path, KeyVariant::Traditional).unwrap();
    let wo = cedict.items.get("我").unwrap();

    // will return an Item struct
    println!(wo.translations);
}
```

A set of example exist which can helps you to see how to do some pinyin manipulation. Namely
convert the pinyin with tone number to a pinyin with tone marker etc...

You can run the example with the following command

```rust
cargo run --example pinyin
```
