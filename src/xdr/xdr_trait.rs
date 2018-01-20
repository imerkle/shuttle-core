use std::io::{Cursor, Read, Write};
use serde::{Deserialize, Serialize};
use error::Result;
use serde_xdr;
use base64;

/// A trait to try and serialize some type into an XDR object.
pub trait ToXdr<T: Serialize>: Sized {
    /// Build the object that can be serialized to XDR.
    fn to_xdr(&self) -> Result<T>;

    /// Serialize to the writer `w`.
    fn to_writer<W: Write>(&self, mut w: &mut W) -> Result<()> {
        let x = self.to_xdr()?;
        Ok(serde_xdr::to_writer(&mut w, &x)?)
    }

    /// Serialize to base64.
    fn to_base64(&self) -> Result<String> {
        let mut buf = Vec::new();
        self.to_writer(&mut buf)?;
        Ok(base64::encode(&buf))
    }
}

/// A trait to try and deserialize an XDR object into some type.
pub trait FromXdr<'de, T: Deserialize<'de>>: Sized {
    /// Build the type from the XDR `other` object .
    fn from_xdr(other: T) -> Result<Self>;

    /// Deserialize from a reader `r`.
    fn from_reader<R: Read>(mut r: &mut R) -> Result<Self> {
        let x = serde_xdr::from_reader(&mut r)?;
        Self::from_xdr(x)
    }

    /// Deserialize from base64.
    fn from_base64(input: &str) -> Result<Self> {
        let buf = base64::decode(&input)?;
        let mut cursor = Cursor::new(&buf);
        Self::from_reader(&mut cursor)
    }
}
