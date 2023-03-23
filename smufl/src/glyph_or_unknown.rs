use serde::Deserialize;

use crate::Glyph;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
#[cfg_attr(test, derive(serde::Serialize))]
#[serde(untagged)]
pub enum GlyphOrUnknown {
    Glyph(Glyph),
    Unknown(String),
}

#[cfg(test)]
impl From<Glyph> for GlyphOrUnknown {
    fn from(value: Glyph) -> Self {
        Self::Glyph(value)
    }
}

#[cfg(test)]
impl From<String> for GlyphOrUnknown {
    fn from(value: String) -> Self {
        Self::Unknown(value)
    }
}

#[cfg(test)]
impl From<&str> for GlyphOrUnknown {
    fn from(value: &str) -> Self {
        Self::Unknown(value.to_owned())
    }
}
