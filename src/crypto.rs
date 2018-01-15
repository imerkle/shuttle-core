use sodiumoxide;
use sodiumoxide::randombytes;
use sodiumoxide::crypto::hash::sha256;

pub fn hash(m: &[u8]) -> Vec<u8> {
    let digest = sha256::hash(&m);
    digest.0.to_vec()
}

pub fn random_bytes(size: usize) -> Vec<u8> {
    randombytes::randombytes(size)
}

pub fn init() -> bool {
    sodiumoxide::init()
}
