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
