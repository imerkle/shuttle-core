
/// Last 4 bytes of a public key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignatureHint(pub [u8; 4]);

impl SignatureHint {
    /// Create a `SignatureHint` with the last 4 bytes of the public key `pk`.
    pub fn from_public_key(pk: &ed25519_dalek::PublicKey) -> SignatureHint {
        let mut hint: [u8; 4] = Default::default();
        let buf = pk.as_bytes();
        let len = buf.len();
        hint.copy_from_slice(&buf[len - 4..len]);
        SignatureHint(hint)
    }

    /// Convert to `Vec<u8>`.
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

/// A `Signature` together with the last 4 bytes of the public key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecoratedSignature {
    hint: SignatureHint,
    signature: ed25519_dalek::Signature,
}

impl DecoratedSignature {
    /// Create a new `DecoratedSignature` with `hint` and `signature`.
    pub fn new(hint: SignatureHint, signature: ed25519_dalek::Signature) -> DecoratedSignature {
        DecoratedSignature { hint, signature }
    }

    /// Return the decorated signature `hint`.
    pub fn hint(&self) -> &SignatureHint {
        &self.hint
    }

    /// Return the decorated signature `signature`.
    pub fn signature(&self) -> &ed25519_dalek::Signature {
        &self.signature
    }
}
