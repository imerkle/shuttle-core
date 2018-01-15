use std::result;
use std::convert::From;
use base64;
use bigdecimal;

#[derive(Debug)]
pub enum Error {
    InvalidStrKey,
    InvalidStrKeyVersionByte,
    InvalidStrKeyChecksum,
    InvalidSeed,
    TBD,
    DecodeError(base64::DecodeError),
    ParseAmountError(bigdecimal::ParseBigDecimalError),
}

pub type Result<T> = result::Result<T, Error>;

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
