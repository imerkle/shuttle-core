use error::{Error, Result};
use crypto::PublicKey;

const MAX_CODE_LEN: usize = 12;

/// Enum representing an asset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Asset {
    /// The native asset (XLM).
    Native,
    /// A non-native asset, identified by asset code/issuer id.
    Credit(CreditAsset),
}

impl Asset {
    /// Create the native asset: Lumens.
    pub fn native() -> Asset {
        Asset::Native
    }

    /// Create the asset with `code` issued by `issuer`.
    pub fn credit<S>(code: S, issuer: PublicKey) -> Result<Asset>
    where
        S: Into<String>,
    {
        let code = code.into();
        let inner = CreditAsset::new(code, issuer)?;
        Ok(Asset::Credit(inner))
    }
}

/// A non-native asset, identified by asset code/issuer id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreditAsset {
    code: String,
    issuer: PublicKey,
}

impl CreditAsset {
    /// Create new credit asset with `code` and `issuer.
    ///
    /// Code must be shorter than 12 characters.
    pub fn new(code: String, issuer: PublicKey) -> Result<CreditAsset> {
        if code.len() > MAX_CODE_LEN {
            Err(Error::InvalidAssetCode)
        } else {
            Ok(CreditAsset { code, issuer })
        }
    }

    /// Return the asset code.
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Return the asset issuer.
    pub fn issuer(&self) -> &PublicKey {
        &self.issuer
    }
}

#[cfg(test)]
mod tests {
    use super::CreditAsset;
    use crypto::PublicKey;

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
