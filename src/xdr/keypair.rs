use std::convert::From;
use try_from::TryFrom;
use std::result;
use error::Error;
use serde_xdr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ed25519 {
    #[serde(with = "serde_xdr::opaque_data::fixed_length")] pub key: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PublicKey {
    Ed25519(Ed25519),
}

impl From<::PublicKey> for PublicKey {
    fn from(key: ::PublicKey) -> Self {
        let mut buf = [0; 32];
        buf.copy_from_slice(key.buf());
        PublicKey::Ed25519(Ed25519 { key: buf })
    }
}

impl TryFrom<PublicKey> for ::PublicKey {
    type Err = Error;

    fn try_from(key: PublicKey) -> result::Result<Self, Error> {
        match key {
            PublicKey::Ed25519(Ed25519 { key }) => ::PublicKey::from_slice(&key),
        }
    }
}

#[cfg(test)]
mod tests {
    use try_from::TryInto;
    use PublicKey;
    use xdr;

    #[test]
    fn test_public_key() {
        let pk = PublicKey::from_account_id(
            "GCEAKB6W342KSAQ6SVJYROF5W5FJTPZDDOSIOT3Y6CNQ3U2ZBAH7AQN3",
        ).unwrap();
        let xdr_pk = xdr::PublicKey::from(pk.clone());
        let encoded = xdr::to_base64(&xdr_pk).unwrap();
        assert_eq!(encoded, "AAAAAIgFB9bfNKkCHpVTiLi9t0qZvyMbpIdPePCbDdNZCA/w");
        let decoded = xdr::from_base64::<xdr::PublicKey>(&encoded).unwrap();
        let pk_back: PublicKey = decoded.try_into().unwrap();
        assert_eq!(pk_back, pk);
    }
}
