#![deny(warnings)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! # shuttle-core
//!
//! The `shuttle-core` crate provides an high-level library to read, write and
//! sign the XDR structures used in the Stellar Network Protocol.
//!
//! ## KeyPair, PublicKey, and Account
//!
//! In `shuttle-core` there are three structures that represent accounts:
//!
//! - `KeyPair` contains both public and secret keys and they are used for signing
//!   transactions
//! - `PublicKey` represents an account public key, that is the addrress starting
//!   with `G`
//! - `Account` is a public key with the associated sequence number.
//!
//! ```ignore
//! let random_keypair = KeyPair::random().unwrap();
//! let keypair = KeyPair::from_secret("SDFRU2NGDPXYIY67BVS6L6W4OY33HCFCEJQ73TZZPR3IDYVVI7BVPV5Q").unwrap();
//! let public_key = keypair.public_key();
//!
//! // Create public key only
//! let address = PublicKey::from_account_id("GBR6A7TTX6MUYO6WZXZFAX3L2QSLYIHIGKN52EBNVKKB4AN4B6CRD22T").unwrap();
//!
//! // Create account
//! let account = Account::new(address, 0);
//! ```
//!
//! ## Asset and Amount
//!
//! The Stellar Network has two different types of assets: native and credit assets.
//! You can create them with `Asset::native` and `Asset::credit`.
//!
//! `Amount` represent an amount of the native asset, a.k.a. Lumens.
//!
//! ```ignore
//! let xlm = Asset::native();
//! let btc = Asset::credit("BTC", issuer_key).unwrap();
//!
//! // Parse string as amount, will error if more than 7 digits
//! let amount = Amount::from_str("123.4567").unwrap();
//! ```
//!
//!
//! ## Creating Transactions
//!
//! `shuttle-core` uses the builder pattern for transactions and operations.
//! Once you have a `SignedTransaction` you can serialize it to the base64 representation
//! of the transaction envelope and submit it to the network.
//! Alternatively, you can inspect it in the [Stellar Laboraty](https://www.stellar.org/laboratory/).
//!
//! ```ignore
//! let tx = TransactionBuilder::new(&mut source_account)
//!     .operation(
//!         OperationBuilder::payment(destination_address, asset, amount).build()
//!     )
//!     .with_memo(memo)
//!     .build();
//! let signed_tx = tx.sign(&source_keypair, &network).unwrap();
//! let encoded = signed_tx.to_base64().unwrap();
//!
//! // You can decode a transaction as well
//! let new_signed_tx = SignedTransaction::from_base64(&encode).unwrap();
//! ```
extern crate base32;
extern crate base64;
extern crate bigdecimal;
extern crate byteorder;
extern crate crc16;
extern crate num_bigint;
extern crate num_traits;
extern crate sodiumoxide;
extern crate curve25519_dalek;
extern crate ed25519_dalek;

extern crate serde;
extern crate serde_bytes;
#[macro_use]
extern crate serde_derive;
extern crate serde_xdr;

mod error;

pub mod crypto;

mod amount;
mod account;
mod asset;
mod memo;
mod network;
mod time_bounds;
mod operation;
mod operation_builder;
mod signature;
mod transaction;
mod transaction_builder;

mod xdr;

pub use self::crypto::{init};
pub use self::error::{Error, Result};
pub use self::amount::{Amount, Price, Stroops};
pub use self::account::Account;
pub use self::asset::{Asset, CreditAsset};
pub use self::memo::Memo;
pub use self::network::Network;
pub use self::time_bounds::{TimeBounds, UnixTimestamp};
pub use self::operation::{CreateAccountOperation, CreatePassiveOfferOperation, InflationOperation,
                          ManageDataOperation, ManageOfferOperation, Operation,
                          PathPaymentOperation, PaymentOperation};

pub use self::operation_builder::{CreateAccountOperationBuilder,
                                  CreatePassiveOfferOperationBuilder, InflationOperationBuilder,
                                  ManageDataOperationBuilder, ManageOfferOperationBuilder,
                                  OperationBuilder, PathPaymentOperationBuilder,
                                  PaymentOperationBuilder};
pub use self::signature::{DecoratedSignature, SignatureHint};
pub use self::transaction::{SignedTransaction, Transaction};
pub use self::transaction_builder::TransactionBuilder;

pub use self::xdr::{FromXdr, ToXdr};
