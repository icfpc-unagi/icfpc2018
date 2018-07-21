#![allow(unused)]
extern crate wata;

use wata::*;
use wata::bfs::*;

#[derive(Clone, Debug)]
enum State {
	Free,
	Moving {
		to: P,
		commands: Vec<Command>
	},
	Filling {
		dir: P
	},
	Halt,
}

#[derive(Clone, Debug)]
struct Bot {
	bid: usize,
	/// current position
	p: P,
	state: State,
}

fn split() -> Vec<Bot> {
	unimplemented!()
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
	let mut bots = split();
	for x in 0..r {
		for z in 0..r {
			if target[x][0][z] {
				ground[x][0][z] = true;
			}
		}
	}
	let mut bfs = bfs::BFS::new(r);
	let mut occupied = mat![0; r; r; r];
	let mut cache = mat![0; r; r; r];
	for tid in 1.. {
		for b in &bots {
			occupied[b.p] = tid;
		}
		let mut moves = vec![];
		// fill
		for b in &mut bots {
			match b.state {
				State::Filling { dir } => {
					let mut q = None;
					for p in b.p.near(r) {
						if p != b.p + dir && !filled[p] && ground[p] && plan[p] == b.bid {
							q = Some(p);
							break;
						}
					}
					if let Some(q) = q {
						if occupied[q] == tid {
							moves.push((b.bid, Command::Wait));
						} else {
							moves.push((b.bid, Command::Fill(q - b.p)));
							occupied[q] = tid;
						}
					} else {
						let mut rem = false;
						'lp: for d in 0.. {
							let bp = b.p + dir * d;
							if !bp.is_valid(r) {
								break;
							}
							for p in bp.near(r) {
								if !filled[p] && plan[p] == b.bid {
									rem = true;
									break 'lp;
								}
							}
						}
						b.state = State::Free;
					}
				},
				_ => {
				}
			}
		}
		// plan
		let mut tid2 = tid * bots.len();
		for b in &mut bots {
			tid2 += 1;
			match b.state {
				State::Free => {
					if let Some(cs) = bfs.bfs(
						|p| filled[p] || occupied[p] == tid || plan[p] != !0,
						&vec![b.p],
						|p| {
							if p.x % 3 != 1 || p.z % 3 != 1 || cache[p] == tid2 {
								return false;
							}
							true
						}) {
					}
				},
				_ => {
					
				}
			}
		}
		// move
		moves.sort();
		for (bid, command) in moves {
			println!("{}", command.to_string());
			match command {
				Command::Fill(p) => {
					let p = bots[bid].p + p;
					filled[p] = true;
					for q in p.adj(r) {
						if target[q] {
							ground[q] = true;
						}
					}
				},
				_ => {
				}
			}
		}
	}
}
