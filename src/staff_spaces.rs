use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, Sub, SubAssign},
};

use serde::Deserialize;

/// The primary unit of measurement for SMuFL fonts.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, PartialOrd)]
#[cfg_attr(test, derive(serde::Serialize))]
#[serde(transparent)]
pub struct StaffSpaces(pub f64);

macro_rules! impl_from {
    ($T:ty) => {
        impl From<$T> for StaffSpaces {
            #[inline(always)]
            fn from(value: $T) -> Self {
                Self(f64::from(value))
            }
        }
    };
}

impl_from!(u8);
impl_from!(u16);
impl_from!(u32);
impl_from!(i8);
impl_from!(i16);
impl_from!(i32);
impl_from!(f32);
impl_from!(f64);

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

impl Mul<Self> for StaffSpaces {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl Mul<f64> for StaffSpaces {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self(self.0 * rhs)
    }
}

impl Div<Self> for StaffSpaces {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self(self.0 / rhs.0)
    }
}

impl Div<f64> for StaffSpaces {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self(self.0 / rhs)
    }
}

impl Sum for StaffSpaces {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self(iter.map(|value| value.0).sum())
    }
}

impl StaffSpaces {
    /// Computes the absolute value of `self`.
    ///
    /// See [f64::abs].
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    /// Returns the minimum of the two numbers.
    ///
    /// See [f64::max].
    #[must_use = "this returns the result of the comparison, without modifying either input"]
    pub fn max(&self, other: Self) -> Self {
        Self(self.0.max(other.0))
    }

    /// Returns the minimum of the two numbers.
    ///
    /// See [f64::min].
    #[must_use = "this returns the result of the comparison, without modifying either input"]
    pub fn min(&self, other: Self) -> Self {
        Self(self.0.min(other.0))
    }

    /// Returns `StaffSpaces(0.0)`.
    pub const fn zero() -> Self {
        Self(0.0)
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use similar_asserts::assert_eq;

    use super::*;

    #[rstest]
    #[case(0, StaffSpaces(0.0))]
    #[case(1, StaffSpaces(1.0))]
    #[case(100, StaffSpaces(100.0))]
    fn from_u8(#[case] from: u8, #[case] expected: StaffSpaces) {
        assert_eq!(StaffSpaces::from(from), expected);
    }

    #[rstest]
    #[case(0, StaffSpaces(0.0))]
    #[case(1, StaffSpaces(1.0))]
    #[case(100, StaffSpaces(100.0))]
    fn from_u16(#[case] from: u16, #[case] expected: StaffSpaces) {
        assert_eq!(StaffSpaces::from(from), expected);
    }

    #[rstest]
    #[case(0, StaffSpaces(0.0))]
    #[case(1, StaffSpaces(1.0))]
    #[case(100, StaffSpaces(100.0))]
    fn from_u32(#[case] from: u32, #[case] expected: StaffSpaces) {
        assert_eq!(StaffSpaces::from(from), expected);
    }

    #[rstest]
    #[case(0, StaffSpaces(0.0))]
    #[case(1, StaffSpaces(1.0))]
    #[case(100, StaffSpaces(100.0))]
    fn from_i8(#[case] from: i8, #[case] expected: StaffSpaces) {
        assert_eq!(StaffSpaces::from(from), expected);
    }

    #[rstest]
    #[case(0, StaffSpaces(0.0))]
    #[case(1, StaffSpaces(1.0))]
    #[case(100, StaffSpaces(100.0))]
    fn from_i16(#[case] from: i16, #[case] expected: StaffSpaces) {
        assert_eq!(StaffSpaces::from(from), expected);
    }

    #[rstest]
    #[case(0, StaffSpaces(0.0))]
    #[case(1, StaffSpaces(1.0))]
    #[case(100, StaffSpaces(100.0))]
    fn from_i32(#[case] from: i32, #[case] expected: StaffSpaces) {
        assert_eq!(StaffSpaces::from(from), expected);
    }

    #[rstest]
    #[case(0.0, StaffSpaces(0.0))]
    #[case(1.5, StaffSpaces(1.5))]
    #[case(100.75, StaffSpaces(100.75))]
    fn from_f32(#[case] from: f32, #[case] expected: StaffSpaces) {
        assert_eq!(StaffSpaces::from(from), expected);
    }

    #[rstest]
    #[case(0.0, StaffSpaces(0.0))]
    #[case(1.5, StaffSpaces(1.5))]
    #[case(100.3, StaffSpaces(100.3))]
    fn from_f64(#[case] from: f64, #[case] expected: StaffSpaces) {
        assert_eq!(StaffSpaces::from(from), expected);
    }

    #[rstest]
    #[case(StaffSpaces(0.0), 0.0)]
    #[case(StaffSpaces(1.5), 1.5)]
    #[case(StaffSpaces(100.3), 100.3)]
    fn into_f64(#[case] from: StaffSpaces, #[case] expected: f64) {
        assert_eq!(f64::from(from), expected);
    }

    #[rstest]
    #[case(StaffSpaces(1.0), StaffSpaces(2.0), StaffSpaces(3.0))]
    #[case(StaffSpaces(2.0), StaffSpaces(1.0), StaffSpaces(3.0))]
    fn add(#[case] left: StaffSpaces, #[case] right: StaffSpaces, #[case] expected: StaffSpaces) {
        assert_eq!(left + right, expected);
    }

    #[rstest]
    #[case(StaffSpaces(1.0), StaffSpaces(2.0), StaffSpaces(3.0))]
    #[case(StaffSpaces(2.0), StaffSpaces(1.0), StaffSpaces(3.0))]
    fn add_assign(
        #[case] mut left: StaffSpaces,
        #[case] right: StaffSpaces,
        #[case] expected: StaffSpaces,
    ) {
        left += right;
        assert_eq!(left, expected);
    }

    #[rstest]
    #[case(StaffSpaces(3.0), StaffSpaces(2.0), StaffSpaces(1.0))]
    #[case(StaffSpaces(1.0), StaffSpaces(3.0), StaffSpaces(-2.0))]
    fn sub(#[case] left: StaffSpaces, #[case] right: StaffSpaces, #[case] expected: StaffSpaces) {
        assert_eq!(left - right, expected);
    }

    #[rstest]
    #[case(StaffSpaces(3.0), StaffSpaces(2.0), StaffSpaces(1.0))]
    #[case(StaffSpaces(1.0), StaffSpaces(3.0), StaffSpaces(-2.0))]
    fn sub_assign(
        #[case] mut left: StaffSpaces,
        #[case] right: StaffSpaces,
        #[case] expected: StaffSpaces,
    ) {
        left -= right;
        assert_eq!(left, expected);
    }

    #[rstest]
    #[case(StaffSpaces(3.0), StaffSpaces(2.0), StaffSpaces(6.0))]
    #[case(StaffSpaces(2.0), StaffSpaces(3.0), StaffSpaces(6.0))]
    fn mul_self(
        #[case] left: StaffSpaces,
        #[case] right: StaffSpaces,
        #[case] expected: StaffSpaces,
    ) {
        assert_eq!(left * right, expected);
    }

    #[rstest]
    #[case(StaffSpaces(3.0), 2.0, StaffSpaces(6.0))]
    #[case(StaffSpaces(2.0), 3.0, StaffSpaces(6.0))]
    fn mul_f64(#[case] left: StaffSpaces, #[case] right: f64, #[case] expected: StaffSpaces) {
        assert_eq!(left * right, expected);
    }

    #[rstest]
    #[case(StaffSpaces(6.0), StaffSpaces(3.0), StaffSpaces(2.0))]
    #[case(StaffSpaces(3.0), StaffSpaces(6.0), StaffSpaces(0.5))]
    fn div_self(
        #[case] left: StaffSpaces,
        #[case] right: StaffSpaces,
        #[case] expected: StaffSpaces,
    ) {
        assert_eq!(left / right, expected);
    }

    #[rstest]
    #[case(StaffSpaces(6.0), 3.0, StaffSpaces(2.0))]
    #[case(StaffSpaces(3.0), 6.0, StaffSpaces(0.5))]
    fn div_f64(#[case] left: StaffSpaces, #[case] right: f64, #[case] expected: StaffSpaces) {
        assert_eq!(left / right, expected);
    }

    #[rstest]
    #[case(&[], StaffSpaces(0.0))]
    #[case(&[StaffSpaces(1.0)], StaffSpaces(1.0))]
    #[case(&[StaffSpaces(1.0), StaffSpaces(2.0), StaffSpaces(3.0)], StaffSpaces(6.0))]
    fn sum(#[case] values: &[StaffSpaces], #[case] expected: StaffSpaces) {
        assert_eq!(values.iter().copied().sum::<StaffSpaces>(), expected);
    }

    #[rstest]
    #[case(StaffSpaces(0.0), StaffSpaces(0.0))]
    #[case(StaffSpaces(1.0), StaffSpaces(1.0))]
    #[case(StaffSpaces(-1.0), StaffSpaces(1.0))]
    fn abs(#[case] value: StaffSpaces, #[case] expected: StaffSpaces) {
        assert_eq!(value.abs(), expected);
    }

    #[rstest]
    #[case(StaffSpaces(0.0), StaffSpaces(1.0), StaffSpaces(1.0))]
    #[case(StaffSpaces(1.0), StaffSpaces(0.0), StaffSpaces(1.0))]
    #[case(StaffSpaces(1.0), StaffSpaces(1.0), StaffSpaces(1.0))]
    fn max(#[case] value: StaffSpaces, #[case] other: StaffSpaces, #[case] expected: StaffSpaces) {
        assert_eq!(value.max(other), expected);
    }

    #[rstest]
    #[case(StaffSpaces(0.0), StaffSpaces(1.0), StaffSpaces(0.0))]
    #[case(StaffSpaces(1.0), StaffSpaces(0.0), StaffSpaces(0.0))]
    #[case(StaffSpaces(1.0), StaffSpaces(1.0), StaffSpaces(1.0))]
    fn min(#[case] value: StaffSpaces, #[case] other: StaffSpaces, #[case] expected: StaffSpaces) {
        assert_eq!(value.min(other), expected);
    }

    #[test]
    fn zero() {
        assert_eq!(StaffSpaces::zero(), StaffSpaces(0.0));
    }
}
