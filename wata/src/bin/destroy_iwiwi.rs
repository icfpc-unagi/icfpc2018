extern crate wata;

use wata::destruction::App;

fn main() {
    assert_eq!(std::env::args().nth(1).unwrap(), ""); // I am destroy-only solver
    let file = std::env::args().nth(2).unwrap();
    let model = wata::read(&file);
    let mut app = App::new(&model);
    app.main();
}
