use error::Result;
use xdr::{FromXdr, ToXdr};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeBounds {
    pub lower: u64,
    pub upper: u64,
}

impl ToXdr<TimeBounds> for ::TimeBounds {
    fn to_xdr(&self) -> Result<TimeBounds> {
        let lower = match *self.lower() {
            None => 0,
            Some(ref t) => t.0 as u64,
        };
        let upper = match *self.upper() {
            None => 0,
            Some(ref t) => t.0 as u64,
        };
        Ok(TimeBounds { lower, upper })
    }
}

impl<'de> FromXdr<'de, TimeBounds> for ::TimeBounds {
    fn from_xdr(time: TimeBounds) -> Result<::TimeBounds> {
        let lower = if time.lower == 0 {
            None
        } else {
            Some(::UnixTimestamp(time.lower as i64))
        };
        let upper = if time.upper == 0 {
            None
        } else {
            Some(::UnixTimestamp(time.upper as i64))
        };
        Ok(::TimeBounds::new(lower, upper))
    }
}
