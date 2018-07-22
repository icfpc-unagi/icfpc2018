#![allow(unused)]
extern crate wata;

use wata::*;
use wata::bfs::*;
use std::collections::*;

const FISSION: bool = true;//false;

#[derive(Clone, Debug)]
struct Bot {
	bid: usize,
	p: P,
	commands: Vec<Command>,
}

fn one_step<F: Fn(i32, i32) -> bool>(x: i32, z: i32, r: usize, filled: F) -> Vec<(i32, i32, Command)> {
	let mut ps = vec![];
	for &(dx, dz) in &[(-1, 0), (0, -1), (0, 1), (1, 0)] {
		for d in 1..16 {
			let x2 = x + dx * d;
			let z2 = z + dz * d;
			if x2 < 0 || x2 >= r as i32 || z2 < 0 || z2 >= r as i32 || filled(x2, z2) {
				break;
			}
			ps.push((x2 - x, z2 - z, Command::SMove(P::new(dx * d, 0, dz * d))));
			if d <= 5 {
				for &(dx2, dz2) in &[(-dz, dx), (dz, -dx)] {
					for d2 in 1..6 {
						let x3 = x2 + dx2 * d2;
						let z3 = z2 + dz2 * d2;
						if x3 < 0 || x3 >= r as i32 || z3 < 0 || z3 >= r as i32 || filled(x3, z3) {
							break;
						}
						ps.push((x3 - x, z3 - z, Command::LMove(P::new(dx * d, 0, dz * d), P::new(dx2 * d2, 0, dz2 * d2))));
					}
				}
			}
		}
	}
	ps
}

fn output_layer(target: &V3<bool>, filled: &V3<bool>, ground: &Vec<Vec<bool>>, bots: &Vec<Bot>, y0: i32) {
	let r = target.len();
	let mut out = mat!['.'; r; r];
	for x in 0..r {
		for z in 0..r {
			out[x][z] = if filled[x][y0 as usize][z] {
				'f'
			} else if ground[x][z] {
				'g'
			} else if target[x][y0 as usize][z] {
				't'
			} else {
				'.'
			};
		}
	}
	for b in bots {
		let c = &mut out[b.p.x as usize][b.p.z as usize];
		if *c == '.' {
			*c = 'B'
		} else {
			*c = c.to_uppercase().to_string().chars().nth(0).unwrap();
		}
	}
	for x in 0..r {
		for z in 0..r {
			eprint!("{}", out[x][z]);
		}
		eprintln!();
	}
}

fn set_occupied(bp: P, command: Command, occupied: &mut InitV3<bool>) {
	match command {
		Command::SMove(d) => {
			let len = d.mlen();
			let d = d / len;
			for i in 1..=len {
				let p = bp + d * i;
				assert!(!occupied[p]);
				occupied[p] = true;
			}
		},
		Command::LMove(d1, d2) => {
			let len1 = d1.mlen();
			let d1 = d1 / len1;
			let len2 = d2.mlen();
			let d2 = d2 / len2;
			for i in 1..=len1 {
				let p = bp + d1 * i;
				assert!(!occupied[p]);
				occupied[p] = true;
			}
			for i in 1..=len2 {
				let p = bp + d1 * len1 + d2 * i;
				assert!(!occupied[p]);
				occupied[p] = true;
			}
		},
		_ => {
		}
	}
}

fn check_occupied(bp: P, command: Command, occupied: &InitV3<bool>) -> bool {
	match command {
		Command::SMove(d) => {
			let len = d.mlen();
			let d = d / len;
			for i in 1..=len {
				let p = bp + d * i;
				if occupied[p] {
					return false;
				}
			}
		},
		Command::LMove(d1, d2) => {
			let len1 = d1.mlen();
			let d1 = d1 / len1;
			let len2 = d2.mlen();
			let d2 = d2 / len2;
			for i in 1..=len1 {
				let p = bp + d1 * i;
				if occupied[p] {
					return false;
				}
			}
			for i in 1..=len2 {
				let p = bp + d1 * len1 + d2 * i;
				if occupied[p] {
					return false;
				}
			}
		},
		_ => {
		}
	}
	true
}

fn destruct_support(target: &V3<bool>, filled: &mut V3<bool>, bots: &mut Vec<Bot>) -> i64 {
	let r = target.len();
	let mut supports = vec![];
	for x in 0..r {
		for z in 0..r {
			let mut y = 0;
			while y < r {
				if !target[x][y][z] && filled[x][y][z] {
					let mut s = y;
					while !target[x][y][z] {
						y += 1;
					}
					let t = y - 1;
					while t - s > 30 {
						supports.push([P::new(x as i32, s as i32, z as i32), P::new(x as i32, (s + 30) as i32, z as i32)]);
						s += 31;
					}
					supports.push([P::new(x as i32, s as i32, z as i32), P::new(x as i32, t as i32, z as i32)]);
				}
				y += 1;
			}
		}
	}
	eprintln!("support = {:?}", supports);
	let mut bs: Vec<[Option<usize>; 2]> = vec![[None, None]; supports.len()];
	let mut finished = vec![false; supports.len()];
	let mut rem = supports.len();
	let mut t = bots[0].commands.len();
	let mut occupied = InitV3::new(false, r);
	let mut bpos = InitV3::new(!0, r);
	let mut bfs = BFS::new(r);
	let mut working = vec![None; bots.len()];
	while rem > 0 {
		eprintln!("rem: {}", rem);
		let mut free = BTreeSet::new();
		occupied.init();
		for b in bots.iter() {
			occupied[b.p] = true;
			if working[b.bid].is_none() {
				free.insert(b.bid);
			}
		}
		let mut moved = vec![false; bots.len()];
		for b in bots.iter_mut() {
			if b.commands.len() > t {
				if check_occupied(b.p, b.commands[t], &occupied) {
					set_occupied(b.p, b.commands[t], &mut occupied);
					moved[b.bid] = true;
				} else {
					if let Some((i, j)) = working[b.bid] {
						// bs[i][j] = None;
						b.commands.truncate(t);
					} else {
						assert!(false);
					}
				}
			}
		}
		if !free.is_empty() {
			for i in 0..bs.len() {
				if finished[i] {
					continue;
				}
				for j in 0..2 {
					if !free.is_empty() && bs[i][j].is_none() {
						bpos.init();
						for &bid in &free {
							bpos[bots[bid].p] = bid;
						}
						let mut starts = vec![];
						for t in supports[i][j].near(r) {
							if !filled[t] {
								starts.push(t);
							}
						}
						bfs.clear();
						if let Some(s) = bfs.bfs(|p| filled[p], &starts, |p| bpos[p] != !0) {
							let bid = bpos[s];
							free.remove(&bid);
							working[bid] = Some((i, j));
							bots[bid].commands.extend(bfs.restore_backward(s));
							bs[i][j] = Some(bid);
							break;
						}
					}
				}
			}
		}
		for i in 0..bs.len() {
			match bs[i] {
				[Some(a), Some(b)] if bots[a].commands.len() == t && bots[b].commands.len() == t => {
					let ca = Command::GVoid(supports[i][0] - bots[a].p, supports[i][1] - supports[i][0]);
					bots[a].commands.push(ca);
					let cb = Command::GVoid(supports[i][1] - bots[b].p, supports[i][0] - supports[i][1]);
					bots[b].commands.push(cb);
					finished[i] = true;
					working[a] = None;
					working[b] = None;
					rem -= 1;
				},
				_ => {
				}
			}
		}
		t += 1;
	}
	for b in bots.iter_mut() {
		b.commands.truncate(t);
	}
	0
}

fn fill_layer_bottom(target: &V3<bool>, filled: &mut V3<bool>, occupied: &mut InitV3<bool>, bots: &mut Vec<Bot>, y0: i32) -> i64 {
	let r = target.len();
	let nbots = bots.len();
	let mut energy = 0;
	let mut rem = 0;
	for x in 0..r {
		for z in 0..r {
			if target[x][y0 as usize][z] {
				rem += 1;
			}
		}
	}
	if bots[0].p.y != y0 + 1 {
		assert!(bots[0].p.y == y0);
		for b in bots.iter_mut() {
			b.p.y += 1;
			b.commands.push(Command::SMove(P::new(0, 1, 0)));
		}
		energy += 1;
	}
	let mut ground = mat![false; r; r];
	for x in 0..r {
		for z in 0..r {
			if target[x][y0 as usize][z] && (y0 == 0 || filled[x][y0 as usize - 1][z]) {
				ground[x][z] = true;
			}
		}
	}
	let mut near_vb = mat![vec![]; r; r];
	let mut t = bots[0].commands.len();
	while rem > 0 {
		eprintln!("y = {}, rem = {}", y0, rem);
		// output_layer(target, filled, &ground, &bots, y0);
		occupied.init();
		let mut near_bv = vec![vec![]; nbots];
		for b in bots.iter() {
			occupied[b.p] = true;
			for p in b.p.near(r) {
				if p.y == y0 && target[p] && !filled[p] {
					near_bv[b.bid].push(p);
					near_vb[p.x as usize][p.z as usize].push(b.bid);
				}
			}
		}
		// fill
		for b in bots.iter_mut() {
			let mut min_size = 100;
			let mut q = P::new(0, 0, 0);
			for p in b.p.near(r) {
				if p.y == y0 && ground[p.x as usize][p.z as usize] && !filled[p] && !occupied[p] {
					if min_size.setmin(near_vb[p.x as usize][p.z as usize].len()) {
						q = p;
					}
				}
			}
			if min_size < 100 {
				b.commands.push(Command::Fill(q - b.p));
				occupied[q] = true;
			}
		}
		macro_rules! score {
			($p:expr) => { {
				let p = $p;
				let mut ok = false;
				for q in p.near(r) {
					if q.y == y0 {
						if target[q] && !filled[q] && ground[q.x as usize][q.z as usize] && !occupied[q] {
							ok = true;
							break;
						}
					}
				}
				let mut score = 0.0;
				if !ok {
					score = -1.0;
				} else {
					for q in p.near(r) {
						if q.y == y0 {
							if target[q] && !filled[q] && !occupied[q] {
								if near_vb[q.x as usize][q.z as usize].len() == 0 {
									score += 1.0;
								} else {
									let mut ok = true;
									for &bid in &near_vb[q.x as usize][q.z as usize] {
										if near_bv[bid].len() <= 2 {
											ok = false;
										}
									}
									if ok {
										score += 0.1;
									}
								}
							}
						}
					}
				}
				score
			} };
		}
		// move 1
		for b in bots.iter_mut() {
			if b.commands.len() > t {
				continue;
			}
			let mut best = -1.0;
			let mut to = P::new(0, 0, 0);
			let mut com = Command::Wait;
			for (dx, dz, command) in one_step(b.p.x, b.p.z, r, |x, z| occupied[P::new(x, y0 + 1, z)]) {
				let p = b.p + P::new(dx, 0, dz);
				let score = score!(p);
				if best.setmax(score) {
					to = p;
					com = command;
				}
			}
			if best > 0.0 {
				for q in to.near(r) {
					if q.y == y0 && target[q] && !filled[q] && !occupied[q] {
						occupied[q] = true;
						break;
					}
				}
				b.commands.push(com);
				set_occupied(b.p, com, occupied);
			}
		}
		// move many
		for d in 1..r as i32 {
			let mut ok = true;
			for b in bots.iter_mut() {
				if b.commands.len() > t {
					continue;
				}
				let mut best = -1.0;
				let mut to = P::new(0, 0, 0);
				for dx in &[-d, d] {
					for dz in -d..=d {
						for r in 0..2 {
							let p = if r == 0 {
								P::new(b.p.x + dx, b.p.y, b.p.z + dz)
							} else {
								P::new(b.p.x + dz, b.p.y, b.p.z + dx)
							};
							let score = score!(p);
							if best.setmax(score) {
								to = p;
							}
						}
					}
				}
				if best > 0.0 {
					for q in to.near(r) {
						if q.y == y0 && target[q] && !filled[q] && !occupied[q] {
							occupied[q] = true;
							break;
						}
					}
					let mut min_dist = (b.p - to).mlen();
					let mut com = Command::Wait;
					for (dx, dz, command) in one_step(b.p.x, b.p.z, r, |x, z| occupied[P::new(x, y0 + 1, z)]) {
						let p = b.p + P::new(dx, 0, dz);
						let dist = (p - to).mlen();
						if min_dist.setmin(dist) {
							com = command;
						}
					}
					b.commands.push(com);
					set_occupied(b.p, com, occupied);
				} else {
					ok = false;
				}
			}
			if ok {
				break;
			}
		}
		// wait
		for b in bots.iter_mut() {
			if b.commands.len() == t {
				b.commands.push(Command::Wait);
			}
		}
		for bid in 0..nbots {
			for &p in &near_bv[bid] {
				near_vb[p.x as usize][p.z as usize].clear();
			}
		}
		eprintln!("{:?}", bots.iter().map(|b| b.commands.last().unwrap()).collect::<Vec<_>>());
		for b in bots.iter_mut() {
			match b.commands[t] {
				Command::SMove(d) => {
					b.p += d;
				},
				Command::LMove(d1, d2) => {
					b.p += d1 + d2;
				},
				Command::Fill(d) => {
					let p = b.p + d;
					assert!(p.y == y0);
					assert!(!filled[p]);
					filled[p] = true;
					rem -= 1;
					for q in p.adj(r) {
						if q.y == y0 && target[q] && !ground[q.x as usize][q.z as usize] {
							ground[q.x as usize][q.z as usize] = true;
						}
					}
				}
				_ => {
				}
			}
		}
		energy += 1;
		t += 1;
	}
	// output_layer(target, filled, &ground, &bots, y0);
	energy
}

fn target_bottom_up(target: &V3<bool>) -> V3<bool> {
	let r = target.len();
	let mut target2 = target.clone();
	let mut ground = mat![false; r; r; r];
	for x in 0..r {
		for z in 0..r {
			if target2[x][0][z] {
				ground[x][0][z] = true;
			}
		}
	}
	for y in 1..r {
		let mut stack = vec![];
		for x in 0..r {
			for z in 0..r {
				if target2[x][y][z] && ground[x][y - 1][z] {
					stack.push(P::new(x as i32, y as i32, z as i32));
				}
			}
		}
		loop {
			while let Some(p) = stack.pop() {
				for q in p.adj(r) {
					if q.y == y as i32 && target2[q] && !ground[q] {
						ground[q] = true;
						stack.push(q);
					}
				}
			}
			let mut max_d = -2;
			let mut q = P::new(0, 0, 0);
			for x in 0..r {
				for z in 0..r {
					if target2[x][y][z] && !ground[x][y][z] {
						let mut d = -1;
						for y2 in (0..y).rev() {
							if target2[x][y2][z] {
								d = y2 as i32;
								break;
							}
						}
						if max_d.setmax(d) {
							q = P::new(x as i32, y as i32, z as i32);
						}
					}
				}
			}
			if max_d == -2 {
				break;
			}
			eprintln!("support: {:?} : {}", q, y as i32 - max_d);
			ground[q] = true;
			stack.push(q);
			for y2 in (0..y).rev() {
				if target2[q.x as usize][y2][q.z as usize] {
					break;
				}
				target2[q.x as usize][y2][q.z as usize] = true;
			}
		}
	}
	target2
}

fn solve_bottom_up(target: &V3<bool>, nbots: usize) -> (i64, Vec<Command>) {
	let r = target.len();
	let target2 = target_bottom_up(target);
	let mut energy = 0;
	let mut init = vec![];
	for x in 0..r {
		for z in 0..r {
			if target2[x][0][z] {
				init.push(P::new(x as i32, 1, z as i32));
			}
		}
	}
	if init.len() >= nbots {
		let mut init2 = vec![];
		for i in 0..nbots {
			init2.push(init[i * init.len() / nbots]);
		}
		init = init2;
	} else {
		for x in 0..r {
			for z in 0..r {
				if init.len() < nbots && !target2[x][0][z] {
					init.push(P::new(x as i32, 1, z as i32));
				}
			}
		}
	}
	
	let mut filled = mat![false; r; r; r];
	let (bids, mut commands) = if FISSION {
		fission_to(&filled, &init)
	} else {
		((0..nbots).collect(), vec![])
	};
	
	let mut bots = vec![];
	for i in 0..nbots {
		bots.push(Bot { bid: bids[i], p: init[i], commands: vec![] });
	}
	bots.sort_by_key(|b| b.bid);
	for i in 0..nbots {
		bots[i].bid = i;
	}
	let mut occupied = InitV3::new(false, r);
	let mut max_r = 0;
	for x in 0..r {
		for y in 0..r {
			for z in 0..r {
				if target2[x][y][z] {
					max_r.setmax(y);
				}
			}
		}
	}
	for y in 0..=max_r {
		energy += fill_layer_bottom(&target2, &mut filled, &mut occupied, &mut bots, y as i32);
	}
	energy += destruct_support(&target, &mut filled, &mut bots);
	let t_max = bots.iter().map(|b| b.commands.len()).max().unwrap();
	for t in 0..t_max {
		for b in &bots {
			if b.commands.len() <= t {
				commands.push(Command::Wait);
			} else {
				commands.push(b.commands[t]);
			}
		}
	}
	if FISSION {
		// TODO: target2 -> target
		commands.extend(postproc::fusion_all(&target2, bots.iter().map(|b| b.p).collect()));
	}
	(energy, commands)
}

fn solve(target: &V3<bool>, nbots: usize) -> (i64, Vec<Command>) {
	let (min_score, min_commands) = solve_bottom_up(target, nbots);
	(min_score, min_commands)
}

fn main() {
	let file = std::env::args().nth(1).unwrap();
	let model = wata::read(&file);
	let target = model.filled;
	let mut min_score = i64::max_value();
	let mut min_commands = vec![];
	let nbots_list = if let Some(s) = std::env::args().nth(3) {
		vec![s.parse().unwrap()]
	} else {
		vec![40]
	};
	for &nbots in &nbots_list {
		let (score, commands) = solve(&target, nbots);
		if min_score.setmin(score) {
			min_commands = commands;
		}
	}
	for command in min_commands {
		println!("{}", command.to_string());
	}
}
