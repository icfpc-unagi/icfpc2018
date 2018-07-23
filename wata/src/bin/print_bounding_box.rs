extern crate wata;
use wata::*;

fn main() {
    let mut n = 1;
    while let Some(file) = std::env::args().nth(n) {
        let model = wata::read(&file);
        println!("{}: {:?}", file, destruction::util::get_bounding_box(&model.filled));
        n += 1;
    }
}

