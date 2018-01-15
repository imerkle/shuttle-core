use std::result;
use std::str::FromStr;
use std::ops::Mul;
use num_bigint::BigInt;
use bigdecimal::BigDecimal;
use num_traits::cast::{FromPrimitive, ToPrimitive};
use error::{Error, Result};

const STELLAR_SCALE: i64 = 7;

/// Amount in XLM.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Amount {
    inner: BigDecimal,
}

impl Amount {
    pub fn from_stroops(stroops: Stroops) -> Result<Amount> {
        let data = BigInt::from_i64(stroops.0).ok_or(Error::TBD)?;
        let inner = BigDecimal::new(data, STELLAR_SCALE);
        Ok(Amount { inner })
    }

    pub fn as_stroops(&self) -> Result<Stroops> {
        self.clone().into_stroops()
    }

    pub fn into_stroops(self) -> Result<Stroops> {
        let (data, exp) = self.inner.into_bigint_and_exponent();
        if exp != STELLAR_SCALE {
            return Err(Error::TBD); // wrong scale
        }
        match data.to_i64() {
            Some(stroops) => Ok(Stroops::new(stroops)),
            None => Err(Error::TBD),
        }
    }
}

impl FromStr for Amount {
    type Err = Error;

    fn from_str(s: &str) -> result::Result<Amount, Error> {
        let inner = BigDecimal::from_str(&s)?;
        // Check we don't lose precision
        let (_, scale) = inner.as_bigint_and_exponent();
        if scale > STELLAR_SCALE {
            Err(Error::TBD)
        } else {
            let scaled_inner = inner.with_scale(STELLAR_SCALE);
            Ok(Amount {
                inner: scaled_inner,
            })
        }
    }
}

/// Amount in stroops.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Stroops(pub i64);

impl Stroops {
    pub fn new(amount: i64) -> Stroops {
        Stroops(amount)
    }
}

impl Mul<usize> for Stroops {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self {
        Stroops(self.0 * rhs as i64)
    }
}

/// Price in fractional representation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Price {
    numerator: i32,
    denominator: i32,
}

impl Price {
    pub fn new(numerator: i32, denominator: i32) -> Price {
        Price {
            numerator,
            denominator,
        }
    }

    pub fn numerator(&self) -> i32 {
        self.numerator
    }

    pub fn denominator(&self) -> i32 {
        self.denominator
    }
}

#[cfg(test)]
mod tests {
    use std::str;
    use super::{Amount, Stroops};

    #[test]
    fn test_amount_from_str() {
        let amount1 = str::parse::<Amount>("123.4567891").unwrap();
        let amount2 = str::parse::<Amount>("123.4567891").unwrap();
        let amount3 = str::parse::<Amount>("123.4567890").unwrap();

        assert_eq!(amount1, amount2);
        assert_ne!(amount1, amount3);
        assert!(amount3 < amount1);
    }

    #[test]
    fn test_error_too_many_decimals() {
        let res = str::parse::<Amount>("123.45678901");
        assert!(res.is_err());
    }

    #[test]
    fn test_amount_as_stroops() {
        let amount = str::parse::<Amount>("123.45678").unwrap();
        let stroops = amount.as_stroops().unwrap();
        assert_eq!(stroops, Stroops::new(1234567800));
    }
}
