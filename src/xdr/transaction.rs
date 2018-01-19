use xdr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub source: xdr::PublicKey,
    pub fee: u32,
    pub sequence: u64,
    pub time_bounds: Option<xdr::TimeBounds>,
    pub memo: xdr::Memo,
    pub operations: Vec<xdr::Operation>,
}
