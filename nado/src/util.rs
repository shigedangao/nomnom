use anyhow::Result;
use csv::Writer;
use serde::Serialize;

/// Convert a slice of type T that can be serialize into a csv representation
///
/// # Arguments
///
/// * `items` - &[T]
pub fn into_csv_string<T>(items: &[T]) -> Result<String>
where
    T: Serialize,
{
    let mut wrt = Writer::from_writer(vec![]);

    for item in items {
        wrt.serialize(item)?;
    }

    let out = wrt.into_inner().map(|inner| String::from_utf8(inner))??;

    Ok(out)
}
