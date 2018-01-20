use sodiumoxide::crypto::sign::ed25519;
use error::{Error, Result};
use keypair::{PublicKey, SecretKey};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    sig: ed25519::Signature,
}

impl Signature {
    pub fn sign(secret: &SecretKey, data: &[u8]) -> Signature {
        let sig = ed25519::sign_detached(data, &secret.inner());
        Signature { sig }
    }

    pub fn from_slice(sb: &[u8]) -> Result<Signature> {
        let sig = ed25519::Signature::from_slice(sb).ok_or(Error::TBD)?;
        Ok(Signature { sig })
    }

    pub fn len(&self) -> usize {
        self.sig.0.len()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.sig.0.to_vec()
    }

    pub fn buf(&self) -> &[u8] {
        &self.sig.0
    }

    pub fn verify(&self, public: &PublicKey, data: &[u8]) -> bool {
        ed25519::verify_detached(&self.sig, data, &public.inner())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignatureHint(pub [u8; 4]);

impl SignatureHint {
    pub fn from_public_key(pk: &PublicKey) -> SignatureHint {
        let mut hint: [u8; 4] = Default::default();
        let buf = pk.buf();
        let len = buf.len();
        hint.copy_from_slice(&buf[len - 4..len]);
        SignatureHint(hint)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecoratedSignature {
    hint: SignatureHint,
    signature: Signature,
}

impl DecoratedSignature {
    pub fn new(hint: SignatureHint, signature: Signature) -> DecoratedSignature {
        DecoratedSignature { hint, signature }
    }

    pub fn hint(&self) -> &SignatureHint {
        &self.hint
    }

    pub fn signature(&self) -> &Signature {
        &self.signature
    }
}
