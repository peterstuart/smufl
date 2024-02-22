#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

//! Parse [SMuFL][smufl] (Standard Music Font Layout) metadata.
//!
//! SMuFL-compliant fonts can provide a metadata file in JSON format in their
//! distribution package. The metadata file allows the designer to provide
//! information that cannot easily (or in some cases at all) be encoded within
//! or retrieved from the font software itself, including recommendations for
//! how to draw the elements of music notation not provided directly by the font
//! itself (such as staff lines, barlines, hairpins, etc.) in a manner
//! complementary to the design of the font, and important glyph-specific
//! metrics, such as the precise coordinates at which a stem should connect to a
//! notehead.
//!
//! See the [SMuFL documentation][smufl-metadata] for more details.
//!
//! This crate supports [version 1.40][smufl-version-history] of the SMuFL
//! specification.
//!
//! The font metadata can be read into the [`Metadata`] struct:
//!
//! ```
//! use std::{fs::File, io::BufReader};
//!
//! use smufl::{Glyph, Metadata, StaffSpaces};
//!
//! # fn example() -> anyhow::Result<()> {
//! let file = File::open("submodules/bravura/redist/bravura_metadata.json")?;
//! let reader = BufReader::new(file);
//! let metadata = Metadata::from_reader(reader)?;
//!
//! assert_eq!(metadata.font_name, "Bravura");
//! assert_eq!(
//!     metadata.engraving_defaults.staff_line_thickness.unwrap(),
//!     StaffSpaces(0.13)
//! );
//! assert_eq!(
//!     metadata.advance_widths.get(Glyph::NoteheadWhole).unwrap(),
//!     StaffSpaces(1.688)
//! );
//! assert_eq!(
//!     metadata
//!         .anchors
//!         .get(Glyph::NoteheadBlack)
//!         .unwrap()
//!         .stem_up_se
//!         .unwrap()
//!         .x(),
//!     StaffSpaces(1.18)
//! );
//! assert_eq!(
//!     metadata
//!         .bounding_boxes
//!         .get(Glyph::NoteheadBlack)
//!         .unwrap()
//!         .ne
//!         .x(),
//!     StaffSpaces(1.18)
//! );
//! # Ok(())
//! # }
//! # example().unwrap()
//! ```
//!
//! [smufl]: https://www.smufl.org/
//! [smufl-metadata]: https://w3c.github.io/smufl/latest/specification/font-specific-metadata.html
//! [smufl-version-history]: https://w3c.github.io/smufl/latest/preamble/version-history.html

mod anchors;
mod bounding_box;
mod coord;
mod engraving_defaults;
mod glyph;
mod glyph_advance_widths;
mod glyph_anchors;
mod glyph_bounding_boxes;
mod glyph_data;
mod glyph_or_unknown;
mod metadata;
mod staff_spaces;

pub use anchors::Anchors;
pub use bounding_box::BoundingBox;
pub use coord::Coord;
pub use engraving_defaults::EngravingDefaults;
pub use glyph::Glyph;
pub use glyph_advance_widths::GlyphAdvanceWidths;
pub use glyph_anchors::GlyphAnchors;
pub use glyph_bounding_boxes::GlyphBoundingBoxes;
pub use glyph_data::GlyphData;
pub use metadata::Metadata;
pub use staff_spaces::StaffSpaces;
