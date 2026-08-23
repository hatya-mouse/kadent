use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Ticks(pub(crate) i64);

impl Ticks {
    pub(crate) const ZERO: Ticks = Ticks(0);
}

impl Add for Ticks {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Ticks {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for Ticks {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Div for Ticks {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl Eq for Ticks {}

impl PartialOrd for Ticks {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ticks {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Display for Ticks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ticks", self.0)
    }
}
