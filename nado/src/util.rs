use anyhow::Result;
use csv::WriterBuilder;
use serde::Serialize;

pub trait IntoRecord {
    fn into_record(&self) -> Vec<String> { vec![] }
}

/// Convert a slice of type T that can be serialize into a csv representation
///
/// # Arguments
///
/// * `items` - &[T]
pub fn into_csv_string<T>(items: &[T], headers: Option<Vec<&str>>) -> Result<String>
where
    T: Serialize + IntoRecord
{
    let mut wrt = WriterBuilder::new()
        .delimiter(b';')
        .from_writer(vec![]);

    if let Some(head) = headers {
        wrt.write_record(head)?;
    }

    for item in items {
        wrt.serialize(item)
            .or_else(|_| wrt.serialize(item.into_record()))?;
    }

    let out = wrt.into_inner().map(|inner| String::from_utf8(inner))??;

    Ok(out)
}
