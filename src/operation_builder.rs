use operation::{CreateAccountOperation, InflationOperation, ManageDataOperation, Operation,
                PaymentOperation};
use keypair::PublicKey;
use asset::Asset;
use amount::Amount;

pub struct OperationBuilder;

impl OperationBuilder {
    pub fn inflation() -> InflationOperationBuilder {
        InflationOperationBuilder::new()
    }

    pub fn create_account(
        destination: PublicKey,
        balance: Amount,
    ) -> CreateAccountOperationBuilder {
        CreateAccountOperationBuilder::new(destination, balance)
    }

    pub fn payment(
        destination: PublicKey,
        asset: Asset,
        amount: Amount,
    ) -> PaymentOperationBuilder {
        PaymentOperationBuilder::new(destination, asset, amount)
    }

    pub fn set_data(name: String, value: Vec<u8>) -> ManageDataOperationBuilder {
        ManageDataOperationBuilder::set_data(name, value)
    }

    pub fn delete_data(name: String) -> ManageDataOperationBuilder {
        ManageDataOperationBuilder::delete_data(name)
    }
}

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

    pub fn build(self) -> Operation {
        Operation::CreateAccount(self.inner)
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

    pub fn build(self) -> Operation {
        Operation::Payment(self.inner)
    }
}

#[derive(Debug, Clone)]
pub struct ManageDataOperationBuilder {
    inner: ManageDataOperation,
}

impl ManageDataOperationBuilder {
    pub fn set_data(name: String, value: Vec<u8>) -> Self {
        let inner = ManageDataOperation {
            source: None,
            name,
            value: Some(value),
        };
        ManageDataOperationBuilder { inner }
    }

    pub fn delete_data(name: String) -> Self {
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

    pub fn build(self) -> Operation {
        Operation::ManageData(self.inner)
    }
}

#[derive(Debug, Clone)]
pub struct InflationOperationBuilder {
    inner: InflationOperation,
}

impl InflationOperationBuilder {
    pub fn new() -> Self {
        let inner = InflationOperation { source: None };
        InflationOperationBuilder { inner }
    }

    pub fn with_source(mut self, source: PublicKey) -> Self {
        self.inner.source = Some(source);
        self
    }

    pub fn build(self) -> Operation {
        Operation::Inflation(self.inner)
    }
}
