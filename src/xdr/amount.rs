use serde::{Deserialize, Deserializer, Serialize, Serializer};
use amount::{Price, Stroops};

impl Serialize for Stroops {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(self.0)
    }
}

impl<'de> Deserialize<'de> for Stroops {
    fn deserialize<D>(deserializer: D) -> Result<Stroops, D::Error>
    where
        D: Deserializer<'de>,
    {
        let amount = i64::deserialize(deserializer)?;
        Ok(Stroops::new(amount))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PriceWrapper {
    pub n: i32,
    pub d: i32,
}

impl Serialize for Price {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let wrapper = PriceWrapper {
            n: self.numerator(),
            d: self.denominator(),
        };
        wrapper.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Price {
    fn deserialize<D>(deserializer: D) -> Result<Price, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wrapper = PriceWrapper::deserialize(deserializer)?;
        Ok(Price::new(wrapper.n, wrapper.d))
    }
}
