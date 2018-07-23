extern crate wata;
use wata::*;

fn main() {
    assert_eq!(std::env::args().nth(1).unwrap(), ""); // I am destroy-only solver
    let file = std::env::args().nth(2).unwrap();
    let model = wata::read(&file);
    let filled2 = xz::any_y(&model.filled);
    for (bx, bz, small) in xz::shrink(&filled2, 30) {
        eprintln!("({}, {})", bx, bz);
        for line in small.iter() {
            for &f in line.iter() {
                eprint!("{}", if f { "#" } else { "." });
            }
            eprintln!("");
        }
    }
}

