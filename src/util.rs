use crate::error::Error;
use serde::Serialize;

/// Convert an items to a JSON string format
///
/// # Arguments
///
/// * `items` - T: Serialize
pub fn to_json<T>(items: T) -> Result<String, Error>
where
    T: Serialize,
{
    serde_json::to_string(&items).map_err(Error::from)
}

/// Convert an items to a CSV string format
///
/// # Arguments
///
/// * `items` - V
pub fn to_csv<V, T>(items: V) -> Result<String, Error>
where
    T: Serialize,
    V: IntoIterator<Item = T>,
{
    let mut wrt = csv::Writer::from_writer(vec![]);

    for item in items {
        wrt.serialize(&item)?;
    }

    let out = wrt
        .into_inner()
        .map_err(|err| Error::Serialize(err.to_string()))?;

    let data = String::from_utf8(out).map_err(|err| Error::Serialize(err.to_string()))?;

    Ok(data)
}
