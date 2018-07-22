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
	/// bs[i][j] := bid of the bot targetting supports[i][j]
	let mut bs: Vec<[Option<usize>; 2]> = vec![[None; 2]; supports.len()];
	/// working[i] := the target of the bod i
	let mut working: Vec<Option<(usize, usize)>> = vec![None; bots.len()];
	let mut finished = vec![false; supports.len()];
	let mut rem = supports.len();
	let mut t = bots[0].commands.len();
	let mut occupied = InitV3::new(false, r);
	let mut bpos = InitV3::new(!0, r);
	let mut ws = InitV3::new(false, r);
	let mut bfs = BFS::new(r);
	let mut energy = 0;
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
						bs[i][j] = None;
						working[b.bid] = None;
						if supports[i][0] == supports[i][1] {
							bs[i][j] = None;
						}
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
						ws.init();
						for b in bots.iter() {
							ws[b.p] = true;
						}
						for &bid in &free {
							bpos[bots[bid].p] = bid;
							ws[bots[bid].p] = false;
						}
						let mut starts = vec![];
						for t in supports[i][j].near(r) {
							if !filled[t] {
								starts.push(t);
							}
						}
						bfs.clear();
						if let Some(s) = bfs.bfs(|p| filled[p] || ws[p], &starts, |p| bpos[p] != !0) {
							let bid = bpos[s];
							assert!(bid != !0);
							free.remove(&bid);
							bs[i][j] = Some(bid);
							working[bid] = Some((i, j));
							bots[bid].commands.extend(bfs.restore_backward(s));
							if supports[i][0] == supports[i][1] {
								bs[i][j] = Some(bid);
							}
							break;
						}
					}
				}
			}
		}
		eprintln!("{:?}", working);
		for i in 0..bs.len() {
			if finished[i] {
				continue;
			}
			match bs[i] {
				[Some(a), Some(b)] if bots[a].commands.len() == t && bots[b].commands.len() == t => {
					if supports[i][0] == supports[i][1] {
						let ca = Command::Void(supports[i][0] - bots[a].p);
						bots[a].commands.push(ca);
					} else {
						let ca = Command::GVoid(supports[i][0] - bots[a].p, supports[i][1] - supports[i][0]);
						bots[a].commands.push(ca);
						let cb = Command::GVoid(supports[i][1] - bots[b].p, supports[i][0] - supports[i][1]);
						bots[b].commands.push(cb);
					}
					finished[i] = true;
					for y in supports[i][0].y..=supports[i][1].y {
						let mut p = supports[i][0];
						p.y = y;
						filled[p] = false;
					}
					working[a] = None;
					working[b] = None;
					bs[i] = [None; 2];
					rem -= 1;
				},
				_ => {
				}
			}
		}
		for b in bots.iter_mut() {
			if moved[b.bid] {
				 continue;
			}
			if b.commands.len() > t {
				if check_occupied(b.p, b.commands[t], &occupied) {
					set_occupied(b.p, b.commands[t], &mut occupied);
				} else {
					if let Some((i, j)) = working[b.bid] {
						bs[i][j] = None;
						working[b.bid] = None;
						if supports[i][0] == supports[i][1] {
							bs[i][j] = None;
						}
						b.commands.truncate(t);
						b.commands.push(Command::Wait);
					} else {
						assert!(false);
					}
				}
			} else {
				b.commands.push(Command::Wait);
			}
		}
		for b in bots.iter_mut() {
			match b.commands[t] {
				Command::SMove(d) => {
					b.p += d;
				},
				Command::LMove(d1, d2) => {
					b.p += d1 + d2;
				},
				_ => {
				}
			}
		}
		t += 1;
		energy += 1;
	}
	for b in bots.iter_mut() {
		b.commands.truncate(t);
	}
	energy
}

fn fill_layer<I: Fn(i32, i32) -> P, X: Fn(P) -> usize, Y: Fn(P) -> usize, Z: Fn(P) -> usize, G: Fn(usize, usize) -> bool>
			(target: &V3<bool>, filled: &mut V3<bool>, occupied: &mut InitV3<bool>, bots: &mut Vec<Bot>, dir: P,
				pos: I, get_x: X, get_y: Y, get_z: Z, is_grounded: G, y0: usize) -> i64 {
	let r = target.len();
	let nbots = bots.len();
	let mut energy = 0;
	let mut rem = 0;
	for x in 0..r {
		for z in 0..r {
			if target[pos(x as i32, z as i32)] {
				rem += 1;
			}
		}
	}
	let first = get_y(bots[0].p) != y0;
	if !first {
		for b in bots.iter_mut() {
			b.p -= dir;
			b.commands.push(Command::SMove(-dir));
		}
		energy += 1;
	}
	let mut ground = mat![false; r; r];
	for x in 0..r {
		for z in 0..r {
			if target[pos(x as i32, z as i32)] && (is_grounded(x, z) || !first && filled[pos(x as i32, z as i32) + dir]) {
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
				if get_y(p) == y0 && target[p] && !filled[p] {
					near_bv[b.bid].push(p);
					near_vb[get_x(p)][get_z(p)].push(b.bid);
				}
			}
		}
		// fill
		for b in bots.iter_mut() {
			let mut min_size = 100;
			let mut q = P::new(0, 0, 0);
			for p in b.p.near(r) {
				if get_y(p) == y0 && ground[get_x(p)][get_z(p)] && !filled[p] && !occupied[p] {
					if min_size.setmin(near_vb[get_x(p)][get_z(p)].len()) {
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
					if get_y(q) == y0 {
						if target[q] && !filled[q] && ground[get_x(q)][get_z(q)] && !occupied[q] {
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
						if get_y(q) == y0 {
							if target[q] && !filled[q] && !occupied[q] {
								if near_vb[get_x(q)][get_z(q)].len() == 0 {
									score += 1.0;
								} else {
									let mut ok = true;
									for &bid in &near_vb[get_x(q)][get_z(q)] {
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
		let dxz = |dx, dz| {
			let mut d = pos(dx, dz);
			if dir.x != 0 {
				d.x = 0;
			} else if dir.y != 0 {
				d.y = 0;
			} else {
				d.z = 0;
			}
			d
		};
		// move 1
		for b in bots.iter_mut() {
			if b.commands.len() > t {
				continue;
			}
			let mut best = -1.0;
			let mut to = P::new(0, 0, 0);
			let mut com = Command::Wait;
			for (dx, dz, command) in one_step(get_x(b.p) as i32, get_z(b.p) as i32, r, |x, z| occupied[pos(x, z) - dir]) {
				let p = b.p + dxz(dx, dz);
				let score = score!(p);
				if best.setmax(score) {
					to = p;
					com = match command {
						Command::SMove(d) => {
							Command::SMove(dxz(d.x, d.z))
						},
						Command::LMove(d1, d2) => {
							Command::LMove(dxz(d1.x, d1.z), dxz(d2.x, d2.z))
						},
						_ => {
							unreachable!()
						}
					};
				}
			}
			if best > 0.0 {
				for q in to.near(r) {
					if get_y(q) == y0 && target[q] && !filled[q] && !occupied[q] {
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
				for &dx in &[-d, d] {
					for dz in -d..=d {
						for r in 0..2 {
							let p = if r == 0 {
								b.p + dxz(dx, dz)
							} else {
								b.p + dxz(dz, dx)
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
						if get_y(q) == y0 && target[q] && !filled[q] && !occupied[q] {
							occupied[q] = true;
							break;
						}
					}
					let mut min_dist = (b.p - to).mlen();
					let mut com = Command::Wait;
					for (dx, dz, command) in one_step(get_x(b.p) as i32, get_z(b.p) as i32, r, |x, z| occupied[pos(x, z) - dir]) {
						let p = b.p + dxz(dx, dz);
						let dist = (p - to).mlen();
						if min_dist.setmin(dist) {
							com = match command {
								Command::SMove(d) => {
									Command::SMove(dxz(d.x, d.z))
								},
								Command::LMove(d1, d2) => {
									Command::LMove(dxz(d1.x, d1.z), dxz(d2.x, d2.z))
								},
								_ => {
									unreachable!()
								}
							};
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
				near_vb[get_x(p)][get_z(p)].clear();
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
					assert!(!filled[p]);
					filled[p] = true;
					rem -= 1;
					for q in p.adj(r) {
						if get_y(q) == y0 && target[q] && !ground[get_x(q)][get_z(q)] {
							ground[get_x(q)][get_z(q)] = true;
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

fn fill_layer_z(target: &V3<bool>, filled: &mut V3<bool>, occupied: &mut InitV3<bool>, bots: &mut Vec<Bot>, z0: i32, dir: i32) -> i64 {
	fill_layer(target, filled, occupied, bots, P::new(0, 0, dir), |x, y| P::new(x, y, z0), |p| p.x as usize, |p| p.z as usize, |p| p.y as usize, |_, y| y == 0, z0 as usize)
}

fn fill_layer_bottom(target: &V3<bool>, filled: &mut V3<bool>, occupied: &mut InitV3<bool>, bots: &mut Vec<Bot>, y0: i32) -> i64 {
	fill_layer(target, filled, occupied, bots, P::new(0, -1, 0), |x, z| P::new(x, y0, z), |p| p.x as usize, |p| p.y as usize, |p| p.z as usize, |_, _| y0 == 0, y0 as usize)
}

fn _fill_layer_bottom(target: &V3<bool>, filled: &mut V3<bool>, occupied: &mut InitV3<bool>, bots: &mut Vec<Bot>, y0: i32) -> i64 {
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
					ground[x][y][z] = true;
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

// [0, z0), [z0, r)
fn target_z(target: &V3<bool>, z0: usize) -> V3<bool> {
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
	for &dir in &[-1, 1] {
		for z in (if dir < 0 { (z0..r).collect::<Vec<_>>() } else { (0..z0).rev().collect() }) {
			if z != z0 && z != z0 - 1 {
				for x in 0..r {
					for y in 0..r {
						if target2[x][y][z] && ground[x][y][(z as i32 + dir) as usize] {
							ground[x][y][z] = true;
						}
					}
				}
			}
			let mut stack = vec![];
			for x in 0..r {
				for y in 0..r {
					if ground[x][y][z] {
						stack.push(P::new(x as i32, y as i32, z as i32));
					}
				}
			}
			loop {
				while let Some(p) = stack.pop() {
					for q in p.adj(r) {
						if q.z == z as i32 && target2[q] && !ground[q] {
							ground[q] = true;
							stack.push(q);
						}
					}
				}
				let mut max_d = -2;
				let mut q = P::new(0, 0, 0);
				for x in 0..r {
					for y in 0..r {
						if target2[x][y][z] && !ground[x][y][z] {
							let mut d = -1;
							for y2 in (0..y).rev() {
								if target2[x][y2][z] && ground[x][y2][z] {
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
				eprintln!("support: {:?} : {}", q, q.y as i32 - max_d);
				ground[q] = true;
				stack.push(q);
				for y2 in (0..q.y).rev() {
					if target2[q.x as usize][y2 as usize][q.z as usize] && ground[q.x as usize][y2 as usize][q.z as usize] {
						break;
					}
					target2[q.x as usize][y2 as usize][q.z as usize] = true;
				}
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
		// TODO
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
		commands.extend(postproc::fusion_all(&target, bots.iter().map(|b| b.p).collect()));
	}
	(energy, commands)
}

fn choose_z0(target: &V3<bool>) -> usize {
	let r = target.len();
	let mut total = 0;
	for x in 0..r {
		for y in 0..r {
			for z in 0..r {
				if target[x][y][z] {
					total += 1;
				}
			}
		}
	}
	let mut sub = 0;
	for z in 0..r {
		for x in 0..r {
			for y in 0..r {
				if target[x][y][z] {
					sub += 1;
				}
			}
		}
		if sub * 2 > total {
			return z;
		}
	}
	return r - 1;
}

fn solve_z(target: &V3<bool>, nbots: usize) -> (i64, Vec<Command>) {
	let r = target.len();
	let z0 = choose_z0(target);
	eprintln!("z0: {} / {}", z0, r);
	let target2 = target_z(target, z0);
	let mut init_all = vec![];
	let mut energy = 0;
	for &dir in &[-1, 1] {
		let mut init = vec![];
		let nbots = if dir < 0 {
			nbots / 2
		} else {
			nbots - nbots / 2
		};
		let z = if dir < 0 { z0 } else { z0 - 1 };
		for x in 0..r {
			for y in 0..r {
				if target2[x][y][z] {
					init.push(P::new(x as i32, y as i32, z as i32 - dir));
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
				for y in 0..r {
					if init.len() < nbots && !target2[x][y][z] {
						init.push(P::new(x as i32, y as i32, z as i32 - dir));
					}
				}
			}
		}
		init_all.extend(init);
	}
	let mut filled = mat![false; r; r; r];
	let (bids, mut commands) = if FISSION {
		fission_to(&filled, &init_all)
	} else {
		((0..nbots).collect(), vec![])
	};
	let mut bots_all = vec![];
	for &dir in &[-1, 1] {
		let mut bots = vec![];
		if dir < 0 {
			for i in 0..nbots / 2 {
				bots.push(Bot { bid: bids[i], p: init_all[i], commands: vec![] });
			}
		} else {
			for i in nbots / 2 .. nbots {
				bots.push(Bot { bid: bids[i], p: init_all[i], commands: vec![] });
			}
		}
		bots.sort_by_key(|b| b.bid);
		let mut bids = vec![];
		for i in 0..bots.len() {
			bids.push(bots[i].bid);
			bots[i].bid = i;
		}
		let mut occupied = InitV3::new(false, r);
		let mut max_d = 0;
		for d in 0.. {
			let z = if dir < 0 {
				z0 as i32 - dir * d
			} else {
				z0 as i32 - 1 - dir * d
			};
			if z < 0 || z >= r as i32 {
				max_d = d;
				break;
			}
			let mut ok = false;
			for x in 0..r {
				for y in 0..r {
					if target2[x][y][z as usize] {
						ok = true;
					}
				}
			}
			if !ok {
				max_d = d;
				break;
			}
		}
		for d in 0..max_d {
			let z = if dir < 0 {
				z0 as i32 - dir * d
			} else {
				z0 as i32 - 1 - dir * d
			};
			energy += fill_layer_z(&target2, &mut filled, &mut occupied, &mut bots, z, dir);
		}
		for i in 0..bots.len() {
			bots[i].bid = bids[i];
		}
		bots_all.extend(bots);
	}
	bots_all.sort_by_key(|b| b.bid);
	for i in 0..nbots {
		bots_all[i].bid = i;
	}
	let t_max = bots_all.iter().map(|b| b.commands.len()).max().unwrap();
	for b in bots_all.iter_mut() {
		while b.commands.len() < t_max {
			b.commands.push(Command::Wait);
		}
	}
	energy += destruct_support(&target, &mut filled, &mut bots_all);
	let t_max = bots_all.iter().map(|b| b.commands.len()).max().unwrap();
	for t in 0..t_max {
		for b in &bots_all {
			if b.commands.len() <= t {
				commands.push(Command::Wait);
			} else {
				commands.push(b.commands[t]);
			}
		}
	}
	if FISSION {
		commands.extend(postproc::fusion_all(&target, bots_all.iter().map(|b| b.p).collect()));
	}
	(energy, commands)
}

fn solve(target: &V3<bool>, nbots: usize, dir: &str) -> (i64, Vec<Command>) {
	match dir {
		"y" => {
			solve_bottom_up(target, nbots)
		},
		"z" => {
			solve_z(target, nbots)
		},
		"x" => {
			let r = target.len();
			let mut target2 = mat![false; r; r; r];
			for x in 0..r {
				for y in 0..r {
					for z in 0..r {
						target2[x][y][z] = target[z][y][x];
					}
				}
			}
			let (score, mut commands) = solve_z(&target2, nbots);
			let f = |p: P| P::new(p.z, p.y, p.x);
			for c in &mut commands {
				*c = match *c {
					Command::SMove(p) => Command::SMove(f(p)),
					Command::LMove(p1, p2) => Command::LMove(f(p1), f(p2)),
					Command::FusionP(p) => Command::FusionP(f(p)),
					Command::FusionS(p) => Command::FusionS(f(p)),
					Command::Fission(p, m) =>Command::Fission(f(p), m),
					Command::Fill(p) => Command::Fill(f(p)),
					Command::Void(p) => Command::Void(f(p)),
					Command::GFill(p1, p2) => Command::GFill(f(p1), f(p2)),
					Command::GVoid(p1, p2) => Command::GVoid(f(p1), f(p2)),
					c => c
				}
			}
			(score, commands)
		},
		_ => {
			panic!("unknown dir: {}", dir);
		}
	}
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
	let dir = if let Some(s) = std::env::args().nth(4) {
		s
	} else {
		"y".to_owned()
	};
	for &nbots in &nbots_list {
		let (score, commands) = solve(&target, nbots, &dir);
		if min_score.setmin(score) {
			min_commands = commands;
		}
	}
	for command in min_commands {
		println!("{}", command.to_string());
	}
}
