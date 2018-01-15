use std::io::Write;
use serde::Serialize;
use error::Result;
use serde_xdr;
use base64;

pub fn to_writer<T, W>(mut w: &mut W, data: &T) -> Result<()>
where
    T: Serialize,
    W: Write,
{
    Ok(serde_xdr::to_writer(&mut w, &data)?)
}

pub fn to_base64<T: Serialize>(data: &T) -> Result<String> {
    let mut buf = Vec::new();
    to_writer(&mut buf, &data)?;
    Ok(base64::encode(&buf))
}
