const BRAVURA: &'static [u8] = include_bytes!("../svg/bravura.svg");

fn main() -> Result<(), std::io::Error> {
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
