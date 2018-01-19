use amount::{Amount, Price, Stroops};
use error::Result;
use operation;
use xdr::{Asset, FromXdr, PublicKey, ToXdr};

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

impl ToXdr<Operation> for ::Operation {
    fn to_xdr(self) -> Result<Operation> {
        match self {
            ::Operation::CreateAccount(op) => to_create_account(op),
            ::Operation::Payment(op) => to_payment(op),
            ::Operation::PathPayment(op) => to_path_payment(op),
            ::Operation::ManageOffer(op) => to_manage_offer(op),
            ::Operation::CreatePassiveOffer(op) => to_create_passive_offer(op),
            ::Operation::ManageData(op) => to_manage_data(op),
            ::Operation::Inflation(op) => to_inflation(op),
            _ => unimplemented!(),
        }
    }
}

fn to_create_account(create: ::CreateAccountOperation) -> Result<Operation> {
    let source = match create.source {
        None => None,
        Some(pk) => Some(pk.to_xdr()?),
    };
    let destination = create.destination.to_xdr()?;
    let balance = create.balance.into_stroops()?;
    let inner = OperationInner::CreateAccount(CreateAccountOperation {
        destination,
        balance,
    });
    Ok(Operation::new(source, inner))
}

fn to_payment(payment: ::PaymentOperation) -> Result<Operation> {
    let source = match payment.source {
        None => None,
        Some(pk) => Some(pk.to_xdr()?),
    };
    let destination = payment.destination.to_xdr()?;
    let asset = payment.asset.to_xdr()?;
    let amount = payment.amount.into_stroops()?;
    let inner = OperationInner::Payment(PaymentOperation {
        destination,
        asset,
        amount,
    });
    Ok(Operation::new(source, inner))
}

fn to_path_payment(payment: ::PathPaymentOperation) -> Result<Operation> {
    let source = match payment.source {
        None => None,
        Some(pk) => Some(pk.to_xdr()?),
    };

    let destination = payment.destination.to_xdr()?;
    let send_asset = payment.send_asset.to_xdr()?;
    let send_max = payment.send_max.into_stroops()?;
    let dest_asset = payment.dest_asset.to_xdr()?;
    let dest_amount = payment.dest_amount.into_stroops()?;
    let path_res: Result<Vec<_>> = payment.path.into_iter().map(|p| p.to_xdr()).collect();
    let path = path_res?;

    let inner = OperationInner::PathPayment(PathPaymentOperation {
        destination,
        send_asset,
        send_max,
        dest_asset,
        dest_amount,
        path,
    });
    Ok(Operation::new(source, inner))
}

fn to_manage_offer(manage: ::ManageOfferOperation) -> Result<Operation> {
    let source = match manage.source {
        None => None,
        Some(pk) => Some(pk.to_xdr()?),
    };
    let selling = manage.selling.to_xdr()?;
    let buying = manage.buying.to_xdr()?;
    let amount = manage.amount.into_stroops()?;
    let price = manage.price;
    let offer_id = manage.offer_id;
    let inner = OperationInner::ManageOffer(ManageOfferOperation {
        selling,
        buying,
        amount,
        price,
        offer_id,
    });
    Ok(Operation::new(source, inner))
}

fn to_create_passive_offer(manage: ::CreatePassiveOfferOperation) -> Result<Operation> {
    let source = match manage.source {
        None => None,
        Some(pk) => Some(pk.to_xdr()?),
    };
    let selling = manage.selling.to_xdr()?;
    let buying = manage.buying.to_xdr()?;
    let amount = manage.amount.into_stroops()?;
    let price = manage.price;
    let inner = OperationInner::CreatePassiveOffer(CreatePassiveOfferOperation {
        selling,
        buying,
        amount,
        price,
    });
    Ok(Operation::new(source, inner))
}

fn to_manage_data(manage: ::ManageDataOperation) -> Result<Operation> {
    let source = match manage.source {
        None => None,
        Some(pk) => Some(pk.to_xdr()?),
    };
    let inner = OperationInner::ManageData(ManageDataOperation {
        name: manage.name,
        value: manage.value,
    });
    Ok(Operation::new(source, inner))
}

fn to_inflation(inflation: ::InflationOperation) -> Result<Operation> {
    let source = match inflation.source {
        None => None,
        Some(pk) => Some(pk.to_xdr()?),
    };
    let inner = OperationInner::Inflation;
    Ok(Operation::new(source, inner))
}

impl<'de> FromXdr<'de, Operation> for ::Operation {
    fn from_xdr(op: Operation) -> Result<::Operation> {
        let source = match op.source {
            None => None,
            Some(pk) => Some(::PublicKey::from_xdr(pk)?),
        };
        match op.inner {
            OperationInner::CreateAccount(inner) => from_create_account(source, inner),
            OperationInner::Payment(inner) => from_payment(source, inner),
            OperationInner::PathPayment(inner) => from_path_payment(source, inner),
            OperationInner::ManageOffer(inner) => from_manage_offer(source, inner),
            OperationInner::CreatePassiveOffer(inner) => from_create_passive_offer(source, inner),
            OperationInner::ManageData(inner) => from_manage_data(source, inner),
            OperationInner::Inflation => from_inflation(source),
            _ => unimplemented!(),
        }
    }
}

fn from_create_account(
    source: Option<::PublicKey>,
    inner: CreateAccountOperation,
) -> Result<::Operation> {
    let destination = ::PublicKey::from_xdr(inner.destination)?;
    let balance = Amount::from_stroops(inner.balance)?;
    Ok(::Operation::CreateAccount(
        operation::CreateAccountOperation {
            source,
            destination,
            balance,
        },
    ))
}

fn from_payment(source: Option<::PublicKey>, inner: PaymentOperation) -> Result<::Operation> {
    let destination = ::PublicKey::from_xdr(inner.destination)?;
    let asset = ::Asset::from_xdr(inner.asset)?;
    let amount = Amount::from_stroops(inner.amount)?;
    Ok(::Operation::Payment(operation::PaymentOperation {
        source,
        destination,
        asset,
        amount,
    }))
}

fn from_path_payment(
    source: Option<::PublicKey>,
    inner: PathPaymentOperation,
) -> Result<::Operation> {
    let destination = ::PublicKey::from_xdr(inner.destination)?;
    let send_asset = ::Asset::from_xdr(inner.send_asset)?;
    let send_max = Amount::from_stroops(inner.send_max)?;
    let dest_asset = ::Asset::from_xdr(inner.dest_asset)?;
    let dest_amount = Amount::from_stroops(inner.dest_amount)?;
    let path_res: Result<Vec<_>> = inner
        .path
        .into_iter()
        .map(|p| ::Asset::from_xdr(p))
        .collect();
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

fn from_manage_offer(
    source: Option<::PublicKey>,
    inner: ManageOfferOperation,
) -> Result<::Operation> {
    let selling = ::Asset::from_xdr(inner.selling)?;
    let buying = ::Asset::from_xdr(inner.buying)?;
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

fn from_create_passive_offer(
    source: Option<::PublicKey>,
    inner: CreatePassiveOfferOperation,
) -> Result<::Operation> {
    let selling = ::Asset::from_xdr(inner.selling)?;
    let buying = ::Asset::from_xdr(inner.buying)?;
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

fn from_manage_data(
    source: Option<::PublicKey>,
    inner: ManageDataOperation,
) -> Result<::Operation> {
    Ok(::Operation::ManageData(operation::ManageDataOperation {
        source,
        name: inner.name,
        value: inner.value,
    }))
}

fn from_inflation(source: Option<::PublicKey>) -> Result<::Operation> {
    Ok(::Operation::Inflation(operation::InflationOperation {
        source,
    }))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use {InflationOperationBuilder, Operation, OperationBuilder};
    use {Amount, Asset, Price, PublicKey};
    use {FromXdr, ToXdr};

    fn do_it(op: Operation, expected: &str) {
        let encoded = op.clone().to_base64().unwrap();
        assert_eq!(encoded, expected);
        let decoded = Operation::from_base64(&encoded).unwrap();
        assert_eq!(decoded, op);
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
