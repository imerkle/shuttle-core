use std::result;
use std::convert::From;
use std::str;
use base64;
use bigdecimal;
use serde_xdr;

#[derive(Debug)]
pub enum Error {
    InvalidStrKey,
    InvalidStrKeyVersionByte,
    InvalidStrKeyChecksum,
    InvalidSeed,
    TBD,
    Utf8Error(str::Utf8Error),
    DecodeError(base64::DecodeError),
    ParseAmountError(bigdecimal::ParseBigDecimalError),
    SerializationError(serde_xdr::CompatSerializationError),
    DeserializationError(serde_xdr::CompatDeserializationError),
}

pub type Result<T> = result::Result<T, Error>;

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Self {
        Error::Utf8Error(err)
    }
}

impl From<base64::DecodeError> for Error {
    fn from(err: base64::DecodeError) -> Self {
        Error::DecodeError(err)
    }
}

impl From<bigdecimal::ParseBigDecimalError> for Error {
    fn from(err: bigdecimal::ParseBigDecimalError) -> Self {
        Error::ParseAmountError(err)
    }
}

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
