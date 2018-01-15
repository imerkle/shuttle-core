use operation::{CreateAccountOperation, CreatePassiveOfferOperation, InflationOperation,
                ManageDataOperation, ManageOfferOperation, Operation, PathPaymentOperation,
                PaymentOperation};
use keypair::PublicKey;
use asset::Asset;
use amount::{Amount, Price};

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

    pub fn path_payment(
        destination: PublicKey,
        send_asset: Asset,
        send_max: Amount,
        dest_asset: Asset,
        dest_amount: Amount,
    ) -> PathPaymentOperationBuilder {
        PathPaymentOperationBuilder::new(destination, send_asset, send_max, dest_asset, dest_amount)
    }

    pub fn manage_offer(
        selling: Asset,
        buying: Asset,
        amount: Amount,
        price: Price,
    ) -> ManageOfferOperationBuilder {
        ManageOfferOperationBuilder::new(selling, buying, amount, price)
    }

    pub fn create_passive_offer(
        selling: Asset,
        buying: Asset,
        amount: Amount,
        price: Price,
    ) -> CreatePassiveOfferOperationBuilder {
        CreatePassiveOfferOperationBuilder::new(selling, buying, amount, price)
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
pub struct PathPaymentOperationBuilder {
    inner: PathPaymentOperation,
}

impl PathPaymentOperationBuilder {
    pub fn new(
        destination: PublicKey,
        send_asset: Asset,
        send_max: Amount,
        dest_asset: Asset,
        dest_amount: Amount,
    ) -> PathPaymentOperationBuilder {
        let inner = PathPaymentOperation {
            source: None,
            destination,
            send_asset,
            send_max,
            dest_asset,
            dest_amount,
            path: Vec::new(),
        };
        PathPaymentOperationBuilder { inner }
    }

    pub fn with_source(mut self, source: PublicKey) -> Self {
        self.inner.source = Some(source);
        self
    }

    pub fn with_path(mut self, path: Vec<Asset>) -> Self {
        self.inner.path = path;
        self
    }

    pub fn push_asset(mut self, asset: Asset) -> Self {
        self.inner.path.push(asset);
        self
    }

    pub fn build(self) -> Operation {
        Operation::PathPayment(self.inner)
    }
}

#[derive(Debug, Clone)]
pub struct ManageOfferOperationBuilder {
    inner: ManageOfferOperation,
}

impl ManageOfferOperationBuilder {
    pub fn new(
        selling: Asset,
        buying: Asset,
        amount: Amount,
        price: Price,
    ) -> ManageOfferOperationBuilder {
        let inner = ManageOfferOperation {
            source: None,
            selling,
            buying,
            amount,
            price,
            offer_id: 0,
        };
        ManageOfferOperationBuilder { inner }
    }

    pub fn with_source(mut self, source: PublicKey) -> Self {
        self.inner.source = Some(source);
        self
    }

    pub fn with_offer_id(mut self, id: u64) -> Self {
        self.inner.offer_id = id;
        self
    }

    pub fn build(self) -> Operation {
        Operation::ManageOffer(self.inner)
    }
}

#[derive(Debug, Clone)]
pub struct CreatePassiveOfferOperationBuilder {
    inner: CreatePassiveOfferOperation,
}

impl CreatePassiveOfferOperationBuilder {
    pub fn new(
        selling: Asset,
        buying: Asset,
        amount: Amount,
        price: Price,
    ) -> CreatePassiveOfferOperationBuilder {
        let inner = CreatePassiveOfferOperation {
            source: None,
            selling,
            buying,
            amount,
            price,
        };
        CreatePassiveOfferOperationBuilder { inner }
    }

    pub fn with_source(mut self, source: PublicKey) -> Self {
        self.inner.source = Some(source);
        self
    }

    pub fn build(self) -> Operation {
        Operation::CreatePassiveOffer(self.inner)
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
