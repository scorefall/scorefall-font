use fonterator;
use fonterator::PathOp::*;

const BRAVURA: &'static [u8] = include_bytes!("../otf/bravura.otf");

fn main() -> Result<(), std::io::Error> {
    let mut font = fonterator::FontGroup::new()
        .add(BRAVURA)?;

    let a = '𝅘𝅥𝅮';
    let mut path = font.render(&format!("{}", a), (0.0,0.0), (512.0, 512.0));

    for op in &mut path {
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
    }

    println!("{:?}", path.xy());

    Ok(())
}
