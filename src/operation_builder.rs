use operation::{CreateAccountOperation, ManageDataOperation, PaymentOperation};
use keypair::PublicKey;
use asset::Asset;
use amount::Amount;

#[derive(Debug, Clone)]
pub struct CreateAccountOperationBuilder {
    inner: CreateAccountOperation,
}

impl CreateAccountOperationBuilder {
    pub fn new(destination: PublicKey, balance: Amount) -> Self {
        let inner = CreateAccountOperation {
            source: None,
            destination,
            balance,
        };
        CreateAccountOperationBuilder { inner }
    }

    pub fn with_source(mut self, source: PublicKey) -> Self {
        self.inner.source = Some(source);
        self
    }

    pub fn build(self) -> CreateAccountOperation {
        self.inner
    }
}

#[derive(Debug, Clone)]
pub struct PaymentOperationBuilder {
    inner: PaymentOperation,
}

impl PaymentOperationBuilder {
    pub fn new(destination: PublicKey, asset: Asset, amount: Amount) -> Self {
        let inner = PaymentOperation {
            source: None,
            destination,
            asset,
            amount,
        };
        PaymentOperationBuilder { inner }
    }

    pub fn with_source(mut self, source: PublicKey) -> Self {
        self.inner.source = Some(source);
        self
    }

    pub fn build(self) -> PaymentOperation {
        self.inner
    }
}

#[derive(Debug, Clone)]
pub struct ManageDataOperationBuilder {
    inner: ManageDataOperation,
}

impl ManageDataOperationBuilder {
    pub fn set_entry(name: String, value: String) -> Self {
        let inner = ManageDataOperation {
            source: None,
            name,
            value: Some(value),
        };
        ManageDataOperationBuilder { inner }
    }

    pub fn delete_entry(name: String) -> Self {
        let inner = ManageDataOperation {
            source: None,
            name,
            value: None,
        };
        ManageDataOperationBuilder { inner }
    }

    pub fn with_source(mut self, source: PublicKey) -> Self {
        self.inner.source = Some(source);
        self
    }

    pub fn build(self) -> ManageDataOperation {
        self.inner
    }
}
