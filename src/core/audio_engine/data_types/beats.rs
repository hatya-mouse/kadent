use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Beats(pub(crate) f64);

impl Add for Beats {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Beats {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for Beats {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Div for Beats {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl Eq for Beats {}

impl PartialOrd for Beats {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Beats {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 > other.0 {
            Ordering::Greater
        } else if self.0 == other.0 {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}
