use super::{CliArgs, CommandRunner, OutputFormat};
use crate::hsk;
use crate::{hsk::HSKLevel, util};
use anyhow::Result;
use async_trait::async_trait;
use dodo::cedict::{Item, KeyVariant};
use serde::Serialize;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub struct Gen;

#[derive(Debug, Default, Clone, Serialize)]
pub struct CedictItem {
    #[serde(flatten)]
    pub cedict_item: Item,
    pub pinyin_tone_marker: Vec<String>,
    pub zhuyins: Vec<String>,
    pub wades: Vec<String>,
    pub hsk_level: Option<HSKLevel>,
}

#[async_trait]
impl CommandRunner for Gen {
    async fn run(args: &CliArgs) -> Result<()> {
        let path = PathBuf::from(&args.file_path);

        // @TODO uses a logger or something to display...
        println!("Load cedict dictionary");
        // Load the Cedict dictionary
        let mut cedict = dodo::load_cedict_dictionary(path, KeyVariant::Traditional)?;

        // @TODO uses a logger or something to display...
        println!("Load HSK Level");
        // Load the HSK level per character
        let hsks = hsk::load_hsk_levels().await?;

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

                citem
            })
            .collect::<Vec<_>>();

        let output = match args.output_format {
            OutputFormat::Json => serde_json::to_string(&items)?,
            OutputFormat::Csv => util::into_csv_string(&items)?,
        };

        std::fs::write(&args.output_path, output)?;

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
            .filter_map(|p| dodo::convert_pinyin_tone_number_to_tone_mark(p).ok())
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
            .filter_map(|p| dodo::convert_pinyin_to_zhuyin(p).ok())
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
            .filter_map(|p| dodo::convert_pinyin_to_wade_giles(p).ok())
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