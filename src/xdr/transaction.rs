use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error as SerdeError;
use std::result;
use serde_xdr::opaque_data;
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

#[derive(Debug, Clone)]
pub enum EnvelopeType {
    Tx = 2,
}

impl Serialize for EnvelopeType {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.clone() as i32)
    }
}

impl<'de> Deserialize<'de> for EnvelopeType {
    fn deserialize<D>(deserializer: D) -> result::Result<EnvelopeType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let case = i32::deserialize(deserializer)?;
        match case {
            2 => Ok(EnvelopeType::Tx),
            t => Err(D::Error::custom(format!("Unknown EnvelopeType {}", t))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionSignaturePayload {
    #[serde(with = "opaque_data::fixed_length")] network_id: [u8; 32],
    envelope_type: EnvelopeType,
    transaction: Transaction,
}

impl<'a> ToXdr<TransactionSignaturePayload>
    for super::super::transaction::TransactionSignaturePayload<'a> {
    fn to_xdr(&self) -> Result<TransactionSignaturePayload> {
        let mut network_id = [0; 32];
        if self.network_id.len() > 32 {
            return Err(Error::InvalidNetworkId);
        }
        network_id.copy_from_slice(&self.network_id);
        let transaction = self.transaction.to_xdr()?;
        Ok(TransactionSignaturePayload {
            network_id,
            envelope_type: EnvelopeType::Tx,
            transaction,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionEnvelope {
    pub transaction: Transaction,
    pub signatures: Vec<DecoratedSignature>,
}

impl ToXdr<TransactionEnvelope> for ::SignedTransaction {
    fn to_xdr(&self) -> Result<TransactionEnvelope> {
        let transaction = self.transaction().to_xdr()?;
        let signatures_res: Result<Vec<_>> = self.signatures().iter().map(|s| s.to_xdr()).collect();
        let signatures = signatures_res?;
        Ok(TransactionEnvelope {
            transaction,
            signatures,
        })
    }
}

impl<'de> FromXdr<'de, TransactionEnvelope> for ::SignedTransaction {
    fn from_xdr(envelope: TransactionEnvelope) -> Result<::SignedTransaction> {
        let transaction = ::Transaction::from_xdr(envelope.transaction)?;
        let signatures_res: Result<Vec<_>> = envelope
            .signatures
            .into_iter()
            .map(|s| ::DecoratedSignature::from_xdr(s))
            .collect();
        let signatures = signatures_res?;
        Ok(::SignedTransaction::new_without_network(
            transaction,
            signatures,
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use {Account, Amount, Asset, KeyPair, Memo, Network};
    use {OperationBuilder, TransactionBuilder};
    use {FromXdr, ToXdr};
    use serde::{Deserialize, Serialize};

    fn do_it<'de, U, T>(tx: T, expected: &str)
    where
        U: Deserialize<'de> + Serialize,
        T: ToXdr<U> + FromXdr<'de, U>,
    {
        let encoded = tx.to_base64().unwrap();
        assert_eq!(encoded, expected);
        let _decoded = T::from_base64(&encoded).unwrap();
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

    #[test]
    fn test_signed_transaction() {
        let kp = KeyPair::from_secret_seed(
            "SDFRU2NGDPXYIY67BVS6L6W4OY33HCFCEJQ73TZZPR3IDYVVI7BVPV5Q",
        ).unwrap();

        let mut account = Account::new(kp.public_key().clone(), 999);
        let tx = TransactionBuilder::new(&mut account)
            .operation(OperationBuilder::inflation().build())
            .build();
        let network = Network::public_network();
        let signed_tx = tx.sign(&kp, &network).unwrap();
        let expected_signature_base = vec![
            0x7A, 0xC3, 0x39, 0x97, 0x54, 0x4E, 0x31, 0x75, 0xD2, 0x66, 0xBD, 0x2, 0x24, 0x39,
            0xB2, 0x2C, 0xDB, 0x16, 0x50, 0x8C, 0x1, 0x16, 0x3F, 0x26, 0xE5, 0xCB, 0x2A, 0x3E,
            0x10, 0x45, 0xA9, 0x79, 0x0, 0x0, 0x0, 0x2, 0x0, 0x0, 0x0, 0x0, 0x63, 0xE0, 0x7E, 0x73,
            0xBF, 0x99, 0x4C, 0x3B, 0xD6, 0xCD, 0xF2, 0x50, 0x5F, 0x6B, 0xD4, 0x24, 0xBC, 0x20,
            0xE8, 0x32, 0x9B, 0xDD, 0x10, 0x2D, 0xAA, 0x94, 0x1E, 0x1, 0xBC, 0xF, 0x85, 0x11, 0x0,
            0x0, 0x0, 0x64, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0xE8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x9, 0x0, 0x0, 0x0,
            0x0,
        ];
        assert_eq!(signed_tx.signature_base().unwrap(), expected_signature_base);
        do_it(signed_tx, "AAAAAGPgfnO/mUw71s3yUF9r1CS8IOgym90QLaqUHgG8D4URAAAAZAAAAAAAAAPoAAAAAAAAAAAAAAABAAAAAAAAAAkAAAAAAAAAAbwPhREAAABAkqlNirgebGCMoc0kdl7FLMl/k2q36LZN1EI7+kfY5xiGg9Mb0txYsIZY3zx1RREQywp/wgpLTpfHqIcnDs2HAg==");
    }
}
