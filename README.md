# nomnom

Just a small util tool to convert the cedict_ts.u8 into a JSON or CSV file. Accent convertion for the pinyin is based on [rules](https://web.mit.edu/jinzhang/www/pinyin/spellingrules/index.html#:~:text=(i)%20If%20the%20first%20vowel,letter%20immediately%20following%20the%20medial.&text=(ii)%20If%20the%20first%20vowel,on%20the%20first%20vowel%20letter.&text=(iii)%20If%20the%20tone%20mark,%22%2C%20the%20dot%20is%20omitted.)

## Usage

### Json

```sh
cargo run -- --cedict-path ./cedict_ts.u8 -o ./cedict.json -f json
```

### Csv

```sh
cargo run -- --cedict-path ./cedict_ts.u8 -o ./cedict.csv -f csv
```
