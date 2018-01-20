use error::{Error, Result};
use xdr::{FromXdr, ToXdr};
use serde_xdr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ed25519 {
    #[serde(with = "serde_xdr::opaque_data::fixed_length")] pub key: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PublicKey {
    Ed25519(Ed25519),
}

impl PublicKey {
    pub fn new(key: &[u8]) -> Result<PublicKey> {
        if key.len() > 32 {
            return Err(Error::TBD);
        }
        let mut buf = [0; 32];
        buf.copy_from_slice(&key);
        Ok(PublicKey::Ed25519(Ed25519 { key: buf }))
    }
}

impl ToXdr<PublicKey> for ::PublicKey {
    fn to_xdr(&self) -> Result<PublicKey> {
        PublicKey::new(self.buf())
    }
}

impl<'de> FromXdr<'de, PublicKey> for ::PublicKey {
    fn from_xdr(x: PublicKey) -> Result<::PublicKey> {
        match x {
            PublicKey::Ed25519(Ed25519 { key }) => Ok(::PublicKey::from_slice(&key)?),
        }
    }
}

#[cfg(test)]
mod tests {
    use PublicKey;
    use {FromXdr, ToXdr};

    #[test]
    fn test_public_key() {
        let pk = PublicKey::from_account_id(
            "GCEAKB6W342KSAQ6SVJYROF5W5FJTPZDDOSIOT3Y6CNQ3U2ZBAH7AQN3",
        ).unwrap();
        let encoded = pk.clone().to_base64().unwrap();
        assert_eq!(encoded, "AAAAAIgFB9bfNKkCHpVTiLi9t0qZvyMbpIdPePCbDdNZCA/w");
        let decoded = PublicKey::from_base64(&encoded).unwrap();
        assert_eq!(decoded, pk);
    }
}
