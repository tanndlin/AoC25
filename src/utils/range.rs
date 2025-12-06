use std::str::FromStr;

#[derive(Clone)]
pub struct Range {
    pub start: u64,
    pub end: u64,
}

pub enum RangeParseError {
    InvalidFormat,
    InvalidNumber(std::num::ParseIntError),
}

impl std::fmt::Debug for RangeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RangeParseError::InvalidFormat => write!(f, "InvalidFormat"),
            RangeParseError::InvalidNumber(err) => write!(f, "InvalidNumber({:?})", err),
        }
    }
}

impl From<std::num::ParseIntError> for RangeParseError {
    fn from(err: std::num::ParseIntError) -> Self {
        RangeParseError::InvalidNumber(err)
    }
}

impl FromStr for Range {
    type Err = RangeParseError;
    fn from_str(range: &str) -> Result<Self, Self::Err> {
        let (start_str, end_str) = range
            .split_once('-')
            .ok_or(RangeParseError::InvalidFormat)?;

        Ok(Range {
            start: start_str.parse()?,
            end: end_str.parse()?,
        })
    }
}

impl Range {
    pub fn intersects(&self, other: &Range) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    pub fn contians(&self, n: u64) -> bool {
        self.start <= n && n <= self.end
    }

    pub fn count_range(&self) -> u64 {
        self.end - self.start + 1
    }

    pub fn expand(&self, other: &Range) -> Range {
        Range {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}
