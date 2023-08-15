# Nomnom 🥘

<p align="center">
  <img src="./nomnom.gif" />
</p>

Just a small util tool to convert the cedict_ts.u8 into a JSON or CSV file. Additionals features are:

- Add pinyin with accent based on these [rules](https://web.mit.edu/jinzhang/www/pinyin/spellingrules/index.html#:~:text=(i)%20If%20the%20first%20vowel,letter%20immediately%20following%20the%20medial.&text=(ii)%20If%20the%20first%20vowel,on%20the%20first%20vowel%20letter.&text=(iii)%20If%20the%20tone%20mark,%22%2C%20the%20dot%20is%20omitted.)
- Add HSK level character based fetched on [mandarinbean](https://mandarinbean.com). The HSK7-9 level is parsed from a pdf file provided by [cultureyard](https://www.cultureyard.net/blog/hsk-7-9-vocabulary-list-pdf-free-download)

## Usage

Clone this project and run one of the cargo command below. If needed I could provided the generate json & csv file.

### Json

```sh
cargo run -- generate --cedict-path ./cedict_ts.u8 --zh-hsk-path ./hsk.csv -o ./cedict.json -f json
```

### Csv

```sh
cargo run -- generate --cedict-path ./cedict_ts.u8 --zh-hsk-path ./hsk.csv -o ./cedict.csv -f csv
```
