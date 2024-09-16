use anyhow::Result;
use csv::WriterBuilder;
use serde::Serialize;

pub trait AsRecord {
    /// Transform the item into a record of CSV. This is used
    /// when the csv item could not be serialized by the CSV library when using Serde
    ///
    /// # Arguments
    ///
    /// * `self` - Self
    fn as_record(&self) -> Vec<String>;
}

/// Convert a slice of type T that can be serialize into a csv representation
///
/// # Arguments
///
/// * `items` - &[T]
pub fn as_csv_string<T>(items: &[T], headers: Option<Vec<&str>>) -> Result<String>
where
    T: Serialize + AsRecord,
{
    let mut wrt = WriterBuilder::new().delimiter(b';').from_writer(vec![]);

    if let Some(head) = headers {
        wrt.write_record(head)?;
    }

    for item in items {
        wrt.serialize(item)
            .or_else(|_| wrt.serialize(item.as_record()))?;
    }

    let out = wrt.into_inner().map(String::from_utf8)??;

    Ok(out)
}
