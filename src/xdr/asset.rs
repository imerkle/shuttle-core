use std::convert::From;
use std::result;
use std::str;
use try_from::{TryFrom, TryInto};
use serde_xdr;
use error::{Error, Result};
use xdr::keypair::PublicKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alphanum4 {
    #[serde(with = "serde_xdr::opaque_data::fixed_length")] code: [u8; 4],
    issuer: PublicKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alphanum12 {
    #[serde(with = "serde_xdr::opaque_data::fixed_length")] code: [u8; 12],
    issuer: PublicKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Asset {
    Native,
    Alphanum4(Alphanum4),
    Alphanum12(Alphanum12),
}

impl From<::Asset> for Asset {
    fn from(asset: ::Asset) -> Self {
        match asset {
            ::Asset::Native => Asset::Native,
            ::Asset::Credit(ref credit_asset) => {
                let code = credit_asset.code();
                let len = code.len();
                if len <= 4 {
                    let mut code_buf = [0; 4];
                    code_buf[..len].copy_from_slice(code.as_bytes());
                    Asset::Alphanum4(Alphanum4 {
                        code: code_buf,
                        issuer: PublicKey::from(credit_asset.issuer().clone()),
                    })
                } else {
                    // we know for sure code length is less than 12
                    // from ::Asset::new
                    let mut code_buf = [0; 12];
                    code_buf[..len].copy_from_slice(code.as_bytes());
                    Asset::Alphanum12(Alphanum12 {
                        code: code_buf,
                        issuer: PublicKey::from(credit_asset.issuer().clone()),
                    })
                }
            }
        }
    }
}

fn alphanum_to_credit(code: &[u8], issuer: PublicKey) -> Result<::CreditAsset> {
    // Don't copy zero bytes
    let mut pos = 0;
    for i in 0..code.len() {
        if code[i] == 0 {
            break;
        }
        pos += 1;
    }
    let code_ = str::from_utf8(&code[..pos])?;
    let issuer_ = issuer.try_into()?;
    Ok(::CreditAsset::new(code_.to_string(), issuer_)?)
}

impl TryFrom<Asset> for ::Asset {
    type Err = Error;

    fn try_from(asset: Asset) -> result::Result<Self, Error> {
        match asset {
            Asset::Native => Ok(::Asset::Native),
            Asset::Alphanum4(Alphanum4 { code, issuer }) => {
                let credit = alphanum_to_credit(&code, issuer)?;
                Ok(::Asset::Credit(credit))
            }
            Asset::Alphanum12(Alphanum12 { code, issuer }) => {
                let credit = alphanum_to_credit(&code, issuer)?;
                Ok(::Asset::Credit(credit))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use try_from::TryInto;
    use {Asset, CreditAsset, PublicKey};
    use xdr;

    #[test]
    fn test_asset_native() {
        let asset = Asset::Native;
        let xdr_asset = xdr::Asset::from(asset.clone());
        let encoded = xdr::to_base64(&xdr_asset).unwrap();
        assert_eq!(encoded, "AAAAAA==");
        let decoded = xdr::from_base64::<xdr::Asset>(&encoded).unwrap();
        let asset_back: Asset = decoded.try_into().unwrap();
        assert_eq!(asset_back, asset);
    }

    #[test]
    fn test_asset_credit() {
        let issuer = PublicKey::from_account_id(
            "GCLDNMHZTEY6PUYQBYOVERBBZ2W3RLMYOSZWHAMY5R4YW2N6MM4LFA72",
        ).unwrap();
        let test_cases = [
            (
                "A",
                "AAAAAUEAAAAAAAAAljaw+Zkx59MQDh1SRCHOrbitmHSzY4GY7HmLab5jOLI=",
            ),
            (
                "ABCD",
                "AAAAAUFCQ0QAAAAAljaw+Zkx59MQDh1SRCHOrbitmHSzY4GY7HmLab5jOLI=",
            ),
            (
                "ABCDE",
                "AAAAAkFCQ0RFAAAAAAAAAAAAAACWNrD5mTHn0xAOHVJEIc6tuK2YdLNjgZjseYtpvmM4sg==",
            ),
            (
                "ABCDEFGHIJKL",
                "AAAAAkFCQ0RFRkdISUpLTAAAAACWNrD5mTHn0xAOHVJEIc6tuK2YdLNjgZjseYtpvmM4sg==",
            ),
        ];
        for &(code, expected) in test_cases.iter() {
            let asset = Asset::credit(code.to_string(), issuer.clone()).unwrap();
            let xdr_asset = xdr::Asset::from(asset.clone());
            let encoded = xdr::to_base64(&xdr_asset).unwrap();
            assert_eq!(encoded, expected);
            let decoded = xdr::from_base64::<xdr::Asset>(&encoded).unwrap();
            let asset_back: Asset = decoded.try_into().unwrap();
            assert_eq!(asset_back, asset);
        }
    }
}
