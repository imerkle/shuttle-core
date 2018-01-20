use xdr::PublicKey;
use xdr::TimeBounds;
use xdr::Memo;
use xdr::Operation;
use xdr::DecoratedSignature;
use error::{Error, Result};
use xdr::{FromXdr, ToXdr};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub source: PublicKey,
    pub fee: u32,
    pub sequence: u64,
    pub time_bounds: Option<TimeBounds>,
    pub memo: Memo,
    pub operations: Vec<Operation>,
    pub ext: i32,
}

impl ToXdr<Transaction> for ::Transaction {
    fn to_xdr(&self) -> Result<Transaction> {
        let source = self.source.to_xdr()?;
        let fee = self.fee.0 as u32;
        let sequence = self.sequence;
        let time_bounds = match self.time_bounds {
            None => None,
            Some(ref t) => Some(t.to_xdr()?),
        };
        let memo = self.memo.to_xdr()?;
        let ops_res: Result<Vec<_>> = self.operations.iter().map(|op| op.to_xdr()).collect();
        let operations = ops_res?;
        Ok(Transaction {
            source,
            fee,
            sequence,
            time_bounds,
            memo,
            operations,
            ext: 0,
        })
    }
}

impl<'de> FromXdr<'de, Transaction> for ::Transaction {
    fn from_xdr(tx: Transaction) -> Result<::Transaction> {
        let source = ::PublicKey::from_xdr(tx.source)?;
        let sequence = tx.sequence;
        let time_bounds = match tx.time_bounds {
            None => None,
            Some(t) => Some(::TimeBounds::from_xdr(t)?),
        };
        let memo = ::Memo::from_xdr(tx.memo)?;
        let operations_res: Result<Vec<_>> = tx.operations
            .into_iter()
            .map(|op| ::Operation::from_xdr(op))
            .collect();
        let operations = operations_res?;
        Ok(::Transaction::new(
            source,
            sequence,
            time_bounds,
            memo,
            operations,
        ))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaggedTransaction {
    EnvelopeTypeTx(Transaction),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionSignaturePayload {
    network_id: [u8; 32],
    tagged_transaction: TaggedTransaction,
}

impl ToXdr<TransactionSignaturePayload> for ::TransactionSignaturePayload {
    fn to_xdr(&self) -> Result<TransactionSignaturePayload> {
        let mut network_id = [0; 32];
        if self.network_id.len() > 32 {
            return Err(Error::TBD);
        }
        network_id.copy_from_slice(&self.network_id);
        let tagged_transaction = TaggedTransaction::EnvelopeTypeTx(self.transaction.to_xdr()?);
        Ok(TransactionSignaturePayload {
            network_id,
            tagged_transaction,
        })
    }
}

impl<'de> FromXdr<'de, TransactionSignaturePayload> for ::TransactionSignaturePayload {
    fn from_xdr(payload: TransactionSignaturePayload) -> Result<Self> {
        let mut network_id = Vec::new();
        network_id.extend_from_slice(&payload.network_id);
        let transaction = match payload.tagged_transaction {
            TaggedTransaction::EnvelopeTypeTx(transaction) => ::Transaction::from_xdr(transaction)?,
        };
        Ok(::TransactionSignaturePayload {
            network_id,
            transaction,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionEnvelope {
    transaction: Transaction,
    signatures: Vec<DecoratedSignature>,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use {Account, Amount, Asset, KeyPair, Memo};
    use {OperationBuilder, Transaction, TransactionBuilder};
    use {FromXdr, ToXdr};

    fn do_it(tx: Transaction, expected: &str) {
        let encoded = tx.to_base64().unwrap();
        assert_eq!(encoded, expected);
        let decoded = Transaction::from_base64(&encoded).unwrap();
    }

    #[test]
    fn test_transaction() {
        let kp = KeyPair::from_secret_seed(
            "SDFRU2NGDPXYIY67BVS6L6W4OY33HCFCEJQ73TZZPR3IDYVVI7BVPV5Q",
        ).unwrap();

        let mut account = Account::new(kp.public_key().clone(), 999);
        let tx = TransactionBuilder::new(&mut account)
            .operation(OperationBuilder::inflation().build())
            .operation(
                OperationBuilder::payment(
                    kp.public_key().clone(),
                    Asset::native(),
                    Amount::from_str("123.4").unwrap(),
                ).build(),
            )
            .with_memo(Memo::Id(123))
            .build();
        do_it(tx, "AAAAAGPgfnO/mUw71s3yUF9r1CS8IOgym90QLaqUHgG8D4URAAAAyAAAAAAAAAPoAAAAAAAAAAIAAAAAAAAAewAAAAIAAAAAAAAACQAAAAAAAAABAAAAAGPgfnO/mUw71s3yUF9r1CS8IOgym90QLaqUHgG8D4URAAAAAAAAAABJjViAAAAAAA==");
    }
}
