# nomnom

Just a small util tool to convert the cedict_ts.u8 into a JSON or CSV file.

## Usage

### Json

```sh
cargo run -- --cedict-path ./cedict_ts.u8 -o ./cedict.json -f json
```

### Csv

```sh
cargo run -- --cedict-path ./cedict_ts.u8 -o ./cedict.csv -f csv
```
