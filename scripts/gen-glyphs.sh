#!/usr/bin/env bash

cargo build --bin smufl-gen
echo "Generating..."
cargo run --bin smufl-gen -- submodules/smufl/metadata/glyphnames.json > smufl/src/glyph-temp.rs
mv smufl/src/glyph-temp.rs smufl/src/glyph.rs
echo "Formatting..."
cargo +nightly fmt
echo "Updated smufl/src/glyph.rs"
