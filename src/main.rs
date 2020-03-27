//! Convert from a SMuFL SVG font to a ScoreFall font (optimized for web).

use std::collections::HashMap;

use sfff::{Glyph, GlyphsBuilder, SfFontMetadata};
use std::fmt::Write;
use svgdom::{AttributeId, AttributeValue, ElementId, FilterSvg, Document, Path};
use serde_json as json;

use Glyph::*;

use smufl_serde::SMuFLMetadata;

mod smufl_serde {
    #![allow(non_snake_case)]

    use serde_derive::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize)]
    pub struct EngravingDefaults {
        /// The thickness of each staff line
        pub staffLineThickness: f32,
        /// The thickness of a stem
        pub stemThickness: f32,
        /// The thickness of a beam
        pub beamThickness: f32,
        /// The distance between the inner edge of the primary and outer edge of
        /// subsequent secondary beams
        pub beamSpacing: f32,
        /// The thickness of a leger line (normally somewhat thicker than a staff line)
        pub legerLineThickness: f32,
        /// The amount by which a leger line should extend either side of a
        /// notehead, scaled proportionally with the notehead's size, e.g. when
        /// scaled down as a grace note
        pub legerLineExtension: f32,
        /// The thickness of the end of a slur
        pub slurEndpointThickness: f32,
        /// The thickness of the mid-point of a slur (i.e. its thickest point)
        pub slurMidpointThickness: f32,
        /// The thickness of the end of a tie
        pub tieEndpointThickness: f32,
        /// The thickness of the mid-point of a tie
        pub tieMidpointThickness: f32,
        /// The thickness of a thin barline, e.g. a normal barline, or each of the
        /// lines of a double barline
        pub thinBarlineThickness: f32,
        /// The thickness of a thick barline, e.g. in a final barline or a repeat
        /// barline
        pub thickBarlineThickness: f32,
        /// The thickness of a dashed barline
        pub dashedBarlineThickness: f32,
        /// The length of the dashes to be used in a dashed barline
        pub dashedBarlineDashLength: f32,
        /// The length of the gap between dashes in a dashed barline
        pub dashedBarlineGapLength: f32,
        /// The default distance between multiple barlines when locked together,
        /// e.g. between two thin barlines making a double barline, or a thin and a
        /// thick barline making a final barline, measured from the right-hand edge
        /// of the left barline to the left-hand edge of the right barline.
        pub barlineSeparation: f32,
        /// The default horizontal distance between the dots and the inner barline
        /// of a repeat barline, measured from the edge of the dots to the edge of
        /// the barline.
        pub repeatBarlineDotSeparation: f32,
        /// The thickness of the vertical line of a bracket grouping staves together
        pub bracketThickness: f32,
        /// The thickness of the vertical line of a sub-bracket grouping staves
        /// belonging to the same instrument together
        pub subBracketThickness: f32,
        /// The thickness of a crescendo/diminuendo hairpin
        pub hairpinThickness: f32,
        /// The thickness of the dashed line used for an octave line
        pub octaveLineThickness: f32,
        /// The thickness of the line used for piano pedaling
        pub pedalLineThickness: f32,
        /// The thickness of the brackets drawn to indicate repeat endings
        pub repeatEndingLineThickness: f32,
        /// The thickness of the line used for the shaft of an arrow
        pub arrowShaftThickness: f32,
        /// The thickness of the lyric extension line to indicate a melisma in vocal
        /// music
        pub lyricLineThickness: f32,
        /// The thickness of a box drawn around text instructions (e.g. rehearsal
        /// marks)
        pub textEnclosureThickness: f32,
        /// The thickness of the brackets drawn either side of tuplet numbers
        pub tupletBracketThickness: f32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct GlyphsWithAnchors {
        /// The exact position at which the bottom right-hand (south-east) corner of
        /// an angled upward-pointing stem connecting the right-hand side of a
        /// notehead to a vertical stem to its left should start, relative to the
        /// glyph origin, expressed as Cartesian coordinates in staff spaces.
        pub splitStemUpSE: Option<[f32; 2]>,
        /// The exact position at which the bottom left-hand (south-west) corner of
        /// an angled upward-pointing stem connecting the left-hand side of a
        /// notehead to a vertical stem to its right should start, relative to the
        /// glyph origin, expressed as Cartesian coordinates in staff spaces.
        pub splitStemUpSW: Option<[f32; 2]>,
        /// The exact position at which the top right-hand (north-east) corner of an
        /// angled downward-pointing stem connecting the right-hand side of a
        /// notehead to a vertical stem to its left should start, relative to the
        /// glyph origin, expressed as Cartesian coordinates in staff spaces.
        pub splitStemDownNE: Option<[f32; 2]>,
        /// The exact position at which the top left-hand (north-west) corner of an
        /// angled downward-pointing stem connecting the left-hand side of a
        /// notehead to a vertical stem to its right should start, relative to the
        /// glyph origin, expressed as Cartesian coordinates in staff spaces.
        pub splitStemDownNW: Option<[f32; 2]>,
        /// The exact position at which the bottom right-hand (south-east) corner of
        /// an upward-pointing stem rectangle should start, relative to the glyph
        /// origin, expressed as Cartesian coordinates in staff spaces.
        pub stemUpSE: Option<[f32; 2]>,
        /// The exact position at which the top left-hand (north-west) corner of a
        /// downward-pointing stem rectangle should start, relative to the glyph
        /// origin, expressed as Cartesian coordinates in staff spaces.
        pub stemDownNW: Option<[f32; 2]>,
        /// The amount by which an up-stem should be lengthened from its nominal
        /// unmodified length in order to ensure a good connection with a flag, in
        /// spaces.
        pub stemUpNW: Option<[f32; 2]>,
        /// The amount by which a down-stem should be lengthened from its nominal
        /// unmodified length in order to ensure a good connection with a flag, in spaces.
        pub stemDownSW: Option<[f32; 2]>,
        /// The width in staff spaces of a given glyph that should be used for e.g.
        /// positioning leger lines correctly.
        pub nominalWidth: Option<[f32; 2]>,
        /// The position in staff spaces that should be used to position numerals
        /// relative to clefs with ligated numbers where those numbers hang from the
        /// bottom of the clef, corresponding horizontally to the center of the
        /// numeral’s bounding box.
        pub numeralTop: Option<[f32; 2]>,
        /// The position in staff spaces that should be used to position numerals
        /// relative to clefs with ligatured numbers where those numbers sit on the
        /// baseline or at the north-east corner of the G clef, corresponding
        /// horizontally to the center of the numeral’s bounding box.
        pub numeralBottom: Option<[f32; 2]>,
        /// The Cartesian coordinates in staff spaces of the bottom left corner of a
        /// nominal rectangle that intersects the top right corner of the glyph’s
        /// bounding box. This rectangle, together with those in the other four
        /// corners of the glyph’s bounding box, can be cut out to produce a more
        /// detailed bounding box (of abutting rectangles), useful for kerning or
        /// interlocking symbols such as accidentals.
        pub cutOutNE: Option<[f32; 2]>,
        /// The Cartesian coordinates in staff spaces of the top left corner of a
        /// nominal rectangle that intersects the bottom right corner of the glyph’s
        /// bounding box.
        pub cutOutSE: Option<[f32; 2]>,
        /// The Cartesian coordinates in staff spaces of the top right corner of a
        /// nominal rectangle that intersects the bottom left corner of the glyph’s
        /// bounding box.
        pub cutOutSW: Option<[f32; 2]>,
        /// The Cartesian coordinates in staff spaces of the bottom right corner of
        /// a nominal rectangle that intersects the top left corner of the glyph’s
        /// bounding box.
        pub cutOutNW: Option<[f32; 2]>,
        /// The Cartesian coordinates in staff spaces of the position at which the
        /// glyph graceNoteSlashStemUp should be positioned relative to the stem-up
        /// flag of an unbeamed grace note; alternatively, the bottom left corner of
        /// a diagonal line drawn instead of using the above glyph.
        pub graceNoteSlashSW: Option<[f32; 2]>,
        /// The Cartesian coordinates in staff spaces of the top right corner of a
        /// diagonal line drawn instead of using the glyph graceNoteSlashStemUp for
        /// a stem-up flag of an unbeamed grace note.
        pub graceNoteSlashNE: Option<[f32; 2]>,
        /// The Cartesian coordinates in staff spaces of the position at which the
        /// glyph graceNoteSlashStemDown should be positioned relative to the
        /// stem-down flag of an unbeamed grace note; alternatively, the top left
        /// corner of a diagonal line drawn instead of using the above glyph.
        pub graceNoteSlashNW: Option<[f32; 2]>,
        /// The Cartesian coordinates in staff spaces of the bottom right corner of
        /// a diagonal line drawn instead of using the glyph graceNoteSlashStemDown
        /// for a stem-down flag of an unbeamed grace note.
        pub graceNoteSlashSE: Option<[f32; 2]>,
        /// The Cartesian coordinates in staff spaces of the horizontal position at
        /// which a glyph repeats, i.e. the position at which the same glyph or
        /// another of the same group should be positioned to ensure correct
        /// tessellation. This is used for e.g. multi-segment lines and the
        /// component glyphs that make up trills and mordents.
        pub repeatOffset: Option<[f32; 2]>,
        /// The Cartesian coordinates in staff spaces of the left-hand edge of a
        /// notehead with a non-zero left-hand side bearing (e.g. a double whole, or
        /// breve, notehead with two vertical lines at each side), to assist in the
        /// correct horizontal alignment of these noteheads with other noteheads
        /// with zero-width left-side bearings.
        pub noteheadOrigin: Option<[f32; 2]>,
        /// The Cartesian coordinates in staff spaces of the optical center of the
        /// glyph, to assist in the correct horizontal alignment of the glyph
        /// relative to a notehead or stem. Currently recommended for use with
        /// glyphs in the Dynamics range.
        pub opticalCenter: Option<[f32; 2]>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Alternate {
        pub codepoint: String,
        pub name: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Alternates {
        pub alternates: Vec<Alternate>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct GlyphBBox {
        pub bBoxNE: [f32; 2],
        pub bBoxSW: [f32; 2],
    }

    #[derive(Serialize, Deserialize)]
    pub struct Ligature {
        pub codepoint: String,
        pub componentGlyphs: Vec<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct SetGlyph {
        pub codepoint: String,
        pub name: String,
        pub alternateFor: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Set {
        /// `type` must be one of:
        /// - "opticalVariantsSmall" Glyphs designed for use on smaller staff sizes.
        /// - "flagsShort" Alternate shorter flags for notes with augmentation dots.
        /// - "flagsStraight" Alternate flags that are straight rather than curved.
        /// - "timeSigsLarge" Alternate time signature digits for use outside the staff.
        /// - "noteheadsLarge" Alternate oversized noteheads.
        pub r#type: String,
        pub description: String,
        pub glyphs: Vec<SetGlyph>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct OptionalGlyph {
        pub classes: Option<Vec<String>>,
        pub codepoint: String,
    }

    /// JSON Metadata for SMuFL format
    #[derive(Serialize, Deserialize)]
    pub struct SMuFLMetadata {
        /// The name of the font to which the metadata applies
        pub fontName: String,
        /// The version number of the font to which the metadata applies
        pub fontVersion: f32,
        /// Recommended defaults for line widths etc, all measurements expressed in
        /// staff spaces
        pub engravingDefaults: Option<EngravingDefaults>,
        /// A structure for each glyph for which metadata is supplied, with the
        /// canonical glyph name as the key
        pub glyphsWithAnchors: Option<GlyphsWithAnchors>,
        /// List of the glyphs in the font for which stylistic alternates are provided,
        /// together with their name and code point. Applications that cannot access
        /// advanced font features like OpenType stylistic alternates can instead
        /// determine the presence of an alternate for a given glyph, and its code
        /// point, using this data.
        pub glyphsWithAlternates: HashMap<String, Alternates>,
        /// Information about the actual bounding box for each glyph.  The glyph
        /// bounding box is defined as the smallest rectangle that encloses every
        /// part of the glyph’s path, and is described as a pair of coordinates for
        /// the bottom-left (or southwest) and top-right (or northeast) corners of
        /// the rectangle, expressed staff spaces to any required degree of
        /// precision, relative to the glyph origin.
        pub glyphBBoxes: HashMap<String, GlyphBBox>,
        /// A list of ligatures defined in the font. Applications that cannot access
        /// advanced font features like OpenType ligatures can instead determine the
        /// presence of a ligature that joins together a number of recommended
        /// glyphs, and its code point, using this data.
        pub ligatures: HashMap<String, Ligature>,
        /// A list of stylistic sets defined in the font. Applications that cannot
        /// access advanced font features like OpenType stylistic sets can instead
        /// determine the presence of sets in a font, the purpose of each set, and
        /// the name and code point of each glyph in each set, using this data.
        pub sets: HashMap<String, Set>,
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
        pub optionalGlyphs: HashMap<String, OptionalGlyph>,
    }
}

fn path(font: &HashMap<u16, Path>, id: u16) -> String {
    let mut output = "".to_string();

    let path: svgdom::Path = font.get(&id).unwrap().clone();

    for i in path.iter() {
        use svgdom::PathSegment::*;
        let i: svgdom::PathSegment = *i;
        match i {
            MoveTo { abs, x, y } => {
                let y = -y;
                let cmd = if abs { "M" } else { "m" };
                if y < 0.0 {
                    write!(output, "{}{}{}", cmd, x, y).unwrap();
                } else {
                    write!(output, "{}{} {}", cmd, x, y).unwrap();
                }
            },
            LineTo { abs, x, y } => {
                let y = -y;
                let cmd = if abs { "L" } else { "l" };
                if y < 0.0 {
                    write!(output, "{}{}{}", cmd, x, y).unwrap();
                } else {
                    write!(output, "{}{} {}", cmd, x, y).unwrap();
                }
            },
            HorizontalLineTo { abs, x } => {
                let cmd = if abs { "H" } else { "h" };
                write!(output, "{}{}", cmd, x).unwrap();
            },
            VerticalLineTo { abs, y } => {
                let y = -y;
                let cmd = if abs { "V" } else { "v" };
                write!(output, "{}{}", cmd, y).unwrap();
            },
            CurveTo {
                abs,
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => {
                let y = -y;
                let y1 = -y1;
                let y2 = -y2;
                let cmd = if abs { "C" } else { "c" };
                write!(output, "{}{}", cmd, x1).unwrap();
                if y1 < 0.0 {
                    write!(output, "{}", y1).unwrap();
                } else {
                    write!(output, " {}", y1).unwrap();
                }
                if x2 < 0.0 {
                    write!(output, "{}", x2).unwrap();
                } else {
                    write!(output, " {}", x2).unwrap();
                }
                if y2 < 0.0 {
                    write!(output, "{}", y2).unwrap();
                } else {
                    write!(output, " {}", y2).unwrap();
                }
                if x < 0.0 {
                    write!(output, "{}", x).unwrap();
                } else {
                    write!(output, " {}", x).unwrap();
                }
                if y < 0.0 {
                    write!(output, "{}", y).unwrap();
                } else {
                    write!(output, " {}", y).unwrap();
                }
            },
            SmoothCurveTo { abs, x2, y2, x, y } => {
                let y = -y;
                let y2 = -y2;
                let cmd = if abs { "S" } else { "s" };
                write!(output, "{}{}", cmd, x2).unwrap();
                if y2 < 0.0 {
                    write!(output, "{}", y2).unwrap();
                } else {
                    write!(output, " {}", y2).unwrap();
                }
                if x < 0.0 {
                    write!(output, "{}", x).unwrap();
                } else {
                    write!(output, " {}", x).unwrap();
                }
                if y < 0.0 {
                    write!(output, "{}", y).unwrap();
                } else {
                    write!(output, " {}", y).unwrap();
                }
            },
            Quadratic { abs, x1, y1, x, y } => {
                let y = -y;
                let y1 = -y1;
                let cmd = if abs { "Q" } else { "q" };
                write!(output, "{}{}", cmd, x1).unwrap();
                if y1 < 0.0 {
                    write!(output, "{}", y1).unwrap();
                } else {
                    write!(output, " {}", y1).unwrap();
                }
                if x < 0.0 {
                    write!(output, "{}", x).unwrap();
                } else {
                    write!(output, " {}", x).unwrap();
                }
                if y < 0.0 {
                    write!(output, "{}", y).unwrap();
                } else {
                    write!(output, " {}", y).unwrap();
                }
            },
            SmoothQuadratic { abs, x, y } => {
                let y = -y;
                let cmd = if abs { "T" } else { "t" };
                if y < 0.0 {
                    write!(output, "{}{}{}", cmd, x, y).unwrap();
                } else {
                    write!(output, "{}{} {}", cmd, x, y).unwrap();
                }
            },
            EllipticalArc {
                abs,
                rx,
                ry,
                x_axis_rotation,
                large_arc,
                sweep,
                x,
                y,
            } => {
                let ry = -ry;
                let y = -y;
                let cmd = if abs { "A" } else { "a" };
                write!(output, "{}{}", cmd, rx).unwrap();
                if ry < 0.0 {
                    write!(output, "{}", ry).unwrap();
                } else {
                    write!(output, " {}", ry).unwrap();
                }
                if x_axis_rotation < 0.0 {
                    write!(output, "{}", x_axis_rotation).unwrap();
                } else {
                    write!(output, " {}", x_axis_rotation).unwrap();
                }
                write!(output, " {} {}", if large_arc { 1 } else { 0 }, if sweep { 1 } else { 0 }).unwrap();
                if x < 0.0 {
                    write!(output, "{}", x).unwrap();
                } else {
                    write!(output, " {}", x).unwrap();
                }
                if y < 0.0 {
                    write!(output, "{}", y).unwrap();
                } else {
                    write!(output, " {}", y).unwrap();
                }
            },
            ClosePath { abs } => if abs {
                write!(output, "Z").unwrap();
            } else {
                write!(output, "z").unwrap();
            },
        }
    }

    output
}

fn main() {
    let mut args = std::env::args();
    let executable = args.next().unwrap();
    let out = args.next().unwrap_or_else(|| {
        eprintln!("Usage: {} output.sfff font.svg meta.json", executable);
        std::process::exit(1);
    });
    let svg = args.next().unwrap_or_else(|| {
        eprintln!("Usage: {} output.sfff font.svg meta.json", executable);
        std::process::exit(1);
    });
    let meta = args.next().unwrap_or_else(|| {
        eprintln!("Usage: {} output.sfff font.svg meta.json", executable);
        std::process::exit(1);
    });
    let svg_data = std::fs::read_to_string(&svg).unwrap_or_else(|_| {
        eprintln!("No such file: {}", svg);
        std::process::exit(1);
    });
    let metadata = std::fs::read_to_string(&meta).unwrap_or_else(|_| {
        eprintln!("No such file: {}", meta);
        std::process::exit(1);
    });
    let font = Document::from_str(&svg_data).unwrap_or_else(|_| {
        eprintln!("Failed to load font file!");
        std::process::exit(1);
    });

    // Build hashmap of svg font document paths.
    let iter = font.root().descendants().svg();
    let mut font = HashMap::new();
    for (id, node) in iter {
        let attrs = node.attributes();
        match id {
            ElementId::Glyph => {
                // Glyph needs a name and path.
                let name = if let Some(&AttributeValue::String(ref name)) =
                    attrs.get_value(AttributeId::GlyphName)
                {
                    name
                } else {
                    continue;
                };
                let path = if let Some(&AttributeValue::Path(ref path)) =
                    attrs.get_value(AttributeId::D)
                {
                    path.clone()
                } else {
                    continue;
                };
                let name = if name.starts_with("uni") {
                    name.trim_start_matches("uni")
                } else {
                    continue;
                };
                if name.contains("uni") || name.contains(".") {
                    // Ignore glyphs for multiple unicode combined or alternate.
                    continue;
                }
                let name = u16::from_str_radix(name, 16).unwrap();

                font.insert(name, path);
            }
            ElementId::Hkern => {
//                println!("Hkern");
            }
            _ => println!("{}", id),
        }
    }

    // Build glyphs string.
    let mut glyphs = GlyphsBuilder::new();
    // Notehead IDS from https://w3c.github.io/smufl/gitbook/tables/noteheads.html
    glyphs.push(NoteheadFill, path(&font, 0xE0A4));
    glyphs.push(NoteheadHalf, path(&font, 0xE0A3));
    glyphs.push(NoteheadWhole, path(&font, 0xE0A2));
    glyphs.push(NoteheadDouble, path(&font, 0xE0A0));
    glyphs.push(NoteheadFillX, path(&font, 0xE0A9));
    glyphs.push(NoteheadHalfX, path(&font, 0xE0A8));
    glyphs.push(NoteheadWholeX, path(&font, 0xE0A7));
    glyphs.push(NoteheadDoubleX, path(&font, 0xE0A6));
    glyphs.push(NoteheadFillTriangle, path(&font, 0xE0BE));
    glyphs.push(NoteheadHalfTriangle, path(&font, 0xE0BC));
    glyphs.push(NoteheadWholeTriangle, path(&font, 0xE0BB));
    glyphs.push(NoteheadDoubleTriangle, path(&font, 0xE0BA));
    glyphs.push(NoteheadFillDiamond, path(&font, 0xE0DB));
    glyphs.push(NoteheadHalfDiamond, path(&font, 0xE0D9));
    glyphs.push(NoteheadWholeDiamond, path(&font, 0xE0D8));
    glyphs.push(NoteheadDoubleDiamond, path(&font, 0xE0D7));
    glyphs.push(NoteheadFillSlash, path(&font, 0xE101));
    glyphs.push(NoteheadHalfSlash, path(&font, 0xE103));
    glyphs.push(NoteheadWholeSlash, path(&font, 0xE102));
    glyphs.push(NoteheadDoubleSlash, path(&font, 0xE10A));
    glyphs.push(NoteheadFillSlashed, path(&font, 0xE0D0));
    glyphs.push(NoteheadHalfSlashed, path(&font, 0xE0D2));
    glyphs.push(NoteheadWholeSlashed, path(&font, 0xE0D4));
    glyphs.push(NoteheadDoubleSlashed, path(&font, 0xE0D6));
    // Accidental IDs from https://w3c.github.io/smufl/gitbook/tables/standard-accidentals-12-edo.html
    glyphs.push(Flat, path(&font, 0xE260));
    glyphs.push(Sharp, path(&font, 0xE262));
    glyphs.push(Natural, path(&font, 0xE261));
    glyphs.push(DoubleFlat, path(&font, 0xE264));
    glyphs.push(DoubleSharp, path(&font, 0xE263));
    // https://w3c.github.io/smufl/gitbook/tables/stein-zimmermann-accidentals-24-edo.html
    glyphs.push(QuarterFlat, path(&font, 0xE280));
    glyphs.push(QuarterSharp, path(&font, 0xE282));
    glyphs.push(ThreeQuarterFlat, path(&font, 0xE281));
    glyphs.push(ThreeQuarterSharp, path(&font, 0xE283));
    glyphs.push(ThirdFlat, path(&font, 0xE48B));
    glyphs.push(ThirdSharp, path(&font, 0xE48A));
    glyphs.push(TwoThirdFlat, path(&font, 0xE48D));
    glyphs.push(TwoThirdSharp, path(&font, 0xE48C));
    glyphs.push(FlagUp8, path(&font, 0xE240));
    glyphs.push(FlagDown8, path(&font, 0xE241));
    glyphs.push(FlagUp16, path(&font, 0xE242));
    glyphs.push(FlagDown16, path(&font, 0xE243));
    glyphs.push(FlagUp32, path(&font, 0xE244));
    glyphs.push(FlagDown32, path(&font, 0xE245));
    glyphs.push(FlagUp64, path(&font, 0xE246));
    glyphs.push(FlagDown64, path(&font, 0xE247));
    glyphs.push(RestMulti, path(&font, 0xE4EE));
    glyphs.push(Rest1, path(&font, 0xE4E3));
    glyphs.push(Rest2, path(&font, 0xE4E4));
    glyphs.push(Rest4, path(&font, 0xE4E5));
    glyphs.push(Rest8, path(&font, 0xE4E6));
    glyphs.push(Rest16, path(&font, 0xE4E7));
    glyphs.push(Rest32, path(&font, 0xE4E8));
    glyphs.push(Rest64, path(&font, 0xE4E9));
    glyphs.push(ClefC, path(&font, 0xE05C));
    glyphs.push(ClefG, path(&font, 0xE050));
    glyphs.push(ClefF, path(&font, 0xE062));
    glyphs.push(ClefN, path(&font, 0xE069));
    glyphs.push(Clef8, path(&font, 0xE07D));
    glyphs.push(Clef15, path(&font, 0xE07E));
    glyphs.push(Tab4, path(&font, 0xE06E));
    glyphs.push(Tab6, path(&font, 0xE06D));
    glyphs.push(P, path(&font, 0xE520));
    glyphs.push(MP, path(&font, 0xE52C));
    glyphs.push(MF, path(&font, 0xE52D));
    glyphs.push(F, path(&font, 0xE522));
    glyphs.push(S, path(&font, 0xE524));
    glyphs.push(Z, path(&font, 0xE525));
    glyphs.push(N, path(&font, 0xE526));
    glyphs.push(TimeSig0, path(&font, 0xE080));
    glyphs.push(TimeSig1, path(&font, 0xE081));
    glyphs.push(TimeSig2, path(&font, 0xE082));
    glyphs.push(TimeSig3, path(&font, 0xE083));
    glyphs.push(TimeSig4, path(&font, 0xE084));
    glyphs.push(TimeSig5, path(&font, 0xE085));
    glyphs.push(TimeSig6, path(&font, 0xE086));
    glyphs.push(TimeSig7, path(&font, 0xE087));
    glyphs.push(TimeSig8, path(&font, 0xE088));
    glyphs.push(TimeSig9, path(&font, 0xE089));
    glyphs.push(TimeSigCommon, path(&font, 0xE08A));
    glyphs.push(TimeSigCut, path(&font, 0xE08B));
    glyphs.push(TimeSigPlus, path(&font, 0xE08C));
    glyphs.push(RepeatSlash, path(&font, 0xE504));
    glyphs.push(RepeatUpDot, path(&font, 0xE503));
    glyphs.push(RepeatDownDot, path(&font, 0xE505));
    glyphs.push(TupletColon, path(&font, 0xE88A));
    glyphs.push(Tuplet0, path(&font, 0xE880));
    glyphs.push(Tuplet1, path(&font, 0xE881));
    glyphs.push(Tuplet2, path(&font, 0xE882));
    glyphs.push(Tuplet3, path(&font, 0xE883));
    glyphs.push(Tuplet4, path(&font, 0xE884));
    glyphs.push(Tuplet5, path(&font, 0xE885));
    glyphs.push(Tuplet6, path(&font, 0xE886));
    glyphs.push(Tuplet7, path(&font, 0xE887));
    glyphs.push(Tuplet8, path(&font, 0xE888));
    glyphs.push(Tuplet9, path(&font, 0xE889));
    glyphs.push(Coda, path(&font, 0xE048));
    glyphs.push(Segno, path(&font, 0xE047));
    glyphs.push(BuzzRoll, path(&font, 0xE217));
    glyphs.push(Damp, path(&font, 0xE218));
    glyphs.push(HarpStringNoise, path(&font, 0xE21F));
    glyphs.push(RimShot, path(&font, 0xE21E));
    glyphs.push(BowBridge, path(&font, 0xE215));
    glyphs.push(BowTailpiece, path(&font, 0xE216));
    glyphs.push(Tremelo1, path(&font, 0xE220));
    glyphs.push(Tremelo2, path(&font, 0xE221));
    glyphs.push(Tremelo3, path(&font, 0xE222));
    glyphs.push(Tremelo4, path(&font, 0xE223));
    glyphs.push(Tremelo5, path(&font, 0xE224));

    let glyph_paths = glyphs.into_string();
    let metadata: SMuFLMetadata = json::from_str(&metadata).unwrap();
    let metadata = metadata.engravingDefaults.unwrap();

    let convert = &mut |ss: f32| {
        (ss * 1000.0) as u32
    };

    let metadata = SfFontMetadata {
        sffonts_version: 0,
        font_name: "Modern".to_string(),
        stave_line_thickness: convert(metadata.staffLineThickness),
        stem_thickness: convert(metadata.stemThickness),
        ledger_line_thickness: convert(metadata.legerLineThickness),
        ledger_line_extension: convert(metadata.legerLineExtension),
        slur_endpoint_thickness: convert(metadata.slurEndpointThickness), 
        slur_midpoint_thickness: convert(metadata.slurMidpointThickness),
        barline_thickness: convert(metadata.thinBarlineThickness),
        thick_barline_thickness: convert(metadata.thickBarlineThickness),
        barlines_space: convert(metadata.barlineSeparation),
        barline_repeatdot_space: convert(metadata.repeatBarlineDotSeparation),
        bracket_thickness: convert(metadata.bracketThickness),
        subbracket_thickness: convert(metadata.subBracketThickness),
        hairpin_thickness: convert(metadata.hairpinThickness),
        rehearsal_box_thickness: convert(metadata.textEnclosureThickness),
    };

    use std::io::Write;

    let file = std::fs::File::create(out).unwrap();
    let mut buf_writer = std::io::BufWriter::new(file);

    metadata.write(&mut buf_writer, &glyph_paths).unwrap();

    buf_writer.flush().unwrap();

    println!("Done!");
}
