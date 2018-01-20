use std::str;
use serde_xdr;
use error::Result;
use xdr::keypair::PublicKey;
use xdr::{FromXdr, ToXdr};

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

impl ToXdr<Asset> for ::Asset {
    fn to_xdr(&self) -> Result<Asset> {
        match *self {
            ::Asset::Native => Ok(Asset::Native),
            ::Asset::Credit(ref credit_asset) => {
                let code = credit_asset.code();
                let len = code.len();
                if len <= 4 {
                    let mut code_buf = [0; 4];
                    code_buf[..len].copy_from_slice(code.as_bytes());
                    Ok(Asset::Alphanum4(Alphanum4 {
                        code: code_buf,
                        issuer: credit_asset.issuer().clone().to_xdr()?,
                    }))
                } else {
                    // we know for sure code length is less than 12
                    // from ::Asset::new
                    let mut code_buf = [0; 12];
                    code_buf[..len].copy_from_slice(code.as_bytes());
                    Ok(Asset::Alphanum12(Alphanum12 {
                        code: code_buf,
                        issuer: credit_asset.issuer().clone().to_xdr()?,
                    }))
                }
            }
        }
    }
}

impl<'de> FromXdr<'de, Asset> for ::Asset {
    fn from_xdr(asset: Asset) -> Result<::Asset> {
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
    let issuer_ = ::PublicKey::from_xdr(issuer)?;
    Ok(::CreditAsset::new(code_.to_string(), issuer_)?)
}

#[cfg(test)]
mod tests {
    use {Asset, CreditAsset, PublicKey};
    use {FromXdr, ToXdr};

    #[test]
    fn test_asset_native() {
        let asset = Asset::Native;
        let encoded = asset.clone().to_base64().unwrap();
        assert_eq!(encoded, "AAAAAA==");
        let decoded = Asset::from_base64(&encoded).unwrap();
        assert_eq!(decoded, asset);
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
            let encoded = asset.clone().to_base64().unwrap();
            assert_eq!(encoded, expected);
            let decoded = Asset::from_base64(&encoded).unwrap();
            assert_eq!(decoded, asset);
        }
    }
}
