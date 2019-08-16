use amount::Stroops;
use time_bounds::TimeBounds;
use memo::Memo;
use network::Network;
use ed25519_dalek::{Keypair, PublicKey};
use signature::DecoratedSignature;
use operation::Operation;
use error::Result;
use xdr::ToXdr;
use crypto;
use crypto::keypair::sign_decorated;
const BASE_FEE: Stroops = Stroops(100);

/// A transaction containing operations that change the ledger state.
#[derive(Debug, Clone)]
pub struct Transaction {
    /// The source account.
    pub source: PublicKey,
    /// The sequence number.
    pub sequence: u64,
    /// The fee.
    pub fee: Stroops,
    /// The validity time bounds.
    pub time_bounds: Option<TimeBounds>,
    /// The attached memo.
    pub memo: Memo,
    /// The operations.
    pub operations: Vec<Operation>,
}

impl Transaction {
    /// Create a new transaction.
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

    /// The transaction source account.
    pub fn source(&self) -> &PublicKey {
        &self.source
    }

    /// The transaction fee.
    pub fn base_fee(&self) -> &Stroops {
        &self.fee
    }

    /// The transaction time bounds for its validity.
    pub fn time_bounds(&self) -> &Option<TimeBounds> {
        &self.time_bounds
    }

    /// The memo attached to the transaction.
    pub fn memo(&self) -> &Memo {
        &self.memo
    }

    /// The transaction sequence number.
    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    /// The operations included in the transaction.
    pub fn operations(&self) -> &Vec<Operation> {
        &self.operations
    }

    /// Sign the transaction, returning a signed transaction that can be submitted to the `network`.
    pub fn sign(self, keypair: &Keypair, network: &Network) -> Result<SignedTransaction> {
        let mut sig = SignedTransaction::new(self, network)?;
        sig.sign(keypair)?;
        Ok(sig)
    }
}

/// A transaction that was signed.
#[derive(Debug, Clone)]
pub struct SignedTransaction {
    network_id: Vec<u8>,
    transaction: Transaction,
    signatures: Vec<DecoratedSignature>,
}

impl SignedTransaction {
    /// Create a new signed transaction on the `network`.
    pub fn new(transaction: Transaction, network: &Network) -> Result<SignedTransaction> {
        Ok(SignedTransaction {
            network_id: network.network_id().clone(),
            transaction,
            signatures: Vec::new(),
        })
    }

    /// Create a new transaction without the network information attached.
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

    /// Add one more signature to the transaction.
    pub fn sign(&mut self, keypair: &Keypair) -> Result<()> {
        let payload = self.hash()?;
        let new_signature = sign_decorated(keypair, &payload);
        self.signatures.push(new_signature);
        Ok(())
    }

    /// Return the transaction hash, suitable for signing.
    pub fn hash(&self) -> Result<Vec<u8>> {
        let payload = self.signature_base()?;
        Ok(crypto::hash(&payload))
    }

    /// Return the *signature base* of the transaction, which is the value
    /// that, when hashed, should be signed.
    pub fn signature_base(&self) -> Result<Vec<u8>> {
        let sig_payload = TransactionSignaturePayload {
            network_id: &self.network_id,
            transaction: &self.transaction,
        };
        let mut payload = Vec::new();
        sig_payload.to_writer(&mut payload)?;
        Ok(payload)
    }

    /// Return the transaction.
    pub fn transaction(&self) -> &Transaction {
        &self.transaction
    }

    /// Return the signatures.
    pub fn signatures(&self) -> &Vec<DecoratedSignature> {
        &self.signatures
    }
}

/// Represent the data, when signed, will be signed.
#[derive(Debug)]
pub struct TransactionSignaturePayload<'a> {
    /// The network where the transaction will be submitted to.
    pub network_id: &'a Vec<u8>,
    /// The transaction.
    pub transaction: &'a Transaction,
}
