use serde::Deserialize;

use crate::Coord;

/// The smallest rectangle that encloses every part of the glyph’s path.
///
/// See the [SMuFL documentation](https://w3c.github.io/smufl/latest/specification/glyphbboxes.html).
#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct BoundingBox {
    /// The north-east corner of the bounding box.
    #[serde(rename = "bBoxNE")]
    pub ne: Coord,

    /// The south-east corner of the bounding box.
    #[serde(rename = "bBoxSW")]
    pub sw: Coord,
}
