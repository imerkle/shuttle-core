use account::Account;
use time_bounds::TimeBounds;
use memo::Memo;
use transaction::Transaction;
use operation::Operation;

/// `Transaction` builder.
#[derive(Debug)]
pub struct TransactionBuilder<'a> {
    source: &'a mut Account,
    time_bounds: Option<TimeBounds>,
    memo: Memo,
    operations: Vec<Operation>,
}

impl<'a> TransactionBuilder<'a> {
    /// Create a transaction builder with `source` account.
    pub fn new(source: &'a mut Account) -> TransactionBuilder<'a> {
        TransactionBuilder {
            source: source,
            time_bounds: None,
            memo: Memo::None,
            operations: Vec::new(),
        }
    }

    /// Set the transaction time bounds.
    pub fn with_time_bounds(mut self, time_bounds: TimeBounds) -> Self {
        self.time_bounds = Some(time_bounds);
        self
    }

    /// Set the transaction memo.
    pub fn with_memo(mut self, memo: Memo) -> Self {
        self.memo = memo;
        self
    }

    /// Add one operation to the transaction.
    pub fn operation(mut self, op: Operation) -> Self {
        self.operations.push(op);
        self
    }

    /// Return the number of operations currently in the transaction.
    pub fn operations_len(&self) -> usize {
        self.operations.len()
    }

    /// Return the transaction.
    pub fn build(self) -> Transaction {
        let keypair = self.source.account_id().clone();
        let sequence = self.source.increment_sequence();
        Transaction::new(
            keypair,
            sequence,
            self.time_bounds,
            self.memo,
            self.operations,
        )
    }
}

#[cfg(test)]
mod tests {
    use Account;
    use Memo;
    use TransactionBuilder;
    use OperationBuilder;
    use ed25519_dalek::Keypair;
    use crypto;
    #[test]
    fn test_builder_success() {
        let seed = crypto::random_bytes(32);
        let kp = Keypair::from_bytes(&seed).unwrap();      
        let mut account = Account::new(kp.public, 999);

        let tx0 = TransactionBuilder::new(&mut account)
            .operation(OperationBuilder::inflation().build())
            .build();

        let tx1 = TransactionBuilder::new(&mut account)
            .operation(OperationBuilder::inflation().build())
            .build();

        assert_eq!(tx0.operations().len(), 1);
        assert_eq!(tx0.sequence(), 1000);
        assert_eq!(tx1.sequence(), 1001);
    }

    #[test]
    fn test_builder_memo() {
        let seed = crypto::random_bytes(32);
        let kp = Keypair::from_bytes(&seed).unwrap();     
        let mut account = Account::new(kp.public, 999);

        let tx = TransactionBuilder::new(&mut account)
            .operation(OperationBuilder::inflation().build())
            .with_memo(Memo::text("TEST STRING").unwrap())
            .build();
        assert_eq!(*tx.memo(), Memo::Text("TEST STRING".to_string()));
    }
}
