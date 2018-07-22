#![allow(unused)]
extern crate wata;

use wata::*;
use wata::bfs::*;
use std::collections::*;

fn main() {
	let file = std::env::args().nth(1).unwrap();
	let model = wata::read(&file);
	let r = model.r;
	let target = model.filled;
}
