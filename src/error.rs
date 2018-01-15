use std::result;
use std::convert::From;
use base64;
//use serde_xdr;

#[derive(Debug)]
pub enum Error {
    InvalidStrKey,
    InvalidStrKeyVersionByte,
    InvalidStrKeyChecksum,
    InvalidSeed,
    TBD,
    DecodeError(base64::DecodeError),
    //SerializationError(serde_xdr::CompatSerializationError),
    //DeserializationError(serde_xdr::CompatDeserializationError),
}

pub type Result<T> = result::Result<T, Error>;

impl From<base64::DecodeError> for Error {
    fn from(err: base64::DecodeError) -> Self {
        Error::DecodeError(err)
    }
}

/*
impl From<serde_xdr::CompatDeserializationError> for Error {
    fn from(err: serde_xdr::CompatDeserializationError) -> Self {
        Error::DeserializationError(err)
    }
}

impl From<serde_xdr::CompatSerializationError> for Error {
    fn from(err: serde_xdr::CompatSerializationError) -> Self {
        Error::SerializationError(err)
    }
}
*/
