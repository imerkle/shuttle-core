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
            return Err(Error::InvalidPublicKey);
        }
        let mut buf = [0; 32];
        buf.copy_from_slice(&key);
        Ok(PublicKey::Ed25519(Ed25519 { key: buf }))
    }
}

impl ToXdr<PublicKey> for ed25519_dalek::PublicKey {
    fn to_xdr(&self) -> Result<PublicKey> {
        PublicKey::new(self.as_bytes())
    }
}

impl<'de> FromXdr<'de, PublicKey> for ed25519_dalek::PublicKey {
    fn from_xdr(x: PublicKey) -> Result<ed25519_dalek::PublicKey> {
        match x {
            PublicKey::Ed25519(Ed25519 { key }) => Ok(ed25519_dalek::PublicKey::from_bytes(&key).unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    use ed25519_dalek::PublicKey;
    use crypto::keypair::from_account_id;
    use {FromXdr, ToXdr};

    #[test]
    fn test_public_key() {
        let pk = from_account_id(
            "GCEAKB6W342KSAQ6SVJYROF5W5FJTPZDDOSIOT3Y6CNQ3U2ZBAH7AQN3",
        ).unwrap();
        let encoded = pk.to_base64().unwrap();
        assert_eq!(encoded, "AAAAAIgFB9bfNKkCHpVTiLi9t0qZvyMbpIdPePCbDdNZCA/w");
        let decoded = PublicKey::from_base64(&encoded).unwrap();
        assert_eq!(decoded, pk);
    }
}
