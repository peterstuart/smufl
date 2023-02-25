use serde::Deserialize;

use crate::Coord;

/// The smallest rectangle that encloses every part of the glyph’s path.
///
/// See the [SMuFL documentation](https://w3c.github.io/smufl/latest/specification/glyphbboxes.html).
#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub struct BoundingBox {
    #[serde(rename = "bBoxNE")]
    pub ne: Coord,

    #[serde(rename = "bBoxSW")]
    pub sw: Coord,
}
