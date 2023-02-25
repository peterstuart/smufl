use std::io::Read;

use itertools::Itertools;
use serde::Deserialize;
use tracing::{debug, instrument, warn};

use crate::{EngravingDefaults, GlyphAdvanceWidths, GlyphAnchors, GlyphBoundingBoxes};

/// Representation of the metadata file provided with a SMuFL font.
///
/// See the [SMuFL documentation](https://w3c.github.io/smufl/latest/specification/font-specific-metadata.html).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    /// The name of the font to which the metadata applies.
    pub font_name: String,

    /// Recommended defaults for line widths, etc.
    ///
    /// See the [SMuFL documentation](https://w3c.github.io/smufl/latest/specification/engravingdefaults.html).
    #[serde(default)]
    pub engraving_defaults: EngravingDefaults,

    /// Advance widths for glyphs.
    ///
    /// See the [SMuFL documentation](https://w3c.github.io/smufl/latest/specification/glyphadvancewidths.html).
    #[serde(default, rename = "glyphAdvanceWidths")]
    pub advance_widths: GlyphAdvanceWidths,

    /// Anchor data for glyphs.
    ///
    /// See the [SMuFL documentation](https://w3c.github.io/smufl/latest/specification/glyphswithanchors.html).
    #[serde(default, rename = "glyphsWithAnchors")]
    pub anchors: GlyphAnchors,

    /// Bounding boxes for glyphs.
    ///
    /// See the [SMuFL documentation](https://w3c.github.io/smufl/latest/specification/glyphbboxes.html)
    #[serde(default, rename = "glyphBBoxes")]
    pub bounding_boxes: GlyphBoundingBoxes,
}

impl Metadata {
    /// Deserializes `Metadata` from JSON data.
    ///
    /// If any unknown glyphs are encountered, they will be logged at the WARN
    /// level.
    #[instrument(skip(reader), err(Debug))]
    pub fn from_reader(reader: impl Read) -> Result<Self, serde_json::Error> {
        let metadata: Self = serde_json::from_reader(reader)?;
        metadata.log_unknowns();

        Ok(metadata)
    }

    fn log_unknowns(&self) {
        let unknowns = self
            .advance_widths
            .unknown_glyphs()
            .chain(self.anchors.unknown_glyphs())
            .chain(self.bounding_boxes.unknown_glyphs())
            .unique()
            .sorted()
            .collect::<Vec<_>>();

        if unknowns.is_empty() {
            debug!("No unknowns glyphs found");
        } else {
            warn!(?unknowns, "Unknown glyphs found");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use anyhow::Result;
    use rstest::*;

    use super::*;
    use crate::{BoundingBox, Coord, Glyph, StaffSpaces};

    #[rstest]
    #[case::bravura(
        "../submodules/bravura/redist/bravura_metadata.json",
        "Bravura",
        Some(StaffSpaces(0.13)),
        Some(StaffSpaces(1.688)),
        Some(Coord(StaffSpaces(1.18), StaffSpaces(0.168))),
        Some(BoundingBox {
            ne: Coord(StaffSpaces(1.18), StaffSpaces(0.5)),
            sw: Coord(StaffSpaces(0.0), StaffSpaces(-0.5))
        })
    )]
    #[case::petaluma(
        "../submodules/petaluma/redist/petaluma_metadata.json",
        "Petaluma",
        Some(StaffSpaces(0.13)),
        None,
        Some(Coord(StaffSpaces(1.336), StaffSpaces(0.288))),
        Some(BoundingBox {
            ne: Coord(StaffSpaces(1.3361857773586716), StaffSpaces(0.656)),
            sw: Coord(StaffSpaces(0.0), StaffSpaces(-0.656))
        })
    )]
    #[case::leland(
        "../submodules/leland/leland_metadata.json",
        "Leland",
        Some(StaffSpaces(0.11)),
        None,
        Some(Coord(StaffSpaces(1.3), StaffSpaces(0.16))),
        Some(BoundingBox {
            ne: Coord(StaffSpaces(1.3), StaffSpaces(0.528)),
            sw: Coord(StaffSpaces(0.0), StaffSpaces(-0.532))
        })
    )]
    fn from_reader(
        #[case] file: &str,
        #[case] expected_font_name: &str,
        #[case] expected_staff_line_thickness: Option<StaffSpaces>,
        #[case] expected_advance_width: Option<StaffSpaces>,
        #[case] expected_anchor: Option<Coord>,
        #[case] expected_bounding_box: Option<BoundingBox>,
    ) -> Result<()> {
        let file = File::open(file)?;
        let reader = BufReader::new(file);
        let metadata = Metadata::from_reader(reader)?;

        assert_eq!(metadata.font_name, expected_font_name);

        assert_eq!(
            metadata.engraving_defaults.staff_line_thickness, expected_staff_line_thickness,
            "Staff line thickness"
        );

        assert_eq!(
            metadata.advance_widths.try_get(Glyph::NoteheadWhole),
            expected_advance_width,
            "NoteheadWhole advance width"
        );

        assert_eq!(
            metadata
                .anchors
                .try_get(Glyph::NoteheadBlack)
                .and_then(|anchor| anchor.stem_up_se),
            expected_anchor,
            "NoteheadBlack stem_up_se anchor"
        );

        assert_eq!(
            metadata.bounding_boxes.try_get(Glyph::NoteheadBlack),
            expected_bounding_box,
            "NoteheadBlack bounding box"
        );

        Ok(())
    }
}
