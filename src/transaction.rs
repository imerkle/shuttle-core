use amount::Stroops;
use time_bounds::TimeBounds;
use memo::Memo;
use network::Network;
use keypair::KeyPair;
use signature::DecoratedSignature;
use operation::Operation;
use crypto;

const BASE_FEE: Stroops = Stroops(100);

#[derive(Debug, Clone)]
pub struct Transaction {
    source: KeyPair,
    sequence: u64,
    fee: Stroops,
    time_bounds: Option<TimeBounds>,
    memo: Memo,
    operations: Vec<Operation>,
}

impl Transaction {
    pub fn new(
        source: KeyPair,
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

    pub fn sign(self, keypair: &KeyPair, network: &Network) -> SignedTransaction {
        let signature_base = self.signature_base(&network);
        let payload = crypto::hash(&signature_base);
        let decorated_signature = keypair.sign_decorated(&payload);
        SignedTransaction::new(payload, decorated_signature)
    }

    pub fn signature_base(self, _network: &Network) -> Vec<u8> {
        unimplemented!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedTransaction {
    payload: Vec<u8>,
    signatures: Vec<DecoratedSignature>,
}

impl SignedTransaction {
    pub fn new(payload: Vec<u8>, signature: DecoratedSignature) -> SignedTransaction {
        SignedTransaction {
            payload,
            signatures: vec![signature],
        }
    }

    pub fn sign(&mut self, keypair: &KeyPair) {
        let new_signature = keypair.sign_decorated(&self.payload);
        self.signatures.push(new_signature);
    }

    pub fn signatures(&self) -> &Vec<DecoratedSignature> {
        &self.signatures
    }
}
