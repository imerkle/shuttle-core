use serde::{Deserialize, Deserializer, Serialize, Serializer};
use amount::Stroops;

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
