use std::collections::HashMap;

use serde::Deserialize;

use crate::{glyph_or_unknown::GlyphOrUnknown, Glyph};

/// A map of [Glyph] to some data (`T`).
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
#[serde(transparent)]
pub struct GlyphData<T> {
    data: HashMap<GlyphOrUnknown, T>,
}

#[cfg(test)]
impl<T, const NUM: usize> From<[(Glyph, T); NUM]> for GlyphData<T> {
    fn from(value: [(Glyph, T); NUM]) -> Self {
        Self {
            data: value
                .into_iter()
                .map(|(glyph, data)| (glyph.into(), data))
                .collect(),
        }
    }
}

#[cfg(test)]
impl<T, const NUM: usize> From<[(GlyphOrUnknown, T); NUM]> for GlyphData<T> {
    fn from(value: [(GlyphOrUnknown, T); NUM]) -> Self {
        Self {
            data: value.into_iter().collect(),
        }
    }
}

impl<T> Default for GlyphData<T> {
    fn default() -> Self {
        Self {
            data: HashMap::default(),
        }
    }
}

impl<T: Copy> GlyphData<T> {
    /// Returns a copy of the data for the given `glyph`, if present.
    pub fn get(&self, glyph: Glyph) -> Option<T> {
        self.data.get(&GlyphOrUnknown::Glyph(glyph)).copied()
    }
}

impl<T> GlyphData<T> {
    /// Insert data from `defaults` for any keys that are not present.
    pub(crate) fn with_defaults(mut self, defaults: Self) -> Self {
        for (glyph, value) in defaults.data {
            self.data.entry(glyph).or_insert(value);
        }

        self
    }

    /// Returns all the unknown glyphs (glyphs whose name was not recognized)
    /// which have data.
    pub(crate) fn unknown_glyphs(&self) -> impl Iterator<Item = &String> {
        self.data.keys().filter_map(|key| match key {
            GlyphOrUnknown::Unknown(unknown) => Some(unknown),
            _ => None,
        })
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use similar_asserts::assert_eq;

    use super::*;

    #[rstest]
    #[case::empty(
        [],
        Glyph::NoteheadBlack,
        None
    )]
    #[case::not_empty(
        [(Glyph::NoteheadBlack, 1), (Glyph::NoteheadWhole, 2)],
        Glyph::NoteheadBlack,
        Some(1)
    )]
    fn get<const NUM: usize>(
        #[case] values: [(Glyph, u64); NUM],
        #[case] glyph: Glyph,
        #[case] expected: Option<u64>,
    ) {
        let glyph_data: GlyphData<u64> = GlyphData {
            data: values
                .into_iter()
                .map(|(glyph, value)| (GlyphOrUnknown::Glyph(glyph), value))
                .collect(),
        };

        assert_eq!(glyph_data.get(glyph), expected);
    }

    #[rstest]
    #[case::both_empty([], [], Glyph::NoteheadBlack, None)]
    #[case::fallback_empty(
        [(Glyph::NoteheadBlack, 1)],
        [],
        Glyph::NoteheadBlack,
        Some(1)
    )]
    #[case::original_empty(
        [],
        [(Glyph::NoteheadBlack, 1)],
        Glyph::NoteheadBlack,
        Some(1)
    )]
    #[case::neither_empty(
        [(Glyph::NoteheadBlack, 1)],
        [(Glyph::NoteheadBlack, 2)],
        Glyph::NoteheadBlack,
        Some(1)
    )]
    fn with_defaults<const ORIGINAL_NUM: usize, const DEFAULTS_NUM: usize>(
        #[case] original: [(Glyph, u64); ORIGINAL_NUM],
        #[case] defaults: [(Glyph, u64); DEFAULTS_NUM],
        #[case] glyph: Glyph,
        #[case] expected: Option<u64>,
    ) {
        let original: GlyphData<u64> = original.into();
        let defaults: GlyphData<u64> = defaults.into();

        let original_with_defaults = original.with_defaults(defaults);

        assert_eq!(original_with_defaults.get(glyph), expected);
    }

    #[rstest]
    #[case::empty([], [])]
    #[case::not_empty(
        [
            (GlyphOrUnknown::Glyph(Glyph::NoteheadBlack), 1),
            (GlyphOrUnknown::Unknown("Unknown".to_owned()), 2)
        ],
        ["Unknown"]
    )]
    fn unknown_glyphs<const NUM: usize, const EXPECTED_NUM: usize>(
        #[case] values: [(GlyphOrUnknown, u64); NUM],
        #[case] expected: [&str; EXPECTED_NUM],
    ) {
        let glyph_data: GlyphData<u64> = values.into();
        let unknown_glyphs: Vec<_> = glyph_data.unknown_glyphs().collect();

        assert_eq!(unknown_glyphs, expected);
    }
}
