# smufl

<!-- cargo-rdme start -->

Parse [SMuFL][smufl] (Standard Music Font Layout) metadata.

SMuFL-compliant fonts can provide a metadata file in JSON format in their
distribution package. The metadata file allows the designer to provide
information that cannot easily (or in some cases at all) be encoded within
or retrieved from the font software itself, including recommendations for
how to draw the elements of music notation not provided directly by the font
itself (such as staff lines, barlines, hairpins, etc.) in a manner
complementary to the design of the font, and important glyph-specific
metrics, such as the precise coordinates at which a stem should connect to a
notehead.

See the [SMuFL documentation][smufl-metadata] for more details.

This crate supports [version 1.40][smufl-version-history] of the SMuFL
specification.

The font metadata can be read into the [`Metadata`] struct:

```rust
use std::{fs::File, io::BufReader};

use smufl::{Glyph, Metadata, StaffSpaces};

let file = File::open("submodules/bravura/redist/bravura_metadata.json")?;
let reader = BufReader::new(file);
let metadata = Metadata::from_reader(reader)?;

assert_eq!(metadata.font_name, "Bravura");
assert_eq!(
    metadata.engraving_defaults.staff_line_thickness.unwrap(),
    StaffSpaces(0.13)
);
assert_eq!(
    metadata.advance_widths.get(Glyph::NoteheadWhole).unwrap(),
    StaffSpaces(1.688)
);
assert_eq!(
    metadata
        .anchors
        .get(Glyph::NoteheadBlack)
        .unwrap()
        .stem_up_se
        .unwrap()
        .x(),
    StaffSpaces(1.18)
);
assert_eq!(
    metadata
        .bounding_boxes
        .get(Glyph::NoteheadBlack)
        .unwrap()
        .ne
        .x(),
    StaffSpaces(1.18)
);
```

[smufl]: https://www.smufl.org/
[smufl-metadata]: https://w3c.github.io/smufl/latest/specification/font-specific-metadata.html
[smufl-version-history]: https://w3c.github.io/smufl/latest/preamble/version-history.html

<!-- cargo-rdme end -->

## License

This project uses the [MIT License][license]. Much of the documentation text is taken from the SMuFL spec, as allowed by [its license][smufl-license].

[license]: https://github.com/peterstuart/smufl/blob/main/LICENSE
[smufl-license]: https://w3c.github.io/smufl/latest/preamble/license.html
