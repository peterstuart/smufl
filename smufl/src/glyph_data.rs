use std::collections::HashMap;

use serde::Deserialize;

use crate::{glyph_or_unknown::GlyphOrUnknown, Glyph};

#[derive(Debug, Deserialize)]
/// A map of [Glyph] to some data (`T`).
#[serde(transparent)]
pub struct GlyphData<T> {
    data: HashMap<GlyphOrUnknown, T>,
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
    pub fn try_get(&self, glyph: Glyph) -> Option<T> {
        self.data.get(&GlyphOrUnknown::Glyph(glyph)).copied()
    }

    /// Returns a copy of the data for the given `glyph`. Panics if it isn't
    /// present.
    pub fn get(&self, glyph: Glyph) -> T {
        self.try_get(glyph).unwrap_or_else(|| {
            panic!(
                "{:?} does not have {} defined",
                glyph,
                std::any::type_name::<T>()
            )
        })
    }
}

impl<T> GlyphData<T> {
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
    fn try_get<const NUM: usize>(
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

        assert_eq!(glyph_data.try_get(glyph), expected);
    }

    #[rstest]
    #[should_panic]
    #[case::empty(
        [],
        Glyph::NoteheadBlack,
        1 // not really expected
    )]
    #[case::not_empty(
        [(Glyph::NoteheadBlack, 1), (Glyph::NoteheadWhole, 2)],
        Glyph::NoteheadBlack,
        1
    )]
    fn get<const NUM: usize>(
        #[case] values: [(Glyph, u64); NUM],
        #[case] glyph: Glyph,
        #[case] expected: u64,
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
        let glyph_data: GlyphData<u64> = GlyphData {
            data: values.into_iter().collect(),
        };
        let unknown_glyphs: Vec<_> = glyph_data.unknown_glyphs().collect();
        assert_eq!(unknown_glyphs, expected);
    }
}
