use super::{CommandRunner, GenerateArgs, OutputFormat};
use crate::hsk;
use crate::progress::ProgressBuilder;
use crate::{hsk::HSKLevel, util};
use anyhow::Result;
use dodo_zh::cedict::{Item, KeyVariant};
use serde::Serialize;
use std::{collections::HashMap, path::PathBuf};

const CSV_HEADERS: [&str; 8] = [
    "traditional_character",
    "simplified_character",
    "pinyin_tone_number",
    "translations",
    "pinyin_tone_mark",
    "zhuyins",
    "wade_giles",
    "hsk_level",
];

#[derive(Debug)]
pub struct Gen {
    args: GenerateArgs,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct CedictItem {
    #[serde(flatten)]
    pub cedict_item: Item,
    pub pinyin_tone_marker: Vec<String>,
    pub zhuyins: Vec<String>,
    pub wades: Vec<String>,
    pub hsk_level: Option<HSKLevel>,
}

impl Gen {
    pub fn new(args: GenerateArgs) -> Self {
        Self { args }
    }
}

impl CommandRunner for Gen {
    async fn run(&self) -> Result<()> {
        let path = PathBuf::from(&self.args.file_path);

        println!("📖 - Loading cedict dictionary");
        // Load the Cedict dictionary
        let mut cedict = dodo_zh::load_cedict_dictionary(path, KeyVariant::Traditional)?;

        // Load the HSK level per character
        let hsks = hsk::load_hsk_levels().await.unwrap_or_else(|_| {
            println!("⚠️ Unable to download HSK data");

            HashMap::new()
        });

        println!("⚙️ - Processing cedict items...");
        let mut pb = ProgressBuilder::new(cedict.items.len() as u64);

        let items = cedict
            .items
            .drain()
            .map(|(_, item)| {
                let citem = CedictItem {
                    cedict_item: item,
                    ..Default::default()
                }
                .generate_pinyin_tone_marker()
                .generate_zhuyin_from_pinyin()
                .generate_wade_giles_from_pinyin()
                .fill_hsk_field(&hsks);

                pb.inc(1);

                citem
            })
            .collect::<Vec<_>>();

        pb.clear();

        println!(
            "🖊️ - Generating target file with the path {}",
            self.args.output_path
        );

        let output = match self.args.output_format {
            OutputFormat::Json => serde_json::to_string(&items)?,
            OutputFormat::Csv => util::as_csv_string(&items, Some(CSV_HEADERS.to_vec()))?,
        };

        std::fs::write(&self.args.output_path, output)?;

        Ok(())
    }
}

impl CedictItem {
    /// Generate a list of pinyin with the tone marker
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Self
    fn generate_pinyin_tone_marker(&mut self) -> &mut Self {
        let pinyins = self
            .cedict_item
            .pinyin_tone_number
            .iter()
            .filter_map(|p| dodo_zh::convert_pinyin_tone_number_to_tone_mark(p).ok())
            .collect::<Vec<String>>();

        self.pinyin_tone_marker = pinyins;

        self
    }

    /// Generate a list of zhuyin from the pinyin
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Self
    fn generate_zhuyin_from_pinyin(&mut self) -> &mut Self {
        let zhuyins = self
            .cedict_item
            .pinyin_tone_number
            .iter()
            .filter_map(|p| dodo_zh::convert_pinyin_to_zhuyin(p).ok())
            .collect::<Vec<String>>();

        self.zhuyins = zhuyins;

        self
    }

    /// Generate a list of wade giles based on the pinyin
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Self
    fn generate_wade_giles_from_pinyin(&mut self) -> &mut Self {
        let wades = self
            .cedict_item
            .pinyin_tone_number
            .iter()
            .filter_map(|p| dodo_zh::convert_pinyin_to_wade_giles(p).ok())
            .collect::<Vec<String>>();

        self.wades = wades;

        self
    }

    /// Fill the hsk level field if the character is found on the dictionary of hsk characters
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Self
    /// * `hsks` - &HashMap<String, HSKLevel>
    fn fill_hsk_field(&mut self, hsks: &HashMap<String, HSKLevel>) -> Self {
        if let Some(level) = hsks.get(&self.cedict_item.simplified_character) {
            self.hsk_level = Some(level.to_owned());
        }

        self.clone()
    }
}

impl util::AsRecord for CedictItem {
    fn as_record(&self) -> Vec<String> {
        let hsk_str = self
            .hsk_level
            .as_ref()
            .map(|h| h.to_string())
            .unwrap_or_default();

        vec![
            self.cedict_item.traditional_character.to_owned(),
            self.cedict_item.simplified_character.to_owned(),
            self.cedict_item.pinyin_tone_number.join(","),
            self.cedict_item.translations.join(","),
            self.pinyin_tone_marker.join(","),
            self.zhuyins.join(","),
            self.wades.join(","),
            hsk_str,
        ]
    }
}
