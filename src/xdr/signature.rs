#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecoratedSignature {
    hint: [u8; 4],
    signature: Vec<u8>,
}
