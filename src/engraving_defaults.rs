use serde::Deserialize;

use crate::StaffSpaces;

/// Recommended defaults for line widths, etc.
///
/// See the [SMuFL documentation](https://w3c.github.io/smufl/latest/specification/engravingdefaults.html).
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(test, derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
pub struct EngravingDefaults {
    /// An array containing the text font family (or families, in descending
    /// order of preference) that are ideally paired with this music font; this
    /// list may also use the generic font family values defined in CSS, i.e.
    /// serif, sans-serif, cursive, fantasy, and monospace. Generic font family
    /// names should be listed after specific font families.
    #[serde(default)]
    pub text_font_family: Vec<String>,
    /// The thickness of each staff line
    pub staff_line_thickness: Option<StaffSpaces>,
    /// The thickness of a stem
    pub stem_thickness: Option<StaffSpaces>,
    /// The thickness of a beam
    pub beam_thickness: Option<StaffSpaces>,
    /// The distance between the inner edge of the primary and outer edge of
    /// subsequent secondary beams
    pub beam_spacing: Option<StaffSpaces>,
    /// The thickness of a leger line (normally somewhat thicker than a staff
    /// line)
    pub leger_line_thickness: Option<StaffSpaces>,
    /// The amount by which a leger line should extend either side of a
    /// notehead, scaled proportionally with the notehead's size, e.g. when
    /// scaled down as a grace note
    pub leger_line_extension: Option<StaffSpaces>,
    /// The thickness of the end of a slur
    pub slur_endpoint_thickness: Option<StaffSpaces>,
    /// The thickness of the mid-point of a slur (i.e. its thickest point)
    pub slur_midpoint_thickness: Option<StaffSpaces>,
    /// The thickness of the end of a tie
    pub tie_endpoint_thickness: Option<StaffSpaces>,
    /// The thickness of the mid-point of a tie
    pub tie_midpoint_thickness: Option<StaffSpaces>,
    /// The thickness of a thin barline, e.g. a normal barline, or each of the
    /// lines of a double barline
    pub thin_barline_thickness: Option<StaffSpaces>,
    /// The thickness of a thick barline, e.g. in a final barline or a repeat
    /// barline
    pub thick_barline_thickness: Option<StaffSpaces>,
    /// The thickness of a dashed barline
    pub dashed_barline_thickness: Option<StaffSpaces>,
    /// The length of the dashes to be used in a dashed barline
    pub dashed_barline_dash_length: Option<StaffSpaces>,
    /// The length of the gap between dashes in a dashed barline
    pub dashed_barline_gap_length: Option<StaffSpaces>,
    /// The default distance between multiple thin barlines when locked
    /// together, e.g. between two thin barlines making a double barline,
    /// measured from the right-hand edge of the left barline to the left-hand
    /// edge of the right barline.
    pub barline_separation: Option<StaffSpaces>,
    /// The default distance between a pair of thin and thick barlines when
    /// locked together, e.g. between the thin and thick barlines making a final
    /// barline, or between the thick and thin barlines making a start repeat
    /// barline.
    pub thin_thick_barline_separation: Option<StaffSpaces>,
    /// The default horizontal distance between the dots and the inner barline
    /// of a repeat barline, measured from the edge of the dots to the edge of
    /// the barline.
    pub repeat_barline_dot_separation: Option<StaffSpaces>,
    /// The thickness of the vertical line of a bracket grouping staves together
    pub bracket_thickness: Option<StaffSpaces>,
    /// The thickness of the vertical line of a sub-bracket grouping staves
    /// belonging to the same instrument together
    pub sub_bracket_thickness: Option<StaffSpaces>,
    /// The thickness of a crescendo/diminuendo hairpin
    pub hairpin_thickness: Option<StaffSpaces>,
    /// The thickness of the dashed line used for an octave line
    pub octave_line_thickness: Option<StaffSpaces>,
    /// The thickness of the line used for piano pedaling
    pub pedal_line_thickness: Option<StaffSpaces>,
    /// The thickness of the brackets drawn to indicate repeat endings
    pub repeat_ending_line_thickness: Option<StaffSpaces>,
    /// The thickness of the line used for the shaft of an arrow
    pub arrow_shaft_thickness: Option<StaffSpaces>,
    /// The thickness of the lyric extension line to indicate a melisma in vocal
    /// music
    pub lyric_line_thickness: Option<StaffSpaces>,
    /// The thickness of a box drawn around text instructions (e.g. rehearsal
    /// marks)
    pub text_enclosure_thickness: Option<StaffSpaces>,
    /// The thickness of the brackets drawn either side of tuplet numbers
    pub tuplet_bracket_thickness: Option<StaffSpaces>,
    /// The thickness of the horizontal line drawn between two vertical lines,
    /// known as the H-bar, in a multi-bar rest
    pub h_bar_thickness: Option<StaffSpaces>,
}

impl EngravingDefaults {
    pub(crate) fn with_defaults(self, defaults: Self) -> Self {
        Self {
            text_font_family: if self.text_font_family.is_empty() {
                defaults.text_font_family
            } else {
                self.text_font_family
            },
            staff_line_thickness: self.staff_line_thickness.or(defaults.staff_line_thickness),
            stem_thickness: self.stem_thickness.or(defaults.stem_thickness),
            beam_thickness: self.beam_thickness.or(defaults.beam_thickness),
            beam_spacing: self.beam_spacing.or(defaults.beam_spacing),
            leger_line_thickness: self.leger_line_thickness.or(defaults.leger_line_thickness),
            leger_line_extension: self.leger_line_extension.or(defaults.leger_line_extension),
            slur_endpoint_thickness: self
                .slur_endpoint_thickness
                .or(defaults.slur_endpoint_thickness),
            slur_midpoint_thickness: self
                .slur_midpoint_thickness
                .or(defaults.slur_midpoint_thickness),
            tie_endpoint_thickness: self
                .tie_endpoint_thickness
                .or(defaults.tie_endpoint_thickness),
            tie_midpoint_thickness: self
                .tie_midpoint_thickness
                .or(defaults.tie_midpoint_thickness),
            thin_barline_thickness: self
                .thin_barline_thickness
                .or(defaults.thin_barline_thickness),
            thick_barline_thickness: self
                .thick_barline_thickness
                .or(defaults.thick_barline_thickness),
            dashed_barline_thickness: self
                .dashed_barline_thickness
                .or(defaults.dashed_barline_thickness),
            dashed_barline_dash_length: self
                .dashed_barline_dash_length
                .or(defaults.dashed_barline_dash_length),
            dashed_barline_gap_length: self
                .dashed_barline_gap_length
                .or(defaults.dashed_barline_gap_length),
            barline_separation: self.barline_separation.or(defaults.barline_separation),
            thin_thick_barline_separation: self
                .thin_thick_barline_separation
                .or(defaults.thin_thick_barline_separation),
            repeat_barline_dot_separation: self
                .repeat_barline_dot_separation
                .or(defaults.repeat_barline_dot_separation),
            bracket_thickness: self.bracket_thickness.or(defaults.bracket_thickness),
            sub_bracket_thickness: self
                .sub_bracket_thickness
                .or(defaults.sub_bracket_thickness),
            hairpin_thickness: self.hairpin_thickness.or(defaults.hairpin_thickness),
            octave_line_thickness: self
                .octave_line_thickness
                .or(defaults.octave_line_thickness),
            pedal_line_thickness: self.pedal_line_thickness.or(defaults.pedal_line_thickness),
            repeat_ending_line_thickness: self
                .repeat_ending_line_thickness
                .or(defaults.repeat_ending_line_thickness),
            arrow_shaft_thickness: self
                .arrow_shaft_thickness
                .or(defaults.arrow_shaft_thickness),
            lyric_line_thickness: self.lyric_line_thickness.or(defaults.lyric_line_thickness),
            text_enclosure_thickness: self
                .text_enclosure_thickness
                .or(defaults.text_enclosure_thickness),
            tuplet_bracket_thickness: self
                .tuplet_bracket_thickness
                .or(defaults.tuplet_bracket_thickness),
            h_bar_thickness: self.h_bar_thickness.or(defaults.h_bar_thickness),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use similar_asserts::assert_eq;

    use super::*;

    #[fixture]
    fn empty() -> EngravingDefaults {
        EngravingDefaults::default()
    }

    #[fixture]
    fn non_empty() -> EngravingDefaults {
        EngravingDefaults {
            text_font_family: vec!["Non Empty".to_owned()],
            staff_line_thickness: Some(StaffSpaces(1.0)),
            stem_thickness: Some(StaffSpaces(2.0)),
            beam_thickness: Some(StaffSpaces(3.0)),
            beam_spacing: Some(StaffSpaces(4.0)),
            leger_line_thickness: Some(StaffSpaces(5.0)),
            leger_line_extension: Some(StaffSpaces(6.0)),
            slur_endpoint_thickness: Some(StaffSpaces(7.0)),
            slur_midpoint_thickness: Some(StaffSpaces(8.0)),
            tie_endpoint_thickness: Some(StaffSpaces(9.0)),
            tie_midpoint_thickness: Some(StaffSpaces(10.0)),
            thin_barline_thickness: Some(StaffSpaces(11.0)),
            thick_barline_thickness: Some(StaffSpaces(12.0)),
            dashed_barline_thickness: Some(StaffSpaces(13.0)),
            dashed_barline_dash_length: Some(StaffSpaces(14.0)),
            dashed_barline_gap_length: Some(StaffSpaces(15.0)),
            barline_separation: Some(StaffSpaces(16.0)),
            thin_thick_barline_separation: Some(StaffSpaces(17.0)),
            repeat_barline_dot_separation: Some(StaffSpaces(18.0)),
            bracket_thickness: Some(StaffSpaces(19.0)),
            sub_bracket_thickness: Some(StaffSpaces(20.0)),
            hairpin_thickness: Some(StaffSpaces(21.0)),
            octave_line_thickness: Some(StaffSpaces(22.0)),
            pedal_line_thickness: Some(StaffSpaces(23.0)),
            repeat_ending_line_thickness: Some(StaffSpaces(24.0)),
            arrow_shaft_thickness: Some(StaffSpaces(25.0)),
            lyric_line_thickness: Some(StaffSpaces(26.0)),
            text_enclosure_thickness: Some(StaffSpaces(27.0)),
            tuplet_bracket_thickness: Some(StaffSpaces(28.0)),
            h_bar_thickness: Some(StaffSpaces(29.0)),
        }
    }

    #[fixture]
    fn defaults() -> EngravingDefaults {
        EngravingDefaults {
            text_font_family: vec!["Defaults".to_owned()],
            staff_line_thickness: Some(StaffSpaces(30.0)),
            stem_thickness: Some(StaffSpaces(31.0)),
            beam_thickness: Some(StaffSpaces(32.0)),
            beam_spacing: Some(StaffSpaces(33.0)),
            leger_line_thickness: Some(StaffSpaces(34.0)),
            leger_line_extension: Some(StaffSpaces(35.0)),
            slur_endpoint_thickness: Some(StaffSpaces(36.0)),
            slur_midpoint_thickness: Some(StaffSpaces(37.0)),
            tie_endpoint_thickness: Some(StaffSpaces(38.0)),
            tie_midpoint_thickness: Some(StaffSpaces(39.0)),
            thin_barline_thickness: Some(StaffSpaces(40.0)),
            thick_barline_thickness: Some(StaffSpaces(41.0)),
            dashed_barline_thickness: Some(StaffSpaces(42.0)),
            dashed_barline_dash_length: Some(StaffSpaces(43.0)),
            dashed_barline_gap_length: Some(StaffSpaces(44.0)),
            barline_separation: Some(StaffSpaces(45.0)),
            thin_thick_barline_separation: Some(StaffSpaces(46.0)),
            repeat_barline_dot_separation: Some(StaffSpaces(47.0)),
            bracket_thickness: Some(StaffSpaces(48.0)),
            sub_bracket_thickness: Some(StaffSpaces(49.0)),
            hairpin_thickness: Some(StaffSpaces(50.0)),
            octave_line_thickness: Some(StaffSpaces(51.0)),
            pedal_line_thickness: Some(StaffSpaces(52.0)),
            repeat_ending_line_thickness: Some(StaffSpaces(53.0)),
            arrow_shaft_thickness: Some(StaffSpaces(54.0)),
            lyric_line_thickness: Some(StaffSpaces(55.0)),
            text_enclosure_thickness: Some(StaffSpaces(56.0)),
            tuplet_bracket_thickness: Some(StaffSpaces(57.0)),
            h_bar_thickness: Some(StaffSpaces(58.0)),
        }
    }

    #[rstest]
    fn with_defaults_empty_original(empty: EngravingDefaults, defaults: EngravingDefaults) {
        assert_eq!(empty.with_defaults(defaults.clone()), defaults);
    }

    #[rstest]
    fn with_defaults_non_empty_original(non_empty: EngravingDefaults, defaults: EngravingDefaults) {
        assert_eq!(non_empty.clone().with_defaults(defaults), non_empty);
    }
}
