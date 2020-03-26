//! Convert from a SMuFL font to a ScoreFall font (optimized for web).

use ttf_parser;
use scorefall_font as font;
use scorefall_font::{Glyph, GlyphsBuilder};

use Glyph::*;

fn path(font: &ttf_parser::Font, id: u16) -> String {
    "".to_string()
}

fn main() {
    let mut args = std::env::args();
    let executable = args.next().unwrap();
    let otf = args.next().unwrap_or_else(|| {
        eprintln!("Usage: {} font.otf meta.json", executable);
        std::process::exit(1);
    });
    let meta = args.next().unwrap_or_else(|| {
        eprintln!("Usage: {} font.otf meta.json", executable);
        std::process::exit(1);
    });
    let otf_data = std::fs::read(&otf).unwrap_or_else(|_| {
        eprintln!("No such file: {}", otf);
        std::process::exit(1);
    });
    let meta_data = std::fs::read(&meta).unwrap_or_else(|_| {
        eprintln!("No such file: {}", meta);
        std::process::exit(1);
    });
    let font = ttf_parser::Font::from_data(&otf_data, 0).unwrap_or_else(|| {
        eprintln!("Failed to load font file!");
        std::process::exit(1);
    });

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

    

    println!("Done!");
}
