use operation::{CreateAccountOperation, CreatePassiveOfferOperation, InflationOperation,
                ManageDataOperation, ManageOfferOperation, Operation, PathPaymentOperation,
                PaymentOperation};
use keypair::PublicKey;
use asset::Asset;
use amount::{Amount, Price};

/// Build an [`Operation`](enum.Operation.html).
#[derive(Debug)]
pub struct OperationBuilder;

impl OperationBuilder {
    /// Build an [`InflationOperation`](struct.InflationOperation.html).
    pub fn inflation() -> InflationOperationBuilder {
        InflationOperationBuilder::new()
    }

    /// Build a [`CreateAccountOperation`](struct.CreateAccountOperation.html) with
    /// `destination` address and starting `balance`.
    pub fn create_account(
        destination: PublicKey,
        balance: Amount,
    ) -> CreateAccountOperationBuilder {
        CreateAccountOperationBuilder::new(destination, balance)
    }

    /// Build a [`PaymentOperation`](struct.PaymentOperation.html) sending `amount`
    /// units of the `asset` to the `destination` account.
    pub fn payment(
        destination: PublicKey,
        asset: Asset,
        amount: Amount,
    ) -> PaymentOperationBuilder {
        PaymentOperationBuilder::new(destination, asset, amount)
    }

    /// Build a [`PathPaymentOperation`](struct.PathPaymentOperation.html).
    pub fn path_payment(
        destination: PublicKey,
        send_asset: Asset,
        send_max: Amount,
        dest_asset: Asset,
        dest_amount: Amount,
    ) -> PathPaymentOperationBuilder {
        PathPaymentOperationBuilder::new(destination, send_asset, send_max, dest_asset, dest_amount)
    }

    /// Build a [`ManageOfferOperation`](struct.ManageOfferOperation.html).
    pub fn manage_offer(
        selling: Asset,
        buying: Asset,
        amount: Amount,
        price: Price,
    ) -> ManageOfferOperationBuilder {
        ManageOfferOperationBuilder::new(selling, buying, amount, price)
    }

    /// Build a [`CreatePassiveOfferOperation`](struct.CreatePassiveOfferOperation.html).
    pub fn create_passive_offer(
        selling: Asset,
        buying: Asset,
        amount: Amount,
        price: Price,
    ) -> CreatePassiveOfferOperationBuilder {
        CreatePassiveOfferOperationBuilder::new(selling, buying, amount, price)
    }

    /// Build a [`ManageDataOperation`](struct.ManageDataOperation.html) setting the key `name` to `value`.
    pub fn set_data(name: String, value: Vec<u8>) -> ManageDataOperationBuilder {
        ManageDataOperationBuilder::set_data(name, value)
    }

    /// Build a [`ManageDataOperation`](struct.ManageDataOperation.html) removing key `name`.
    pub fn delete_data(name: String) -> ManageDataOperationBuilder {
        ManageDataOperationBuilder::delete_data(name)
    }
}

/// `CreateAccountOperation` builder.
#[derive(Debug, Clone)]
pub struct CreateAccountOperationBuilder {
    inner: CreateAccountOperation,
}

impl CreateAccountOperationBuilder {
    /// Create with `destination` address and starting `balance`.
    pub fn new(destination: PublicKey, balance: Amount) -> Self {
        let inner = CreateAccountOperation {
            source: None,
            destination,
            balance,
        };
        CreateAccountOperationBuilder { inner }
    }

    /// Set the operation `source`.
    pub fn with_source(mut self, source: PublicKey) -> Self {
        self.inner.source = Some(source);
        self
    }

    /// Return the `Operation`.
    pub fn build(self) -> Operation {
        Operation::CreateAccount(self.inner)
    }
}

/// `PaymentOperation` builder.
#[derive(Debug, Clone)]
pub struct PaymentOperationBuilder {
    inner: PaymentOperation,
}

impl PaymentOperationBuilder {
    /// Create payment of `amount` units of `asset` to `destination` address.
    pub fn new(destination: PublicKey, asset: Asset, amount: Amount) -> Self {
        let inner = PaymentOperation {
            source: None,
            destination,
            asset,
            amount,
        };
        PaymentOperationBuilder { inner }
    }

    /// Set the operation `source`.
    pub fn with_source(mut self, source: PublicKey) -> Self {
        self.inner.source = Some(source);
        self
    }

    /// Return the `Operation`.
    pub fn build(self) -> Operation {
        Operation::Payment(self.inner)
    }
}

/// `PathPaymentOperation` builder.
#[derive(Debug, Clone)]
pub struct PathPaymentOperationBuilder {
    inner: PathPaymentOperation,
}

impl PathPaymentOperationBuilder {
    /// TODO
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

    /// Set the operation `source`.
    pub fn with_source(mut self, source: PublicKey) -> Self {
        self.inner.source = Some(source);
        self
    }

    /// Set the payment path.
    pub fn with_path(mut self, path: Vec<Asset>) -> Self {
        self.inner.path = path;
        self
    }

    /// Push `asset` to the payment path.
    pub fn push_asset(mut self, asset: Asset) -> Self {
        self.inner.path.push(asset);
        self
    }

    /// Return the `Operation`.
    pub fn build(self) -> Operation {
        Operation::PathPayment(self.inner)
    }
}

/// `ManageOfferOperation` builder.
#[derive(Debug, Clone)]
pub struct ManageOfferOperationBuilder {
    inner: ManageOfferOperation,
}

impl ManageOfferOperationBuilder {
    /// TODO
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

    /// Set the operation `source`.
    pub fn with_source(mut self, source: PublicKey) -> Self {
        self.inner.source = Some(source);
        self
    }

    /// Set the offer `id`.
    pub fn with_offer_id(mut self, id: u64) -> Self {
        self.inner.offer_id = id;
        self
    }

    /// Return the `Operation`.
    pub fn build(self) -> Operation {
        Operation::ManageOffer(self.inner)
    }
}

/// `CreatePassiveOfferOperation` builder.
#[derive(Debug, Clone)]
pub struct CreatePassiveOfferOperationBuilder {
    inner: CreatePassiveOfferOperation,
}

impl CreatePassiveOfferOperationBuilder {
    /// TODO
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

    /// Set the operation `source`.
    pub fn with_source(mut self, source: PublicKey) -> Self {
        self.inner.source = Some(source);
        self
    }

    /// Return the `Operation`.
    pub fn build(self) -> Operation {
        Operation::CreatePassiveOffer(self.inner)
    }
}

/// `ManageDataOperation` build
#[derive(Debug, Clone)]
pub struct ManageDataOperationBuilder {
    inner: ManageDataOperation,
}

impl ManageDataOperationBuilder {
    /// Create a new operation to set account data `name` to `value`.
    pub fn set_data(name: String, value: Vec<u8>) -> Self {
        let inner = ManageDataOperation {
            source: None,
            name,
            value: Some(value),
        };
        ManageDataOperationBuilder { inner }
    }

    /// Create a new operation to delete `name` from the account data.
    pub fn delete_data(name: String) -> Self {
        let inner = ManageDataOperation {
            source: None,
            name,
            value: None,
        };
        ManageDataOperationBuilder { inner }
    }

    /// Set the operation `source`.
    pub fn with_source(mut self, source: PublicKey) -> Self {
        self.inner.source = Some(source);
        self
    }

    /// Return the `Operation`.
    pub fn build(self) -> Operation {
        Operation::ManageData(self.inner)
    }
}

/// `InflationOperation` builder.
#[derive(Debug, Clone)]
pub struct InflationOperationBuilder {
    inner: InflationOperation,
}

impl InflationOperationBuilder {
    /// Create a new inflation operation.
    pub fn new() -> Self {
        let inner = InflationOperation { source: None };
        InflationOperationBuilder { inner }
    }

    /// Set the operation `source`.
    pub fn with_source(mut self, source: PublicKey) -> Self {
        self.inner.source = Some(source);
        self
    }

    /// Return the `Operation`.
    pub fn build(self) -> Operation {
        Operation::Inflation(self.inner)
    }
}
