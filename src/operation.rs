use amount::Amount;
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

/// Add data entry to the ledger.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManageDataOperation {
    /// The source account for the operation.
    pub source: Option<PublicKey>,
    /// The key of the data entry
    pub name: String,
    /// The value of the data entry. A value of `None` will delete the entry.
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    CreateAccount(CreateAccountOperation),
    Payment(PaymentOperation),
    ManageData(ManageDataOperation),
}
