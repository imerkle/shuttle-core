use serde_xdr::opaque_data;
use error::Result;
use xdr::{FromXdr, ToXdr};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecoratedSignature {
    #[serde(with = "opaque_data::fixed_length")] pub hint: [u8; 4],
    // TODO(fracek): Once var length opaque data support is added,
    // change to be a Vec<u8>
    pub len: u32,
    #[serde(with = "opaque_data::fixed_length")] pub signature0: [u8; 32],
    #[serde(with = "opaque_data::fixed_length")] pub signature1: [u8; 32],
}

impl ToXdr<DecoratedSignature> for ::DecoratedSignature {
    fn to_xdr(&self) -> Result<DecoratedSignature> {
        let hint = self.hint().0.clone();
        let signature_vec = self.signature().to_vec();
        let len = signature_vec.len();
        let mut signature0 = [0; 32];
        let mut signature1 = [0; 32];
        if len > 32 {
            signature0.copy_from_slice(&signature_vec[..32]);
            signature1.copy_from_slice(&signature_vec[32..len]);
        } else {
            signature0.copy_from_slice(&signature_vec[..len]);
        }
        Ok(DecoratedSignature {
            hint,
            len: len as u32,
            signature0,
            signature1,
        })
    }
}

impl<'de> FromXdr<'de, DecoratedSignature> for ::DecoratedSignature {
    fn from_xdr(sig: DecoratedSignature) -> Result<::DecoratedSignature> {
        let hint = ::signature::SignatureHint(sig.hint);
        let mut signature_slice = [0; 64];
        signature_slice[..32].copy_from_slice(&sig.signature0);
        signature_slice[32..].copy_from_slice(&sig.signature1);
        let signature = ::signature::Signature::from_slice(&signature_slice)?;
        Ok(::DecoratedSignature::new(hint, signature))
    }
}

#[cfg(test)]
mod tests {
    use {DecoratedSignature, KeyPair};
    use {FromXdr, ToXdr};

    #[test]
    fn test_decorated_signature() {
        let message = vec![
            0x1E, 0x8E, 0x5F, 0xCE, 0x48, 0xDF, 0xB9, 0xBE, 0x40, 0x8, 0x4, 0xA3, 0x2C, 0x5B, 0x81,
            0xB2, 0xDC, 0xEF, 0xA8, 0xE5, 0x1F, 0x6C, 0xA4, 0xD3, 0x24, 0x3B, 0x2F, 0x27, 0x90,
            0xCA, 0x95, 0x48,
        ];
        let kp = KeyPair::from_secret_seed(
            "SDFRU2NGDPXYIY67BVS6L6W4OY33HCFCEJQ73TZZPR3IDYVVI7BVPV5Q",
        ).unwrap();
        let sig = kp.sign_decorated(&message);
        let encoded = sig.to_base64().unwrap();
        assert_eq!(encoded, "vA+FEQAAAECSqU2KuB5sYIyhzSR2XsUsyX+Tarfotk3UQjv6R9jnGIaD0xvS3FiwhljfPHVFERDLCn/CCktOl8eohycOzYcC");
        let decoded = DecoratedSignature::from_base64(&encoded).unwrap();
        assert_eq!(decoded, sig);
    }
}
