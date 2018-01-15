use std::convert::From;
use try_from::{TryFrom, TryInto};
use std::result;
use amount::{Amount, Price, Stroops};
use error::{Error, Result};
use operation;
use xdr::{Asset, PublicKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub source: Option<PublicKey>,
    pub inner: OperationInner,
}

impl Operation {
    pub fn new(source: Option<PublicKey>, inner: OperationInner) -> Operation {
        Operation { source, inner }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationInner {
    CreateAccount(CreateAccountOperation),
    Payment(PaymentOperation),
    PathPayment(PathPaymentOperation),
    ManageOffer(ManageOfferOperation),
    CreatePassiveOffer(CreatePassiveOfferOperation),
    SetOptions,
    ChangeTrust,
    AllowTrust,
    AccountMerge,
    Inflation,
    ManageData(ManageDataOperation),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccountOperation {
    destination: PublicKey,
    balance: Stroops,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentOperation {
    destination: PublicKey,
    asset: Asset,
    amount: Stroops,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathPaymentOperation {
    send_asset: Asset,
    send_max: Stroops,
    destination: PublicKey,
    dest_asset: Asset,
    dest_amount: Stroops,
    path: Vec<Asset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManageOfferOperation {
    selling: Asset,
    buying: Asset,
    amount: Stroops,
    price: Price,
    offer_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePassiveOfferOperation {
    selling: Asset,
    buying: Asset,
    amount: Stroops,
    price: Price,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManageDataOperation {
    name: String,
    value: Option<Vec<u8>>,
}

fn from_create_account(create: ::CreateAccountOperation) -> Operation {
    let source = match create.source {
        None => None,
        Some(pk) => Some(PublicKey::from(pk)),
    };
    let destination = PublicKey::from(create.destination);
    let balance = create.balance.into_stroops().unwrap();
    let inner = OperationInner::CreateAccount(CreateAccountOperation {
        destination,
        balance,
    });
    Operation::new(source, inner)
}

fn from_payment(payment: ::PaymentOperation) -> Operation {
    let source = match payment.source {
        None => None,
        Some(pk) => Some(PublicKey::from(pk)),
    };
    let destination = PublicKey::from(payment.destination);
    let asset = Asset::from(payment.asset);
    let amount = payment.amount.into_stroops().unwrap();
    let inner = OperationInner::Payment(PaymentOperation {
        destination,
        asset,
        amount,
    });
    Operation::new(source, inner)
}

fn from_path_payment(payment: ::PathPaymentOperation) -> Operation {
    let source = match payment.source {
        None => None,
        Some(pk) => Some(PublicKey::from(pk)),
    };

    let destination = PublicKey::from(payment.destination);
    let send_asset = Asset::from(payment.send_asset);
    let send_max = payment.send_max.into_stroops().unwrap();
    let dest_asset = Asset::from(payment.dest_asset);
    let dest_amount = payment.dest_amount.into_stroops().unwrap();
    let path = payment
        .path
        .into_iter()
        .map(|p| Asset::from(p))
        .collect::<Vec<_>>();

    let inner = OperationInner::PathPayment(PathPaymentOperation {
        destination,
        send_asset,
        send_max,
        dest_asset,
        dest_amount,
        path,
    });
    Operation::new(source, inner)
}

fn from_manage_offer(manage: ::ManageOfferOperation) -> Operation {
    let source = match manage.source {
        None => None,
        Some(pk) => Some(PublicKey::from(pk)),
    };
    let selling = Asset::from(manage.selling);
    let buying = Asset::from(manage.buying);
    let amount = manage.amount.into_stroops().unwrap();
    let price = manage.price;
    let offer_id = manage.offer_id;
    let inner = OperationInner::ManageOffer(ManageOfferOperation {
        selling,
        buying,
        amount,
        price,
        offer_id,
    });
    Operation::new(source, inner)
}

fn from_create_passive_offer(manage: ::CreatePassiveOfferOperation) -> Operation {
    let source = match manage.source {
        None => None,
        Some(pk) => Some(PublicKey::from(pk)),
    };
    let selling = Asset::from(manage.selling);
    let buying = Asset::from(manage.buying);
    let amount = manage.amount.into_stroops().unwrap();
    let price = manage.price;
    let inner = OperationInner::CreatePassiveOffer(CreatePassiveOfferOperation {
        selling,
        buying,
        amount,
        price,
    });
    Operation::new(source, inner)
}

fn from_manage_data(manage: ::ManageDataOperation) -> Operation {
    let source = match manage.source {
        None => None,
        Some(pk) => Some(PublicKey::from(pk)),
    };
    let inner = OperationInner::ManageData(ManageDataOperation {
        name: manage.name,
        value: manage.value,
    });
    Operation::new(source, inner)
}

fn from_inflation(inflation: ::InflationOperation) -> Operation {
    let source = match inflation.source {
        None => None,
        Some(pk) => Some(PublicKey::from(pk)),
    };
    let inner = OperationInner::Inflation;
    Operation::new(source, inner)
}

impl From<::Operation> for Operation {
    fn from(op: ::Operation) -> Self {
        match op {
            ::Operation::CreateAccount(op) => from_create_account(op),
            ::Operation::Payment(op) => from_payment(op),
            ::Operation::PathPayment(op) => from_path_payment(op),
            ::Operation::ManageOffer(op) => from_manage_offer(op),
            ::Operation::CreatePassiveOffer(op) => from_create_passive_offer(op),
            ::Operation::ManageData(op) => from_manage_data(op),
            ::Operation::Inflation(op) => from_inflation(op),
            _ => unimplemented!(),
        }
    }
}

fn to_create_account(
    source: Option<::PublicKey>,
    inner: CreateAccountOperation,
) -> Result<::Operation> {
    let destination = inner.destination.try_into()?;
    let balance = Amount::from_stroops(inner.balance)?;
    Ok(::Operation::CreateAccount(
        operation::CreateAccountOperation {
            source,
            destination,
            balance,
        },
    ))
}

fn to_payment(source: Option<::PublicKey>, inner: PaymentOperation) -> Result<::Operation> {
    let destination = inner.destination.try_into()?;
    let asset = inner.asset.try_into()?;
    let amount = Amount::from_stroops(inner.amount)?;
    Ok(::Operation::Payment(operation::PaymentOperation {
        source,
        destination,
        asset,
        amount,
    }))
}

fn to_path_payment(
    source: Option<::PublicKey>,
    inner: PathPaymentOperation,
) -> Result<::Operation> {
    let destination = inner.destination.try_into()?;
    let send_asset = inner.send_asset.try_into()?;
    let send_max = Amount::from_stroops(inner.send_max)?;
    let dest_asset = inner.dest_asset.try_into()?;
    let dest_amount = Amount::from_stroops(inner.dest_amount)?;
    let path_res: Result<Vec<_>> = inner.path.into_iter().map(|p| p.try_into()).collect();
    let path = path_res?;

    Ok(::Operation::PathPayment(operation::PathPaymentOperation {
        source,
        destination,
        send_asset,
        send_max,
        dest_asset,
        dest_amount,
        path,
    }))
}

fn to_manage_offer(
    source: Option<::PublicKey>,
    inner: ManageOfferOperation,
) -> Result<::Operation> {
    let selling = inner.selling.try_into()?;
    let buying = inner.buying.try_into()?;
    let amount = Amount::from_stroops(inner.amount)?;
    let price = inner.price;
    let offer_id = inner.offer_id;
    Ok(::Operation::ManageOffer(operation::ManageOfferOperation {
        source,
        selling,
        buying,
        amount,
        price,
        offer_id,
    }))
}

fn to_create_passive_offer(
    source: Option<::PublicKey>,
    inner: CreatePassiveOfferOperation,
) -> Result<::Operation> {
    let selling = inner.selling.try_into()?;
    let buying = inner.buying.try_into()?;
    let amount = Amount::from_stroops(inner.amount)?;
    let price = inner.price;
    Ok(::Operation::CreatePassiveOffer(
        operation::CreatePassiveOfferOperation {
            source,
            selling,
            buying,
            amount,
            price,
        },
    ))
}

fn to_manage_data(source: Option<::PublicKey>, inner: ManageDataOperation) -> Result<::Operation> {
    Ok(::Operation::ManageData(operation::ManageDataOperation {
        source,
        name: inner.name,
        value: inner.value,
    }))
}

fn to_inflation(source: Option<::PublicKey>) -> Result<::Operation> {
    Ok(::Operation::Inflation(operation::InflationOperation {
        source,
    }))
}

impl TryFrom<Operation> for ::Operation {
    type Err = Error;

    fn try_from(op: Operation) -> result::Result<Self, Error> {
        let source = match op.source {
            None => None,
            Some(pk) => Some(pk.try_into()?),
        };
        match op.inner {
            OperationInner::CreateAccount(inner) => to_create_account(source, inner),
            OperationInner::Payment(inner) => to_payment(source, inner),
            OperationInner::PathPayment(inner) => to_path_payment(source, inner),
            OperationInner::ManageOffer(inner) => to_manage_offer(source, inner),
            OperationInner::CreatePassiveOffer(inner) => to_create_passive_offer(source, inner),
            OperationInner::ManageData(inner) => to_manage_data(source, inner),
            OperationInner::Inflation => to_inflation(source),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use try_from::TryInto;
    use {InflationOperationBuilder, Operation, OperationBuilder};
    use {Amount, Asset, Price, PublicKey};
    use xdr;

    fn do_it(op: Operation, expected: &str) {
        let xdr_op = xdr::Operation::from(op.clone());
        let encoded = xdr::to_base64(&xdr_op).unwrap();
        assert_eq!(encoded, expected);
        let decoded = xdr::from_base64::<xdr::Operation>(&encoded).unwrap();
        let op_back: Operation = decoded.try_into().unwrap();
        assert_eq!(op_back, op);
    }

    #[test]
    fn test_inflation() {
        let op = OperationBuilder::inflation().build();
        do_it(op, "AAAAAAAAAAk=");
    }

    #[test]
    fn test_create_account() {
        let balance = Amount::from_str("20.0").unwrap();
        let dest = PublicKey::from_account_id(
            "GCLDNMHZTEY6PUYQBYOVERBBZ2W3RLMYOSZWHAMY5R4YW2N6MM4LFA72",
        ).unwrap();
        let op = OperationBuilder::create_account(dest, balance).build();
        do_it(
            op,
            "AAAAAAAAAAAAAAAAljaw+Zkx59MQDh1SRCHOrbitmHSzY4GY7HmLab5jOLIAAAAAC+vCAA==",
        );
    }

    #[test]
    fn test_payment() {
        let dest = PublicKey::from_account_id(
            "GCLDNMHZTEY6PUYQBYOVERBBZ2W3RLMYOSZWHAMY5R4YW2N6MM4LFA72",
        ).unwrap();
        let asset = Asset::credit("ABCD".to_string(), dest.clone()).unwrap();
        let amount = Amount::from_str("100.123").unwrap();
        let op = OperationBuilder::payment(dest, asset, amount).build();
        do_it(
            op,
            "AAAAAAAAAAEAAAAAljaw+Zkx59MQDh1SRCHOrbitmHSzY4GY7HmLab5jOLIAAAABQUJDRAAAAACWNrD5mTHn0xAOHVJEIc6tuK2YdLNjgZjseYtpvmM4sgAAAAA7rY6w"
        );
    }

    #[test]
    fn test_path_payment() {
        let dest = PublicKey::from_account_id(
            "GCLDNMHZTEY6PUYQBYOVERBBZ2W3RLMYOSZWHAMY5R4YW2N6MM4LFA72",
        ).unwrap();

        let send_asset = Asset::native();
        let send_max = Amount::from_str("100.123").unwrap();
        let dest_asset = Asset::credit("ABCD".to_string(), dest.clone()).unwrap();
        let dest_amount = Amount::from_str("2.123").unwrap();

        let int_asset = Asset::credit("XXXYYYZZZ".to_string(), dest.clone()).unwrap();

        let op =
            OperationBuilder::path_payment(dest, send_asset, send_max, dest_asset, dest_amount)
                .push_asset(int_asset)
                .build();
        do_it(op, "AAAAAAAAAAIAAAAAAAAAADutjrAAAAAAljaw+Zkx59MQDh1SRCHOrbitmHSzY4GY7HmLab5jOLIAAAABQUJDRAAAAACWNrD5mTHn0xAOHVJEIc6tuK2YdLNjgZjseYtpvmM4sgAAAAABQ/GwAAAAAQAAAAJYWFhZWVlaWloAAAAAAAAAljaw+Zkx59MQDh1SRCHOrbitmHSzY4GY7HmLab5jOLI=");
    }

    #[test]
    fn test_manage_offer() {
        let issuer = PublicKey::from_account_id(
            "GCLDNMHZTEY6PUYQBYOVERBBZ2W3RLMYOSZWHAMY5R4YW2N6MM4LFA72",
        ).unwrap();

        let selling = Asset::native();
        let buying = Asset::credit("ABCD".to_string(), issuer).unwrap();
        let amount = Amount::from_str("100.123").unwrap();
        let price = Price::new(100, 3);
        let op = OperationBuilder::manage_offer(selling, buying, amount, price)
            .with_offer_id(8)
            .build();
        do_it(op, "AAAAAAAAAAMAAAAAAAAAAUFCQ0QAAAAAljaw+Zkx59MQDh1SRCHOrbitmHSzY4GY7HmLab5jOLIAAAAAO62OsAAAAGQAAAADAAAAAAAAAAg=");
    }

    #[test]
    fn test_create_passive_offer() {
        let issuer = PublicKey::from_account_id(
            "GCLDNMHZTEY6PUYQBYOVERBBZ2W3RLMYOSZWHAMY5R4YW2N6MM4LFA72",
        ).unwrap();

        let selling = Asset::native();
        let buying = Asset::credit("ABCD".to_string(), issuer).unwrap();
        let amount = Amount::from_str("100.123").unwrap();
        let price = Price::new(100, 3);
        let op = OperationBuilder::create_passive_offer(selling, buying, amount, price).build();
        do_it(op, "AAAAAAAAAAQAAAAAAAAAAUFCQ0QAAAAAljaw+Zkx59MQDh1SRCHOrbitmHSzY4GY7HmLab5jOLIAAAAAO62OsAAAAGQAAAAD");
    }

    #[test]
    fn test_manage_data() {
        let delete_op = OperationBuilder::delete_data("THE KEY".to_string()).build();
        do_it(delete_op, "AAAAAAAAAAoAAAAHVEhFIEtFWQAAAAAA");
        // TODO(fracek) Figure out serialization of var length opaque types
        //let set_op = OperationBuilder::set_data("THE KEY".to_string(), vec![1, 2, 3]).build();
        //do_it(set_op, "AAAAAAAAAAoAAAAHVEhFIEtFWQAAAAABAAAAAwECAwA=");
    }
}
