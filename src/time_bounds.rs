/// Unix timestamp. Number of seconds since epoch.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnixTimestamp(pub i64);

/// A time range for the validity of an operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeBounds {
    lower: Option<UnixTimestamp>,
    upper: Option<UnixTimestamp>,
}

impl TimeBounds {
    /// Create new time bounds for the validity of an operation.
    ///
    /// If `lower` is not `None`, the operation will not be valid before the specified date.
    /// If `upper` is not `None`, the operation will not be valid after the specified date.
    pub fn new(lower: Option<UnixTimestamp>, upper: Option<UnixTimestamp>) -> TimeBounds {
        TimeBounds { lower, upper }
    }

    /// The lower time bound.
    pub fn lower(&self) -> &Option<UnixTimestamp> {
        &self.lower
    }

    /// The upper time bound.
    pub fn upper(&self) -> &Option<UnixTimestamp> {
        &self.upper
    }
}
