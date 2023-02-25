use serde::Deserialize;

use crate::StaffSpaces;

/// X, Y coordinates in staff spaces.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub struct Coord(pub(crate) StaffSpaces, pub(crate) StaffSpaces);

impl Coord {
    pub fn x(&self) -> StaffSpaces {
        self.0
    }

    pub fn y(&self) -> StaffSpaces {
        self.1
    }
}

#[cfg(test)]
mod tests {
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
