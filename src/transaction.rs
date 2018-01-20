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
        sig.sign(keypair);
        Ok(sig)
    }

    pub fn signature_base(self, network: &Network) -> Result<Vec<u8>> {
        let payload = TransactionSignaturePayload::new(&network, self);
        let mut out = Vec::new();
        payload.to_writer(&mut out)?;
        Ok(crypto::hash(&out))
    }
}

#[derive(Debug, Clone)]
pub struct SignedTransaction {
    payload: Vec<u8>,
    transaction: Transaction,
    signatures: Vec<DecoratedSignature>,
}

impl SignedTransaction {
    pub fn new(transaction: Transaction, network: &Network) -> Result<SignedTransaction> {
        let payload = transaction.clone().signature_base(network)?;
        Ok(SignedTransaction {
            payload,
            transaction,
            signatures: Vec::new(),
        })
    }

    pub fn sign(&mut self, keypair: &KeyPair) {
        let new_signature = keypair.sign_decorated(&self.payload);
        self.signatures.push(new_signature);
    }

    pub fn signatures(&self) -> &Vec<DecoratedSignature> {
        &self.signatures
    }
}

#[derive(Debug)]
pub struct TransactionSignaturePayload {
    pub network_id: Vec<u8>,
    pub transaction: Transaction,
}

impl TransactionSignaturePayload {
    pub fn new(network: &Network, transaction: Transaction) -> Self {
        let network_id = network.network_id();
        TransactionSignaturePayload {
            network_id,
            transaction,
        }
    }
}
