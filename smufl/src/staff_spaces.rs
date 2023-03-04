use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

use serde::Deserialize;

/// The primary unit of measurement for SMuFL fonts.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct StaffSpaces(pub f64);

impl From<f64> for StaffSpaces {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<i32> for StaffSpaces {
    fn from(n: i32) -> Self {
        Self(f64::from(n))
    }
}

impl From<StaffSpaces> for f64 {
    fn from(value: StaffSpaces) -> Self {
        value.0
    }
}

impl Add for StaffSpaces {
    type Output = Self;

    fn add(self, StaffSpaces(rhs): Self) -> Self {
        Self(self.0 + rhs)
    }
}

impl AddAssign for StaffSpaces {
    fn add_assign(&mut self, StaffSpaces(rhs): Self) {
        self.0 += rhs
    }
}

impl Sub for StaffSpaces {
    type Output = Self;

    fn sub(self, StaffSpaces(rhs): Self) -> Self {
        Self(self.0 - rhs)
    }
}

impl SubAssign for StaffSpaces {
    fn sub_assign(&mut self, StaffSpaces(rhs): Self) {
        self.0 -= rhs;
    }
}

impl Mul<f64> for StaffSpaces {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Div<f64> for StaffSpaces {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_f64() {
        assert_eq!(StaffSpaces::from(1.0f64), StaffSpaces(1.0));
    }

    #[test]
    fn from_i32() {
        assert_eq!(StaffSpaces::from(1i32), StaffSpaces(1.0));
    }

    #[test]
    fn into_f64() {
        assert_eq!(f64::from(StaffSpaces(1.0)), 1.0);
    }

    #[test]
    fn add() {
        assert_eq!(StaffSpaces(1.0) + StaffSpaces(2.0), StaffSpaces(3.0));
    }

    #[test]
    fn add_assign() {
        let mut value = StaffSpaces(1.0);
        value += StaffSpaces(2.0);

        assert_eq!(value, StaffSpaces(3.0));
    }

    #[test]
    fn sub() {
        assert_eq!(StaffSpaces(3.0) - StaffSpaces(2.0), StaffSpaces(1.0));
    }

    #[test]
    fn sub_assign() {
        let mut value = StaffSpaces(3.0);
        value -= StaffSpaces(2.0);

        assert_eq!(value, StaffSpaces(1.0));
    }

    #[test]
    fn mul() {
        assert_eq!(StaffSpaces(2.0) * 3.0, StaffSpaces(6.0));
    }

    #[test]
    fn div() {
        assert_eq!(StaffSpaces(6.0) / 3.0, StaffSpaces(2.0));
    }
}
