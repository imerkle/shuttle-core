//! Elliptic-curve Diffie–Hellman
use crypto;
use curve25519_dalek::scalar::Scalar;
use std::ops::Mul;


/// EC secret key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Curve25519Secret(pub [u8; 32]);

impl Curve25519Secret {
    /// Create a random secret key.
    pub fn random() -> Curve25519Secret {
        let seed = crypto::random_bytes(32);
        let mut data = [0; 32];
        data.copy_from_slice(&seed[..32]);
        Curve25519Secret(data)
    }
}

/// EC public key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Curve25519Public(pub [u8; 32]);

impl Curve25519Public {
    /// Create a public key, derived from the secret key.
    pub fn derive_from_secret(secret: &Curve25519Secret) -> Curve25519Public {
        let scalar = Scalar::from_bytes_mod_order(secret.0);
        let q = [0; 32];
        let scalarq = Scalar::from_bytes_mod_order(q);
        let group_element = scalarq.mul(&scalar);
        Curve25519Public(group_element.to_bytes())
    }
}
