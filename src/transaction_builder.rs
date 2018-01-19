use account::Account;
use time_bounds::TimeBounds;
use memo::Memo;
use transaction::Transaction;
use operation::Operation;

#[derive(Debug)]
pub struct TransactionBuilder<'a> {
    source: &'a mut Account,
    time_bounds: Option<TimeBounds>,
    memo: Memo,
    operations: Vec<Operation>,
}

impl<'a> TransactionBuilder<'a> {
    pub fn new(source: &'a mut Account) -> TransactionBuilder<'a> {
        TransactionBuilder {
            source: source,
            time_bounds: None,
            memo: Memo::None,
            operations: Vec::new(),
        }
    }

    pub fn with_time_bounds(mut self, time_bounds: TimeBounds) -> Self {
        self.time_bounds = Some(time_bounds);
        self
    }

    pub fn with_memo(mut self, memo: Memo) -> Self {
        self.memo = memo;
        self
    }

    pub fn push_operation(mut self, op: Operation) -> Self {
        self.operations.push(op);
        self
    }

    pub fn operations_len(&self) -> usize {
        self.operations.len()
    }

    pub fn build(self) -> Transaction {
        let keypair = self.source.keypair().clone();
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
    use KeyPair;
    use Memo;
    use TransactionBuilder;
    use OperationBuilder;

    #[test]
    fn test_builder_success() {
        let kp = KeyPair::random().unwrap();
        let mut account = Account::new(kp, 999);

        let tx0 = TransactionBuilder::new(&mut account)
            .push_operation(OperationBuilder::inflation().build())
            .build();

        let tx1 = TransactionBuilder::new(&mut account)
            .push_operation(OperationBuilder::inflation().build())
            .build();

        assert_eq!(tx0.operations().len(), 1);
        assert_eq!(tx0.sequence(), 1000);
        assert_eq!(tx1.sequence(), 1001);
    }

    #[test]
    fn test_builder_memo() {
        let kp = KeyPair::random().unwrap();
        let mut account = Account::new(kp, 999);

        let tx = TransactionBuilder::new(&mut account)
            .push_operation(OperationBuilder::inflation().build())
            .with_memo(Memo::text("TEST STRING").unwrap())
            .build();
        assert_eq!(*tx.memo(), Memo::Text("TEST STRING".to_string()));
    }
}
