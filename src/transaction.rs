use account::Account;
use amount::Stroops;
use time_bounds::TimeBounds;
use memo::Memo;
use keypair::{KeyPair, PublicKey};
use operation::Operation;

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
}
