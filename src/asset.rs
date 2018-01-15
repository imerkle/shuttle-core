use error::{Error, Result};
use keypair::PublicKey;

const MAX_CODE_LEN: usize = 12;

/// Enum representing an asset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Asset {
    /// The native asset (XLM).
    Native,
    /// A non-native asset, identified by asset code/issuer id.
    Credit(CreditAsset),
}

/// A non-native asset, identified by asset code/issuer id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreditAsset {
    code: String,
    issuer: PublicKey,
}

impl CreditAsset {
    pub fn new(code: String, issuer: PublicKey) -> Result<CreditAsset> {
        if code.len() > MAX_CODE_LEN {
            Err(Error::TBD)
        } else {
            Ok(CreditAsset { code, issuer })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CreditAsset;
    use keypair::PublicKey;

    #[test]
    fn test_error_code_too_long() {
        let code = "1234567890123".to_string();
        let pk = PublicKey::from_account_id(
            "GCZHXL5HXQX5ABDM26LHYRCQZ5OJFHLOPLZX47WEBP3V2PF5AVFK2A5D",
        ).unwrap();
        let asset = CreditAsset::new(code, pk);
        assert!(asset.is_err());
    }
}
