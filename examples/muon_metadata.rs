//! This example converts JSON metadata from the font into MuON

use muon_rs as muon;
use serde_json as json;

const BRAVURA_METADATA_FILE: &str = include_str!("../bravura-redist/bravura_metadata.json");

mod smufl_serde {
    #![allow(non_snake_case)]

    use serde_derive::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize)]
    pub struct EngravingDefaults {
        /// The thickness of each staff line
        staffLineThickness: f32,
        /// The thickness of a stem
        stemThickness: f32,
        /// The thickness of a beam
        beamThickness: f32,
        /// The distance between the inner edge of the primary and outer edge of
        /// subsequent secondary beams
        beamSpacing: f32,
        /// The thickness of a leger line (normally somewhat thicker than a staff line)
        legerLineThickness: f32,
        /// The amount by which a leger line should extend either side of a
        /// notehead, scaled proportionally with the notehead's size, e.g. when
        /// scaled down as a grace note
        legerLineExtension: f32,
        /// The thickness of the end of a slur
        slurEndpointThickness: f32,
        /// The thickness of the mid-point of a slur (i.e. its thickest point)
        slurMidpointThickness: f32,
        /// The thickness of the end of a tie
        tieEndpointThickness: f32,
        /// The thickness of the mid-point of a tie
        tieMidpointThickness: f32,
        /// The thickness of a thin barline, e.g. a normal barline, or each of the
        /// lines of a double barline
        thinBarlineThickness: f32,
        /// The thickness of a thick barline, e.g. in a final barline or a repeat
        /// barline
        thickBarlineThickness: f32,
        /// The thickness of a dashed barline
        dashedBarlineThickness: f32,
        /// The length of the dashes to be used in a dashed barline
        dashedBarlineDashLength: f32,
        /// The length of the gap between dashes in a dashed barline
        dashedBarlineGapLength: f32,
        /// The default distance between multiple barlines when locked together,
        /// e.g. between two thin barlines making a double barline, or a thin and a
        /// thick barline making a final barline, measured from the right-hand edge
        /// of the left barline to the left-hand edge of the right barline.
        barlineSeparation: f32,
        /// The default horizontal distance between the dots and the inner barline
        /// of a repeat barline, measured from the edge of the dots to the edge of
        /// the barline.
        repeatBarlineDotSeparation: f32,
        /// The thickness of the vertical line of a bracket grouping staves together
        bracketThickness: f32,
        /// The thickness of the vertical line of a sub-bracket grouping staves
        /// belonging to the same instrument together
        subBracketThickness: f32,
        /// The thickness of a crescendo/diminuendo hairpin
        hairpinThickness: f32,
        /// The thickness of the dashed line used for an octave line
        octaveLineThickness: f32,
        /// The thickness of the line used for piano pedaling
        pedalLineThickness: f32,
        /// The thickness of the brackets drawn to indicate repeat endings
        repeatEndingLineThickness: f32,
        /// The thickness of the line used for the shaft of an arrow
        arrowShaftThickness: f32,
        /// The thickness of the lyric extension line to indicate a melisma in vocal
        /// music
        lyricLineThickness: f32,
        /// The thickness of a box drawn around text instructions (e.g. rehearsal
        /// marks)
        textEnclosureThickness: f32,
        /// The thickness of the brackets drawn either side of tuplet numbers
        tupletBracketThickness: f32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct GlyphsWithAnchors {
        /// The exact position at which the bottom right-hand (south-east) corner of
        /// an angled upward-pointing stem connecting the right-hand side of a
        /// notehead to a vertical stem to its left should start, relative to the
        /// glyph origin, expressed as Cartesian coordinates in staff spaces.
        splitStemUpSE: Option<[f32; 2]>,
        /// The exact position at which the bottom left-hand (south-west) corner of
        /// an angled upward-pointing stem connecting the left-hand side of a
        /// notehead to a vertical stem to its right should start, relative to the
        /// glyph origin, expressed as Cartesian coordinates in staff spaces.
        splitStemUpSW: Option<[f32; 2]>,
        /// The exact position at which the top right-hand (north-east) corner of an
        /// angled downward-pointing stem connecting the right-hand side of a
        /// notehead to a vertical stem to its left should start, relative to the
        /// glyph origin, expressed as Cartesian coordinates in staff spaces.
        splitStemDownNE: Option<[f32; 2]>,
        /// The exact position at which the top left-hand (north-west) corner of an
        /// angled downward-pointing stem connecting the left-hand side of a
        /// notehead to a vertical stem to its right should start, relative to the
        /// glyph origin, expressed as Cartesian coordinates in staff spaces.
        splitStemDownNW: Option<[f32; 2]>,
        /// The exact position at which the bottom right-hand (south-east) corner of
        /// an upward-pointing stem rectangle should start, relative to the glyph
        /// origin, expressed as Cartesian coordinates in staff spaces.
        stemUpSE: Option<[f32; 2]>,
        /// The exact position at which the top left-hand (north-west) corner of a
        /// downward-pointing stem rectangle should start, relative to the glyph
        /// origin, expressed as Cartesian coordinates in staff spaces.
        stemDownNW: Option<[f32; 2]>,
        /// The amount by which an up-stem should be lengthened from its nominal
        /// unmodified length in order to ensure a good connection with a flag, in
        /// spaces.
        stemUpNW: Option<[f32; 2]>,
        /// The amount by which a down-stem should be lengthened from its nominal
        /// unmodified length in order to ensure a good connection with a flag, in spaces.
        stemDownSW: Option<[f32; 2]>,
        /// The width in staff spaces of a given glyph that should be used for e.g.
        /// positioning leger lines correctly.
        nominalWidth: Option<[f32; 2]>,
        /// The position in staff spaces that should be used to position numerals
        /// relative to clefs with ligated numbers where those numbers hang from the
        /// bottom of the clef, corresponding horizontally to the center of the
        /// numeral’s bounding box.
        numeralTop: Option<[f32; 2]>,
        /// The position in staff spaces that should be used to position numerals
        /// relative to clefs with ligatured numbers where those numbers sit on the
        /// baseline or at the north-east corner of the G clef, corresponding
        /// horizontally to the center of the numeral’s bounding box.
        numeralBottom: Option<[f32; 2]>,
        /// The Cartesian coordinates in staff spaces of the bottom left corner of a
        /// nominal rectangle that intersects the top right corner of the glyph’s
        /// bounding box. This rectangle, together with those in the other four
        /// corners of the glyph’s bounding box, can be cut out to produce a more
        /// detailed bounding box (of abutting rectangles), useful for kerning or
        /// interlocking symbols such as accidentals.
        cutOutNE: Option<[f32; 2]>,
        /// The Cartesian coordinates in staff spaces of the top left corner of a
        /// nominal rectangle that intersects the bottom right corner of the glyph’s
        /// bounding box.
        cutOutSE: Option<[f32; 2]>,
        /// The Cartesian coordinates in staff spaces of the top right corner of a
        /// nominal rectangle that intersects the bottom left corner of the glyph’s
        /// bounding box.
        cutOutSW: Option<[f32; 2]>,
        /// The Cartesian coordinates in staff spaces of the bottom right corner of
        /// a nominal rectangle that intersects the top left corner of the glyph’s
        /// bounding box.
        cutOutNW: Option<[f32; 2]>,
        /// The Cartesian coordinates in staff spaces of the position at which the
        /// glyph graceNoteSlashStemUp should be positioned relative to the stem-up
        /// flag of an unbeamed grace note; alternatively, the bottom left corner of
        /// a diagonal line drawn instead of using the above glyph.
        graceNoteSlashSW: Option<[f32; 2]>,
        /// The Cartesian coordinates in staff spaces of the top right corner of a
        /// diagonal line drawn instead of using the glyph graceNoteSlashStemUp for
        /// a stem-up flag of an unbeamed grace note.
        graceNoteSlashNE: Option<[f32; 2]>,
        /// The Cartesian coordinates in staff spaces of the position at which the
        /// glyph graceNoteSlashStemDown should be positioned relative to the
        /// stem-down flag of an unbeamed grace note; alternatively, the top left
        /// corner of a diagonal line drawn instead of using the above glyph.
        graceNoteSlashNW: Option<[f32; 2]>,
        /// The Cartesian coordinates in staff spaces of the bottom right corner of
        /// a diagonal line drawn instead of using the glyph graceNoteSlashStemDown
        /// for a stem-down flag of an unbeamed grace note.
        graceNoteSlashSE: Option<[f32; 2]>,
        /// The Cartesian coordinates in staff spaces of the horizontal position at
        /// which a glyph repeats, i.e. the position at which the same glyph or
        /// another of the same group should be positioned to ensure correct
        /// tessellation. This is used for e.g. multi-segment lines and the
        /// component glyphs that make up trills and mordents.
        repeatOffset: Option<[f32; 2]>,
        /// The Cartesian coordinates in staff spaces of the left-hand edge of a
        /// notehead with a non-zero left-hand side bearing (e.g. a double whole, or
        /// breve, notehead with two vertical lines at each side), to assist in the
        /// correct horizontal alignment of these noteheads with other noteheads
        /// with zero-width left-side bearings.
        noteheadOrigin: Option<[f32; 2]>,
        /// The Cartesian coordinates in staff spaces of the optical center of the
        /// glyph, to assist in the correct horizontal alignment of the glyph
        /// relative to a notehead or stem. Currently recommended for use with
        /// glyphs in the Dynamics range.
        opticalCenter: Option<[f32; 2]>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Alternate {
        codepoint: String,
        name: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Alternates {
        alternates: Vec<Alternate>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct GlyphBBox {
        bBoxNE: [f32; 2],
        bBoxSW: [f32; 2],
    }

    #[derive(Serialize, Deserialize)]
    pub struct Ligature {
        codepoint: String,
        componentGlyphs: Vec<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct SetGlyph {
        codepoint: String,
        name: String,
        alternateFor: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Set {
        /// `type` must be one of:
        /// - "opticalVariantsSmall" Glyphs designed for use on smaller staff sizes.
        /// - "flagsShort" Alternate shorter flags for notes with augmentation dots.
        /// - "flagsStraight" Alternate flags that are straight rather than curved.
        /// - "timeSigsLarge" Alternate time signature digits for use outside the staff.
        /// - "noteheadsLarge" Alternate oversized noteheads.
        r#type: String,
        description: String,
        glyphs: Vec<SetGlyph>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct OptionalGlyph {
        classes: Option<Vec<String>>,
        codepoint: String,
    }

    /// JSON Metadata for SMuFL format
    #[derive(Serialize, Deserialize)]
    pub struct SMuFLMetadata {
        /// The name of the font to which the metadata applies
        fontName: String,
        /// The version number of the font to which the metadata applies
        fontVersion: f32,
        /// Recommended defaults for line widths etc, all measurements expressed in
        /// staff spaces
        engravingDefaults: Option<EngravingDefaults>,
        /// A structure for each glyph for which metadata is supplied, with the
        /// canonical glyph name as the key
        glyphsWithAnchors: Option<GlyphsWithAnchors>,
        /// List of the glyphs in the font for which stylistic alternates are provided,
        /// together with their name and code point. Applications that cannot access
        /// advanced font features like OpenType stylistic alternates can instead
        /// determine the presence of an alternate for a given glyph, and its code
        /// point, using this data.
        glyphsWithAlternates: HashMap<String, Alternates>,
        /// Information about the actual bounding box for each glyph.  The glyph
        /// bounding box is defined as the smallest rectangle that encloses every
        /// part of the glyph’s path, and is described as a pair of coordinates for
        /// the bottom-left (or southwest) and top-right (or northeast) corners of
        /// the rectangle, expressed staff spaces to any required degree of
        /// precision, relative to the glyph origin.
        glyphBBoxes: HashMap<String, GlyphBBox>,
        /// A list of ligatures defined in the font. Applications that cannot access
        /// advanced font features like OpenType ligatures can instead determine the
        /// presence of a ligature that joins together a number of recommended
        /// glyphs, and its code point, using this data.
        ligatures: HashMap<String, Ligature>,
        /// A list of stylistic sets defined in the font. Applications that cannot
        /// access advanced font features like OpenType stylistic sets can instead
        /// determine the presence of sets in a font, the purpose of each set, and
        /// the name and code point of each glyph in each set, using this data.
        sets: HashMap<String, Set>,
        /// A list of all the optional glyphs (those in the range of code points
        /// U+F400–U+FFFF) contained within the font. Applications that cannot use
        /// advanced OpenType features can use this structure to identify the
        /// presence of stylistic alternates (though the "glyphsWithAlternates" and
        /// "sets" structures also specify the original glyphs for each alternate by
        /// name).
        ///
        /// However, a font designer may choose to include some characters in his
        /// font that are neither recommended characters in the core SMuFL ranges
        /// nor alternates for any of those characters, i.e. completely private to
        /// the particular font. This structure provides a direct way for a
        /// consuming application to identify the name, code point, and optional
        /// class (or classes) for each optional glyph in the font.
        optionalGlyphs: HashMap<String, OptionalGlyph>,
    }
}

fn main() {
    use smufl_serde::SMuFLMetadata;

    let metadata: SMuFLMetadata = json::from_str(BRAVURA_METADATA_FILE).unwrap();
    let muon = muon::to_string(&metadata).unwrap();

    std::fs::write("scorefall/metadata.muon", muon).unwrap();
}
