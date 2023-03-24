use std::io::Read;

use itertools::Itertools;
use serde::Deserialize;
use tracing::{debug, instrument, warn};

use crate::{EngravingDefaults, GlyphAdvanceWidths, GlyphAnchors, GlyphBoundingBoxes};

/// Representation of the metadata file provided with a SMuFL font.
///
/// See the [SMuFL documentation](https://w3c.github.io/smufl/latest/specification/font-specific-metadata.html).
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
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

    /// Returns a new `Metadata` which combines `self` and `defaults`, using
    /// values from `defaults` wherever `self` does not have data.
    pub fn with_defaults(mut self, defaults: Self) -> Self {
        self.engraving_defaults = self
            .engraving_defaults
            .with_defaults(defaults.engraving_defaults);
        self.advance_widths = self.advance_widths.with_defaults(defaults.advance_widths);
        self.anchors = self.anchors.with_defaults(defaults.anchors);
        self.bounding_boxes = self.bounding_boxes.with_defaults(defaults.bounding_boxes);

        self
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
    use std::{fs::File, io::BufReader, path::Path};

    use anyhow::Result;
    use rstest::*;
    use similar_asserts::assert_eq;

    use super::*;
    use crate::{Anchors, BoundingBox, Coord, Glyph, StaffSpaces};

    macro_rules! set_snapshot_suffix {
        ($($expr:expr),*) => {
            let mut settings = insta::Settings::clone_current();
            settings.set_snapshot_suffix(format!($($expr,)*));
            let _guard = settings.bind_to_scope();
        }
    }

    #[rstest]
    #[case::bravura("submodules/bravura/redist/bravura_metadata.json")]
    #[case::petaluma("submodules/petaluma/redist/petaluma_metadata.json")]
    #[case::leland("submodules/leland/leland_metadata.json")]
    #[case::sebastian("submodules/sebastian/fonts/Sebastian.json")]
    #[case::leipzig("submodules/verovio/fonts/Leipzig/leipzig_metadata.json")]
    fn from_reader(#[case] file: &str) -> Result<()> {
        let path = Path::new(file);
        let file_name = path.file_name().unwrap().to_str().unwrap();

        set_snapshot_suffix!("{file_name}");

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let metadata = Metadata::from_reader(reader)?;

        insta::with_settings!({sort_maps => true}, {
            insta::assert_ron_snapshot!(metadata);
        });

        Ok(())
    }

    #[fixture]
    fn empty() -> Metadata {
        Metadata {
            font_name: "Empty".to_owned(),
            engraving_defaults: Default::default(),
            advance_widths: Default::default(),
            anchors: Default::default(),
            bounding_boxes: Default::default(),
        }
    }

    #[fixture]
    fn non_empty() -> Metadata {
        let staff_line_thickness: Option<StaffSpaces> = Some(StaffSpaces(1.0));
        let notehead_black_advance_width: StaffSpaces = StaffSpaces(1.0);
        let notehead_black_anchors: Anchors = Anchors {
            split_stem_up_se: Some(Coord(StaffSpaces(1.0), StaffSpaces(1.0))),
            ..Default::default()
        };
        let notehead_black_bounding_box: BoundingBox = BoundingBox {
            ne: Coord(StaffSpaces(1.0), StaffSpaces(1.0)),
            sw: Coord(StaffSpaces(1.0), StaffSpaces(1.0)),
        };

        Metadata {
            font_name: "Defaults".to_owned(),
            engraving_defaults: EngravingDefaults {
                staff_line_thickness,
                ..Default::default()
            },
            advance_widths: [(Glyph::NoteheadBlack, notehead_black_advance_width)].into(),
            anchors: [(Glyph::NoteheadBlack, notehead_black_anchors)].into(),
            bounding_boxes: [(Glyph::NoteheadBlack, notehead_black_bounding_box)].into(),
        }
    }

    #[fixture]
    fn defaults() -> Metadata {
        let staff_line_thickness: Option<StaffSpaces> = Some(StaffSpaces(2.0));
        let notehead_black_advance_width: StaffSpaces = StaffSpaces(2.0);
        let notehead_black_anchors: Anchors = Anchors {
            split_stem_up_se: Some(Coord(StaffSpaces(2.0), StaffSpaces(2.0))),
            ..Default::default()
        };
        let notehead_black_bounding_box: BoundingBox = BoundingBox {
            ne: Coord(StaffSpaces(2.0), StaffSpaces(2.0)),
            sw: Coord(StaffSpaces(2.0), StaffSpaces(2.0)),
        };

        Metadata {
            font_name: "Defaults".to_owned(),
            engraving_defaults: EngravingDefaults {
                staff_line_thickness,
                ..Default::default()
            },
            advance_widths: [(Glyph::NoteheadBlack, notehead_black_advance_width)].into(),
            anchors: [(Glyph::NoteheadBlack, notehead_black_anchors)].into(),
            bounding_boxes: [(Glyph::NoteheadBlack, notehead_black_bounding_box)].into(),
        }
    }

    #[rstest]
    fn with_defaults_original_empty(empty: Metadata, defaults: Metadata) {
        let empty_with_defaults = empty.clone().with_defaults(defaults.clone());

        assert_eq!(empty_with_defaults.font_name, empty.font_name);
        assert_eq!(
            empty_with_defaults.engraving_defaults.staff_line_thickness,
            defaults.engraving_defaults.staff_line_thickness
        );
        assert_eq!(
            empty_with_defaults.advance_widths.get(Glyph::NoteheadBlack),
            defaults.advance_widths.get(Glyph::NoteheadBlack)
        );
        assert_eq!(
            empty_with_defaults.anchors.get(Glyph::NoteheadBlack),
            defaults.anchors.get(Glyph::NoteheadBlack),
        );
        assert_eq!(
            empty_with_defaults.bounding_boxes.get(Glyph::NoteheadBlack),
            defaults.bounding_boxes.get(Glyph::NoteheadBlack),
        );
    }

    #[rstest]
    fn with_defaults_original_non_empty(non_empty: Metadata, defaults: Metadata) {
        let non_empty_with_defaults = non_empty.clone().with_defaults(defaults);

        assert_eq!(non_empty_with_defaults.font_name, non_empty.font_name);
        assert_eq!(
            non_empty_with_defaults
                .engraving_defaults
                .staff_line_thickness,
            non_empty.engraving_defaults.staff_line_thickness
        );
        assert_eq!(
            non_empty_with_defaults
                .advance_widths
                .get(Glyph::NoteheadBlack),
            non_empty.advance_widths.get(Glyph::NoteheadBlack)
        );
        assert_eq!(
            non_empty_with_defaults.anchors.get(Glyph::NoteheadBlack),
            non_empty.anchors.get(Glyph::NoteheadBlack),
        );
        assert_eq!(
            non_empty_with_defaults
                .bounding_boxes
                .get(Glyph::NoteheadBlack),
            non_empty.bounding_boxes.get(Glyph::NoteheadBlack),
        );
    }
}
