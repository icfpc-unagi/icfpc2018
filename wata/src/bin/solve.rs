#![allow(unused)]
extern crate wata;

use wata::*;

enum State {
	Free,
	Moving {
		to: P,
		commands: Vec<Command>
	},
	Filling
}

struct Bot {
	bid: usize,
	seed: Vec<usize>,
	/// current position
	p: P,
	state: State,
}

fn main() {
	let file = std::env::args().nth(1).unwrap();
	let model = wata::read(&file);
	let r = model.r;
	let r3 = (r + 2) / 3;
	let target = model.filled;
	let mut filled = mat![false; r; r; r];
	let mut ground = mat![false; r; r; r];
	let mut plan = mat![!0; r; r; r];
	let mut bots = vec![Bot { bid: 1, seed: (2..21).collect::<Vec<_>>(), p: P::new(0, 0, 0), state: State::Free }];
	for x in 0..r {
		for z in 0..r {
			if target[x][0][z] {
				ground[x][0][z] = true;
			}
		}
	}
	// loop {
	// 	let mut next = vec![];
	// 	let mut free = vec![];
	// 	let mut moves = vec![];
	// 	moves.sort();
	// }
	// loop {
	// 	let mut next = vec![];
	// 	let mut free = vec![];
	// 	let mut moves = vec![];
	// 	// fill
	// 	for b in bots {
	// 		if b.p.x != b.t.0 || b.p.z != b.t.1 {
	// 			free.push(b);
	// 		} else {
				
	// 		}
	// 	}
	// 	for x in 0..r3 {
	// 		for z in 0..r3 {
				
	// 		}
	// 	}
	// 	bots = next;
	// }
}
