use error::{Error, Result};
use signature::{DecoratedSignature, SignatureHint};
//use network::Network;
use crypto::strkey;
//use crypto;
use ed25519_dalek::{PublicKey, SecretKey};

/// Create from `account_id`, e.g. `GB3KJPLFUYN5VL6R3GU3EGCGVCKFDSD7BEDX42HWG5BWFKB3KQGJJRMA`.
pub fn from_account_id(account_id: &str) -> Result<PublicKey> {
    let bytes = strkey::decode_account_id(&account_id)?;
    from_slice(&bytes)
}
/// Create from raw bytes.
pub fn from_slice(data: &[u8]) -> Result<PublicKey> {
    let key = ed25519_dalek::PublicKey::from_bytes(&data).ok().ok_or(Error::InvalidPublicKey)?;
    Ok(key)
}
/// Get account id from public key
pub fn account_id(public: &ed25519_dalek::PublicKey) -> Result<String> {
    strkey::encode_account_id(public.as_bytes())
}
/*
impl PublicKey {

    /// Return the public key as string, starting with `G`.
    pub fn account_id(&self) -> Result<String> {
        strkey::encode_account_id(&self.key.to_bytes())
    }

    /// Return the inner key buffer.
    pub fn buf(&self) -> &[u8] {
        self.key.as_bytes()SecretKey
    }SecretKey
SecretKey
    /// Return the inner key.SecretKey
    pub fn inner(&self) -> &ed255SecretKey19_dalek::PublicKey {
        &self.keySecretKey
    }SecretKey
}
*/
/// Get secret string from secret key
pub fn secret_seed(key: &SecretKey) -> Result<String> {
    strkey::encode_secret_seed(key.as_bytes())
}
/*
/// The secret key of the account.
#[derive(Debug, Clone)]
pub struct SecretKey {
    /// A field.
    pub key: ed25519_dalek::SecretKey,
}

impl SecretKey {
    /// Return the inner key.
    pub fn inner(&self) -> &ed25519_dalek::SecretKey {
        &self.key
    }

    /// Return the secret key as String, starting with `S`.
    pub fn secret_seed(&self) -> Result<String> {
        strkey::encode_secret_seed(&self.key.to_bytes())
    }
}
*/
/// Get keypair from secret string
pub fn from_secret_seed(data: &str) -> Result<ed25519_dalek::Keypair> {
    let bytes = strkey::decode_secret_seed(&data)?;
    let secret = ed25519_dalek::SecretKey::from_bytes(&bytes).unwrap();
    let public = ed25519_dalek::PublicKey::from(&secret);
    let keypair = ed25519_dalek::Keypair{secret, public};
    Ok(keypair)
}
/// sign a message
pub fn sign_decorated(kp: &ed25519_dalek::Keypair, message: &[u8]) -> DecoratedSignature {
    let hint = signature_hint(&kp);
    let signature = kp.sign(message);
    DecoratedSignature::new(hint, signature)
}
/// signature hint
pub fn signature_hint(kp: &ed25519_dalek::Keypair) -> SignatureHint {
    SignatureHint::from_public_key(&kp.public)
}
/*
impl KeyPair {
    /// Create the key pair from the secret seed, e.g. `SDAKFNYEIAORZKKCYRILFQKLLOCNPL5SWJ3YY5NM3ZH6GJSZGXHZEPQS`.
    pub fn from_secret_seed(data: &str) -> Result<KeyPair> {
        let bytes = strkey::decode_secret_seed(&data)?;
        Self::from_seed_bytes(&bytes)
    }

    /// Create a random key pair.
    pub fn random() -> Result<KeyPair> {
        let seed = crypto::random_bytes(32);
        Self::from_seed_bytes(&seed)
    }

    /// Create a key pair from the `network` passphrase.
    pub fn from_network(network: &Network) -> Result<KeyPair> {
        let bytes = network.network_id();
        Self::from_seed_bytes(&bytes)
    }

    /// Crete a key pair from raw bytes.
    pub fn from_seed_bytes(data: &[u8]) -> Result<KeyPair> {
        let keypair = ed25519_dalek::Keypair::from_bytes(&data).ok().ok_or(Error::InvalidSeed)?;
        let public = PublicKey { key: keypair.public };
        let secret = SecretKey { key: keypair.secret };

        Ok(KeyPair { public, secret })
    }

    /// Return the public key.
    pub fn public_key(&self) -> &PublicKey {
        &self.public
    }

    /// Return the secret key.
    pub fn secret_key(&self) -> &SecretKey {
        &self.secret
    }

    /// Sign the `message`.
    pub fn sign(&self, message: &[u8]) -> Signature {
        let kp = ed25519_dalek::Keypair{
            public: self.public.key,
            secret: self.secret.key,
        };        
        Signature{sig: kp.sign(&message)}
        
        //Signature::sign(&self.secret, &message)
    }

    /// Sign the `message` together with the signature hint.
    pub fn sign_decorated(&self, message: &[u8]) -> DecoratedSignature {
        let hint = self.signature_hint();
        let signature = self.sign(message);
        DecoratedSignature::new(hint, signature)
    }

    /// Verify the `signature` against the `message`.
    pub fn verify(&self, message: &[u8], signature: &Signature) -> bool {
        let kp = ed25519_dalek::Keypair{
            secret: self.secret.key,
            public: self.public.key,
        };        
        kp.verify(&message, &signature.sig).is_ok()
        //signature.verify(&self.public, &message)
    }

    /// Return the signature hint, that is the last 4 bytes of the public key.
    pub fn signature_hint(&self) -> SignatureHint {
        SignatureHint::from_public_key(&self.public)
    }
}
*/
#[cfg(test)]
mod tests {
    use super::{from_secret_seed, account_id, sign_decorated};
    use Network;
    #[test]
    fn test_from_secret_seed() {
        let keypairs = [
            (
                "SCRG6SFG64YDEVGWDWTZBE6BWEW25WICOGOUTODVICCA3L3FOQIG26E6",
                "GDRZ6UR4VI3DWATUKAXCGCGAUXGF3IJYG5T3CTJDUJ674TFLN4AR6RV4",
            ),
            (
                "SCOHYGOOQBONAMHLFFTS3OHG2V45GHRVU6Q5HI7VZ2T4C3WRSVCUAWRN",
                "GB5YW3UGJQ635LBGCXJVU3D2TXJIQRX7LRRWFMD2PKZLQ3G2OIZY4ZWO",
            ),
            (
                "SD4IDUZJZKPWU2T2AJWB35AN42L3NADBRCXHL5HY4WXPWO43MQTMCIIF",
                "GCEAKB6W342KSAQ6SVJYROF5W5FJTPZDDOSIOT3Y6CNQ3U2ZBAH7AQN3",
            ),
            (
                "SBB3NC6JM6IT6RXXQKMWH5XBGWWVWXL2MDBVOUPWWKP52QRFB2ISF2IR",
                "GCG5C6WXDSN55I25M7C6734EYVJ562KR4CKWEKK7RISUISNO36LX5J7G",
            ),
            (
                "SDGLIKRUQX5WJCLKGJUSAG3XK7GW262H7SQMLCKV4K5RWXNOWFD5VFG6",
                "GAGKPXZNQ7QDYKRMKE34IB7S5SUKPK4AWETZRBG6CYK3TNANV4HQHJGV",
            ),
            (
                "SD3AXQQRD4HXWHWVX7R56UHPT7ZDJPADTYTC3UTGRYHZ2AR37O66OZQB",
                "GD6KUX4TY4BFKCQBNAFK577IWNB3PNPASBG2JEBIJK6X4CSXNRWCINHU",
            ),
            (
                "SBHRV6BPRPIMTY7EYWFOLUNILQDU3UP5KWQA43UPW7R6WBTWTUGCY446",
                "GARHX6B4IAM2E3WUISLBWYY6TVWWQIWZPUYWXHSQ5BSSV5NKQX5CT4MC",
            ),
            (
                "SA7M4FOPTIT3CNW3B44CQMQRA4A7ZJCAOVHRUEHAMGFVWWCLIA3UXULE",
                "GASQUYLK6GRHBE6Z4A4HH5NHQPAHMHWAUSQP5N7D465QYYBEHDOPFXVP",
            ),
            (
                "SBKNQCZGK2X2VDOIAZWWZ7H3J5XWSU23UGUU62IXVTMWC6AGVN4N3OX7",
                "GD7EMEPGLBYDHUEDSJMAI64BS6HX46SAIPGIF3E5HI4CJ53VQ4OWEEBE",
            ),
            (
                "SA7UGZLM6Q6RR4BC7252IMUAHCZJLDHGBJFKVENIE2HKQ66T7L4T4ZMF",
                "GC22QEDRADRMZ2RPSBGY5N3RMGGSBKCMBQBFCJH3M4RZQ4KQQXUVQNID",
            ),
        ];

        for &(secret, address) in keypairs.iter() {
            let keypair = from_secret_seed(&secret).unwrap();
            let account_id = account_id(&keypair.public).unwrap();
            assert_eq!(&account_id, address);
        }
    }

    #[test]
    fn test_from_network() {
        let network = Network::public_network();
        let bytes: Vec<u8> = network.network_id();
        let secret = ed25519_dalek::SecretKey::from_bytes(&bytes).unwrap();
        let public = ed25519_dalek::PublicKey::from(&secret);
        let kp = ed25519_dalek::Keypair{secret, public};

        let public = account_id(&kp.public).unwrap();
        assert_eq!(
            public,
            "GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7"
        );
    }

    #[test]
    fn test_sign_and_verify() {
        let the_secret = "SD7X7LEHBNMUIKQGKPARG5TDJNBHKC346OUARHGZL5ITC6IJPXHILY36";
        let kp = from_secret_seed(&the_secret).unwrap();
        let message = "test post please ignore".as_bytes();
        let sign = kp.sign(&message);
        let expected_sign = vec![
            0x19, 0xDB, 0xFD, 0xAF, 0x0A, 0xA8, 0x4D, 0xF9, 0xA7, 0xFF, 0x6F, 0xE3, 0xC1, 0x0E,
            0xBC, 0x1F, 0xE2, 0x14, 0xC5, 0x10, 0xB9, 0x5D, 0xB0, 0xD6, 0x33, 0xBE, 0xD9, 0x3D,
            0xF9, 0x25, 0x6B, 0xA9, 0x92, 0xEF, 0x7D, 0x94, 0xB2, 0xA6, 0xE4, 0x54, 0xDE, 0x8F,
            0x21, 0x9, 0x28, 0xCA, 0x96, 0x11, 0x39, 0x03, 0x29, 0xC8, 0x40, 0xC8, 0xE5, 0x64,
            0xE7, 0xA0, 0x72, 0x16, 0x02, 0x7A, 0xB4, 0xA,
        ];
        assert_eq!(sign.to_bytes().to_vec(), expected_sign);
        assert!(kp.verify(&message, &sign).is_ok());
    }

    #[test]
    fn test_sign_decorated() {
        let the_secret = "SD7X7LEHBNMUIKQGKPARG5TDJNBHKC346OUARHGZL5ITC6IJPXHILY36";
        let kp = from_secret_seed(&the_secret).unwrap();
        let message = "test post please ignore".as_bytes();
        let sign = sign_decorated(&kp, &message);
        assert_eq!(sign.hint().to_vec(), vec![0x0B, 0xFA, 0xD1, 0x34]);
    }

}
