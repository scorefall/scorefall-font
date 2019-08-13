use std::env;
use std::fs;

use score2svg::GlyphId;
use svgdom::{AttributeId, AttributeValue, Document, ElementId, FilterSvg, PathCommand};
use svgdom::{NodeType, WriteBuffer};
use svgdom::PathBuilder;

const BRAVURA: &'static str = include_str!("../svg/bravura.svg");
const FONTNAME: &'static str = "vfont/bravura.vfont";

fn add(out: &mut String, id: GlyphId, path: &svgdom::Path) {
    let mut newpath = svgdom::Path(Vec::with_capacity(path.0.capacity()));

    for i in path.0.iter() {
        use svgdom::PathSegment::*;
        match i {
            &MoveTo{abs, x, y} => newpath.0.push(MoveTo{abs, x, y:-y}),
            &LineTo{abs, x, y} => newpath.0.push(LineTo{abs, x, y:-y}),
            &HorizontalLineTo{abs, x} => newpath.0.push(HorizontalLineTo{abs, x}),
            &VerticalLineTo{abs, y} => newpath.0.push(VerticalLineTo{abs, y:-y}),
            &CurveTo{abs, x1, y1, x2, y2, x, y} => newpath.0.push(CurveTo{abs, x1, y1:-y1, x2, y2:-y2, x, y:-y}),
            &SmoothCurveTo{abs, x2, y2, x, y} => newpath.0.push(SmoothCurveTo{abs, x2, y2:-y2, x, y:-y}),
            &Quadratic{abs, x1, y1, x, y} => newpath.0.push(Quadratic{abs, x1, y1:-y1, x, y:-y}),
            &SmoothQuadratic{abs, x, y} => newpath.0.push(SmoothQuadratic{abs, x, y:-y}),
            &EllipticalArc{abs, rx, ry, x_axis_rotation, large_arc, sweep, x, y} => newpath.0.push(EllipticalArc{abs, rx, ry:-ry, x_axis_rotation, large_arc, sweep, x, y:-y}),
            &ClosePath{abs} => newpath.0.push(ClosePath{abs}),
        }
    }

    out.push_str(&format!("<path id=\"{:x}\" d=\"{}\"/>\n", id as u32, newpath));
}

fn main() -> Result<(), svgdom::ParserError> {
    println!("Converting SVG font to ScoreFall Vector Graphics Font (VFONT)…");

    let input_data = BRAVURA; //fs::read_to_string(&args[1])?;
    let doc = svgdom::Document::from_str(&input_data)?;
    let mut out = String::new();

    out.push_str("<defs>\n");

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
                    "uniE030" => add(&mut out, Barline, path),
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

                    "uniE4E3" => add(&mut out, Rest1, path),
                    "uniE4E4" => add(&mut out, Rest2, path),
                    "uniE4E5" => add(&mut out, Rest4, path),
                    "uniE4F2" => add(&mut out, Rest4Old, path),
                    "uniE4E6" => add(&mut out, Rest8, path),
                    "uniE4E7" => add(&mut out, Rest16, path),
                    "uniE4E8" => add(&mut out, Rest32, path),
                    "uniE4E9" => add(&mut out, Rest64, path),
                    "uniE4EA" => add(&mut out, Rest128, path),

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

    out.push_str("</defs>\n");

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
