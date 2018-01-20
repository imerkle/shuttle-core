use std::io::{Cursor, Read, Write};
use serde::{Deserialize, Serialize};
use error::Result;
use serde_xdr;
use base64;

pub trait ToXdr<T: Serialize>: Sized {
    fn to_xdr(&self) -> Result<T>;

    fn to_writer<W: Write>(&self, mut w: &mut W) -> Result<()> {
        let x = self.to_xdr()?;
        Ok(serde_xdr::to_writer(&mut w, &x)?)
    }

    fn to_base64(&self) -> Result<String> {
        let mut buf = Vec::new();
        self.to_writer(&mut buf)?;
        Ok(base64::encode(&buf))
    }
}

pub trait FromXdr<'de, T: Deserialize<'de>>: Sized {
    fn from_xdr(other: T) -> Result<Self>;

    fn from_reader<R: Read>(mut r: &mut R) -> Result<Self> {
        let x = serde_xdr::from_reader(&mut r)?;
        Self::from_xdr(x)
    }
    fn from_base64(input: &str) -> Result<Self> {
        let buf = base64::decode(&input)?;
        let mut cursor = Cursor::new(&buf);
        Self::from_reader(&mut cursor)
    }
}
