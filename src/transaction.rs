use account::Account;
use amount::Stroops;
use time_bounds::TimeBounds;
use memo::Memo;
use keypair::PublicKey;
use operation::Operation;

#[derive(Debug, Clone)]
pub struct Transaction {
    source: Account,
    fee: Stroops,
    time_bounds: Option<TimeBounds>,
    memo: Memo,
    operations: Vec<Operation>,
}

impl Transaction {
    pub fn new(
        source: Account,
        fee: Stroops,
        time_bounds: Option<TimeBounds>,
        memo: Memo,
        operations: Vec<Operation>,
    ) -> Transaction {
        Transaction {
            source,
            fee,
            time_bounds,
            memo,
            operations,
        }
    }

    pub fn source(&self) -> &Account {
        &self.source
    }

    pub fn source_public_key(&self) -> &PublicKey {
        self.source.public_key()
    }

    pub fn source_sequence(&self) -> u64 {
        self.source.sequence()
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
}
