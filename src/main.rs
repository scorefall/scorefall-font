use std::env;
use std::fs;

use score2svg::GlyphId;
use svgdom::{AttributeId, AttributeValue, Document, ElementId, FilterSvg, PathCommand};
use svgdom::{NodeType, WriteBuffer};

const BRAVURA: &'static str = include_str!("../svg/bravura.svg");
const FONTNAME: &'static str = "vfont/bravura.vfont";

fn add(out: &mut String, id: GlyphId, path: &svgdom::Path) {
    out.push_str(&format!("<path transform=\"scale(1 -1)\" id=\"{:x}\" d=\"{}\"/>", id as u32, path));
}

fn main() -> Result<(), svgdom::ParserError> {
    println!("Converting SVG font to ScoreFall Vector Graphics Font (VFONT)…");

    let input_data = BRAVURA; //fs::read_to_string(&args[1])?;
    let doc = svgdom::Document::from_str(&input_data)?;
    let mut out = String::new();

    out.push_str("<defs>");

    for (id, node) in doc.root().descendants().svg() {
        let attrs = node.attributes();
        match id {
            ElementId::Path => {
                if let Some(&AttributeValue::Path(ref path)) = attrs.get_value(AttributeId::D) {
                    println!("path");
                }
            }
            ElementId::Glyph => {
                let name = if let Some(&AttributeValue::String(ref name)) =
                    attrs.get_value(AttributeId::GlyphName)
                {
                    //                    println!("Glyph Name {}", name);
                    name
                } else {
                    continue;
                };
                let path = if let Some(&AttributeValue::Path(ref path)) =
                    attrs.get_value(AttributeId::D)
                {
                    //                    println!("Glyph Path {}", path);
                    path
                } else {
                    continue;
                };

                use GlyphId::*;

                match name.as_str() {
                    "uniE06E" => add(&mut out, ClefTab4, path),
                    "uniE06D" => add(&mut out, ClefTab6, path),
                    "uniE05C" => add(&mut out, ClefC, path),
                    "uniE07B" => add(&mut out, ClefCChange, path),
                    "uniE050" => add(&mut out, ClefG, path),
                    "uniE07A" => add(&mut out, ClefGChange, path),
                    "uniE062" => add(&mut out, ClefF, path),
                    "uniE07C" => add(&mut out, ClefFChange, path),
                    "uniE210" => add(&mut out, Stem, path),
                    "uniE217" => add(&mut out, StemBuzzRoll, path),
                    "uniE218" => add(&mut out, StemDamp, path),
                    "uniE21F" => add(&mut out, StemHarpStringNoise, path),
                    "uniE21E" => add(&mut out, StemRimShot, path),
                    "uniE215" => add(&mut out, StemBowBridge, path),
                    "uniE216" => add(&mut out, StemBowTailpiece, path),
                    "uniE240" | "flags.u3" => add(&mut out, FlagUp8, path),
                    "uniE241" | "flags.d3" => add(&mut out, FlagDown8, path),
                    "uniE242" | "flags.u4" => add(&mut out, FlagUp16, path),
                    "uniE243" | "flags.d4" => add(&mut out, FlagDown16, path),
                    "uniE244" | "flags.u5" => add(&mut out, FlagUp32, path),
                    "uniE245" | "flags.d5" => add(&mut out, FlagDown32, path),
                    "uniE246" | "flags.u6" => add(&mut out, FlagUp64, path),
                    "uniE247" | "flags.d6" => add(&mut out, FlagDown64, path),
                    "uniE248" | "flags.u7" => add(&mut out, FlagUp128, path),
                    "uniE249" | "flags.d7" => add(&mut out, FlagDown128, path),
                    "uniE0A2" | "noteheads.s0" => add(&mut out, NoteheadWhole, path),
                    "uniE0A3" | "noteheads.s1" => add(&mut out, NoteheadHalf, path),
                    "uniE0A4" | "noteheads.s2" => add(&mut out, NoteheadFill, path),

                    "uniE080" => add(&mut out, TimeSig0, path),
                    "uniE081" => add(&mut out, TimeSig1, path),
                    "uniE082" => add(&mut out, TimeSig2, path),
                    "uniE083" => add(&mut out, TimeSig3, path),
                    "uniE084" => add(&mut out, TimeSig4, path),
                    "uniE085" => add(&mut out, TimeSig5, path),
                    "uniE086" => add(&mut out, TimeSig6, path),
                    "uniE087" => add(&mut out, TimeSig7, path),
                    "uniE088" => add(&mut out, TimeSig8, path),
                    "uniE089" => add(&mut out, TimeSig9, path),
                    /*"uni" => add(&mut out, , path),
                    "uni" => add(&mut out, , path),
                    "uni" => add(&mut out, , path),*/
                    _ => {}
                }
            }
            ElementId::Hkern => {
                // TODO
            }
            _ => println!("{}", id),
        }
    }

    out.push_str("</defs>");

    fs::write(FONTNAME, &out).unwrap();

    /*    for op in &mut path {
        match op {
            Close() => {
                println!("CLOSE");
            }
            Move(x, y) => {
                println!("MOVE");
            }
            Line(x, y) => {
                println!("LINE");
            }
            Quad(cx, cy, x, y) => {
                println!("QUAD");
            }
            Cubic(ax, ay, bx, by, x, y) => {
                println!("CUBIC");
            }
            PenWidth(a) => {
                println!("We don't do that {}", a);
            }
        }
    }*/

    //    println!("{:?}", path.xy());

    Ok(())
}
