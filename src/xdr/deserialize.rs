use std::io::{Cursor, Read};
use serde::Deserialize;
use error::Result;
use serde_xdr;
use base64;

pub fn from_reader<'de, T, R>(mut r: &mut R) -> Result<T>
where
    T: Deserialize<'de>,
    R: Read,
{
    Ok(serde_xdr::from_reader(&mut r)?)
}

pub fn from_base64<'de, T: Deserialize<'de>>(input: &str) -> Result<T> {
    let buf = base64::decode(&input)?;
    let mut cursor = Cursor::new(&buf);
    from_reader(&mut cursor)
}
