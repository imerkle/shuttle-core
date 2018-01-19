#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnixTimestamp(pub i64);

/// A time range for the validity of an operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeBounds {
    lower: Option<UnixTimestamp>,
    upper: Option<UnixTimestamp>,
}

impl TimeBounds {
    pub fn new(lower: Option<UnixTimestamp>, upper: Option<UnixTimestamp>) -> TimeBounds {
        TimeBounds { lower, upper }
    }

    pub fn lower(&self) -> &Option<UnixTimestamp> {
        &self.lower
    }

    pub fn upper(&self) -> &Option<UnixTimestamp> {
        &self.upper
    }
}
