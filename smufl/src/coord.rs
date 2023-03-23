use serde::Deserialize;

use crate::StaffSpaces;

/// X, Y coordinates in staff spaces.
#[doc(alias = "coordinate")]
#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct Coord(pub(crate) StaffSpaces, pub(crate) StaffSpaces);

impl Coord {
    /// Returns the horizontal component of the coordinate.
    pub fn x(&self) -> StaffSpaces {
        self.0
    }

    /// Returns the vertical component of the coordinate.
    pub fn y(&self) -> StaffSpaces {
        self.1
    }
}

#[cfg(test)]
mod tests {
    use similar_asserts::assert_eq;

    use super::*;

    #[test]
    fn x() {
        assert_eq!(
            Coord(StaffSpaces(1.0), StaffSpaces(2.0)).x(),
            StaffSpaces(1.0)
        );
    }

    #[test]
    fn y() {
        assert_eq!(
            Coord(StaffSpaces(1.0), StaffSpaces(2.0)).y(),
            StaffSpaces(2.0)
        );
    }
}
