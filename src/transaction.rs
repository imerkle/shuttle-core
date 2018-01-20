use amount::Stroops;
use time_bounds::TimeBounds;
use memo::Memo;
use network::Network;
use keypair::{KeyPair, PublicKey};
use signature::DecoratedSignature;
use operation::Operation;
use error::Result;
use xdr::ToXdr;
use crypto;

const BASE_FEE: Stroops = Stroops(100);

#[derive(Debug, Clone)]
pub struct Transaction {
    pub source: PublicKey,
    pub sequence: u64,
    pub fee: Stroops,
    pub time_bounds: Option<TimeBounds>,
    pub memo: Memo,
    pub operations: Vec<Operation>,
}

impl Transaction {
    pub fn new(
        source: PublicKey,
        sequence: u64,
        time_bounds: Option<TimeBounds>,
        memo: Memo,
        operations: Vec<Operation>,
    ) -> Transaction {
        let fee = BASE_FEE * operations.len();
        Transaction {
            source,
            sequence,
            fee,
            time_bounds,
            memo,
            operations,
        }
    }

    pub fn source(&self) -> &PublicKey {
        &self.source
    }

    pub fn base_fee(&self) -> &Stroops {
        &self.fee
    }

    pub fn time_bounds(&self) -> &Option<TimeBounds> {
        &self.time_bounds
    }

    pub fn memo(&self) -> &Memo {
        &self.memo
    }

    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    pub fn operations(&self) -> &Vec<Operation> {
        &self.operations
    }

    pub fn sign(self, keypair: &KeyPair, network: &Network) -> Result<SignedTransaction> {
        let mut sig = SignedTransaction::new(self, network)?;
        sig.sign(keypair)?;
        Ok(sig)
    }
}

#[derive(Debug, Clone)]
pub struct SignedTransaction {
    network_id: Vec<u8>,
    transaction: Transaction,
    signatures: Vec<DecoratedSignature>,
}

impl SignedTransaction {
    pub fn new(transaction: Transaction, network: &Network) -> Result<SignedTransaction> {
        Ok(SignedTransaction {
            network_id: network.network_id().clone(),
            transaction,
            signatures: Vec::new(),
        })
    }

    pub fn new_without_network(
        transaction: Transaction,
        signatures: Vec<DecoratedSignature>,
    ) -> SignedTransaction {
        SignedTransaction {
            network_id: Vec::new(),
            transaction,
            signatures,
        }
    }

    pub fn sign(&mut self, keypair: &KeyPair) -> Result<()> {
        let sig_payload = TransactionSignaturePayload {
            network_id: &self.network_id,
            transaction: &self.transaction,
        };
        let mut payload = Vec::new();
        sig_payload.to_writer(&mut payload)?;
        let hashed_payload = crypto::hash(&payload);
        let new_signature = keypair.sign_decorated(&hashed_payload);
        self.signatures.push(new_signature);
        Ok(())
    }

    pub fn hash(&self) -> Result<Vec<u8>> {
        let payload = self.signature_base()?;
        Ok(crypto::hash(&payload))
    }

    pub fn signature_base(&self) -> Result<Vec<u8>> {
        let sig_payload = TransactionSignaturePayload {
            network_id: &self.network_id,
            transaction: &self.transaction,
        };
        let mut payload = Vec::new();
        sig_payload.to_writer(&mut payload)?;
        Ok(payload)
    }

    pub fn transaction(&self) -> &Transaction {
        &self.transaction
    }

    pub fn signatures(&self) -> &Vec<DecoratedSignature> {
        &self.signatures
    }
}

#[derive(Debug)]
pub struct TransactionSignaturePayload<'a> {
    pub network_id: &'a Vec<u8>,
    pub transaction: &'a Transaction,
}
