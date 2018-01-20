use amount::{Amount, Price};
use asset::Asset;
use keypair::PublicKey;

/// Create and fund a new account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateAccountOperation {
    /// The source account for the operation.
    pub source: Option<PublicKey>,
    /// New account id.
    pub destination: PublicKey,
    /// Amount (in XLM) the account should be funded for. Must be greater than
    /// the [reserve balance amount](https://www.stellar.org/developers/guides/concepts/fees.html)
    pub balance: Amount,
}

/// Payment operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentOperation {
    /// The source account for the operation.
    pub source: Option<PublicKey>,
    /// The destination account id.
    pub destination: PublicKey,
    /// The asset to send.
    pub asset: Asset,
    /// The amount to send.
    pub amount: Amount,
}

/// Send the specified asset to the destination account, optionally through a path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathPaymentOperation {
    /// The source account for the operation.
    pub source: Option<PublicKey>,
    /// The destination account id.
    pub destination: PublicKey,
    /// The asset to pay with.
    pub send_asset: Asset,
    /// The maximum amount of send_asset to send.
    pub send_max: Amount,
    /// The asset the destination will receive.
    pub dest_asset: Asset,
    /// The amount the destination receives.
    pub dest_amount: Amount,
    /// The assets path.
    pub path: Vec<Asset>,
}

/// Create, update, or delete an offer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManageOfferOperation {
    /// The source account for the operation.
    pub source: Option<PublicKey>,
    /// What you're selling.
    pub selling: Asset,
    /// What you're buying.
    pub buying: Asset,
    /// The total amount you're selling. If 0, deletes the offer.
    pub amount: Amount,
    /// The exchange rate ratio.
    pub price: Price,
    /// Offer id. If 0, creates a new offer.
    pub offer_id: u64,
}

/// Create a passive offer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatePassiveOfferOperation {
    /// The source account for the operation.
    pub source: Option<PublicKey>,
    /// What you're selling.
    pub selling: Asset,
    /// What you're buying.
    pub buying: Asset,
    /// The total amount you're selling. If 0, deletes the offer.
    pub amount: Amount,
    /// The exchange rate ratio.
    pub price: Price,
}

/// Add data entry to the ledger.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManageDataOperation {
    /// The source account for the operation.
    pub source: Option<PublicKey>,
    /// The key of the data entry
    pub name: String,
    /// The value of the data entry. A value of `None` will delete the entry.
    pub value: Option<Vec<u8>>,
}

/// Generate inflation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InflationOperation {
    /// The source account for the operation.
    pub source: Option<PublicKey>,
}

/// An operation that mutates the ledger.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    /// Create new account.
    CreateAccount(CreateAccountOperation),
    /// Send payment.
    Payment(PaymentOperation),
    /// Send specified payment to account, optionally through path.
    PathPayment(PathPaymentOperation),
    /// Create, update, and delete offer.
    ManageOffer(ManageOfferOperation),
    /// Create an offer that won't consume a counter offer.
    CreatePassiveOffer(CreatePassiveOfferOperation),
    /// Set or clear account flags.
    SetOptions,
    /// Add, update, or remove a trust line.
    ChangeTrust,
    /// Allow another account to hold the account credit for an asset.
    AllowTrust,
    /// Transfer balance to destination account.
    AccountMerge,
    /// Generate inflation.
    Inflation(InflationOperation),
    /// Add, update, or remove account data.
    ManageData(ManageDataOperation),
}
