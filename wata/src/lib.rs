use std::io::Read;
use std::ops::*;

#[macro_export]
macro_rules! debug {
	($($v: expr),*) => {
		$(let _ = write!(::std::io::stderr(), "{} = {:?} ", stringify!($v), $v);)*
		let _ = writeln!(::std::io::stderr(), "@ {}:{}", file!(), line!());
	}
}
#[macro_export]
macro_rules! mat {
	($e:expr) => { $e };
	($e:expr; $d:expr $(; $ds:expr)*) => { vec![mat![$e $(; $ds)*]; $d] };
}
#[macro_export]
macro_rules! ok {
	($a:ident$([$i:expr])*.$f:ident()$(@$t:ident)*) => {
		$a$([$i])*.$f($($t),*)
	};
	($a:ident$([$i:expr])*.$f:ident($e:expr$(,$es:expr)*)$(@$t:ident)*) => { {
		let t = $e;
		ok!($a$([$i])*.$f($($es),*)$(@$t)*@t)
	} };
}

pub trait SetMin {
	fn setmin(&mut self, v: Self) -> bool;
}
impl<T> SetMin for T where T: PartialOrd {
	fn setmin(&mut self, v: T) -> bool {
		*self > v && { *self = v; true }
	}
}
pub trait SetMax {
	fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMax for T where T: PartialOrd {
	fn setmax(&mut self, v: T) -> bool {
		*self < v && { *self = v; true }
	}
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Command {
	Halt,
	Wait,
	SMove(P),
	LMove(P, P),
	FusionP(P),
	FusionS(P),
	Fission(P, usize),
	Fill(P),
}

impl ToString for Command {
	fn to_string(&self) -> String {
		match self {
			Command::Halt => {
				"HALT".to_owned()
			},
			Command::Wait => {
				"WAIT".to_owned()
			},
			Command::SMove(d) => {
				format!("SMOVE {} {}", if d.x != 0 { "x "} else if d.y != 0 { "y" } else { "z" }, d.mlen())
			},
			Command::LMove(d1, d2) => {
				format!("LMOVE {} {} {} {}", if d1.x != 0 { "x "} else if d1.y != 0 { "y" } else { "z" }, d1.mlen(),
											 if d2.x != 0 { "x "} else if d2.y != 0 { "y" } else { "z" }, d2.mlen())
			},
			Command::FusionP(p) => {
				format!("FUSIONP {} {} {}", p.x, p.y, p.z)
			},
			Command::FusionS(p) => {
				format!("FUSIONS {} {} {}", p.x, p.y, p.z)
			},
			Command::Fission(p, m) => {
				format!("FISSION {} {} {} {}", p.x, p.y, p.z, m)
			},
			Command::Fill(p) => {
				format!("FILL {} {} {}", p.x, p.y, p.z)
			},
		}
	}
}

pub type V3<T> = Vec<Vec<Vec<T>>>;

#[derive(Clone, Debug)]
pub struct Model {
	pub r: usize,
	pub filled: V3<bool>
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct P {
	pub x: i32,
	pub y: i32,
	pub z: i32,
}

impl P {
	pub fn mlen(&self) -> i32 {
		self.x.abs() + self.y.abs() + self.z.abs()
	}
}

pub const NEAR: [P; 18] = [
	P { x: -1, y: -1, z: 0 }, P { x: -1, y: 0, z: -1 }, P { x: -1, y: 0, z: 0 }, P { x: -1, y: 0, z: 1 }, P { x: -1, y: 1, z: 0 },
	P { x: 0, y: -1, z: -1 }, P { x: 0, y: -1, z: 0 }, P { x: 0, y: -1, z: 1 },
	P { x: 0, y: 0, z: -1 }, P { x: 0, y: 0, z: 1 },
	P { x: 0, y: 1, z: -1 }, P { x: 0, y: 1, z: 0 }, P { x: 0, y: 1, z: 1 },
	P { x: 1, y: -1, z: 0 }, P { x: 1, y: 0, z: -1 }, P { x: 1, y: 0, z: 0 }, P { x: 1, y: 0, z: 1 }, P { x: 1, y: 1, z: 0 }
];

pub const ADJ: [P; 6] = [
	P { x: -1, y: 0, z: 0 }, P { x: 1, y: 0, z: 0 },
	P { x: 0, y: -1, z: 0 }, P { x: 0, y: 1, z: 0 },
	P { x: 0, y: 0, z: -1 }, P { x: 0, y: 0, z: 1 },
];
impl P {
	pub fn new(x: i32, y: i32, z: i32) -> P {
		P { x, y, z }
	}
	pub fn is_valid(&self, r: usize) -> bool {
		let r = r as i32;
		0 <= self.x && self.x < r && 0 <= self.y && self.y < r && 0 <= self.z && self.z < r
	}
	pub fn near(&self, r: usize) -> Vec<P> {
		let mut near = vec![];
		for d in &NEAR {
			let q = self + d;
			if q.is_valid(r) {
				near.push(q);
			}
		}
		near
	}
	pub fn adj(&self, r: usize) -> Vec<P> {
		let mut adj = vec![];
		for d in &ADJ {
			let q = self + d;
			if d.is_valid(r) {
				adj.push(q);
			}
		}
		adj
	}
}

impl<'a> Add for &'a P {
	type Output = P;
	fn add(self, a: &P) -> P {
		P::new(self.x + a.x, self.y + a.y, self.z + a.z)
	}
}

impl<'a> Sub for &'a P {
	type Output = P;
	fn sub(self, a: &P) -> P {
		P::new(self.x - a.x, self.y - a.y, self.z - a.z)
	}
}

impl Mul<i32> for P {
	type Output = P;
	fn mul(self, a: i32) -> P {
		P::new(self.x * a, self.y * a, self.z * a)
	}
}

macro_rules! impl_all {
	($t:ident$(<$($g:ident),*>)*; $Op:ident:$op:ident:$Opa:ident:$opa:ident) => {
		impl<$($($g),*)*> $Op for $t$(<$($g),*>)* where for<'b> &'b $t$(<$($g),*>)*: $Op<Output = $t$(<$($g),*>)*> {
			type Output = $t$(<$($g),*>)*;
			#[inline]
			fn $op(self, a: $t$(<$($g),*>)*) -> $t$(<$($g),*>)* { (&self).$op(&a) }
		}
		impl<'a, $($($g),*)*> $Op<&'a $t$(<$($g),*>)*> for $t$(<$($g),*>)* where for<'b> &'b $t$(<$($g),*>)*: $Op<Output = $t$(<$($g),*>)*> {
			type Output = $t$(<$($g),*>)*;
			#[inline]
			fn $op(self, a: &$t$(<$($g),*>)*) -> $t$(<$($g),*>)* { (&self).$op(&a) }
		}
		impl<'a, $($($g),*)*> $Op<$t$(<$($g),*>)*> for &'a $t$(<$($g),*>)* where for<'b> &'b $t$(<$($g),*>)*: $Op<Output = $t$(<$($g),*>)*> {
			type Output = $t$(<$($g),*>)*;
			#[inline]
			fn $op(self, a: $t$(<$($g),*>)*) -> $t$(<$($g),*>)* { (&self).$op(&a) }
		}
		impl<$($($g),*)*> $Opa for $t$(<$($g),*>)* where for<'b> &'b $t$(<$($g),*>)*: $Op<Output = $t$(<$($g),*>)*> {
			#[inline]
			fn $opa(&mut self, a: $t$(<$($g),*>)*) { *self = (&*self).$op(&a) }
		}
	}
}

impl_all!(P; Add:add:AddAssign:add_assign);
impl_all!(P; Sub:sub:SubAssign:sub_assign);

macro_rules! impl_index {
	($($T: ty),*) => {
		$(
			impl Index<P> for V3<$T> {
				type Output = $T;
				fn index(&self, p: P) -> &$T {
					&self[p.x as usize][p.y as usize][p.z as usize]
				}
			}
			impl IndexMut<P> for V3<$T> {
				fn index_mut(&mut self, p: P) -> &mut $T {
					&mut self[p.x as usize][p.y as usize][p.z as usize]
				}
			}
		)*
	};
}

impl_index!(bool, usize);

pub const SEEDS: usize = 20;

pub fn read(path: &str) -> Model {
	let file = std::fs::File::open(path).unwrap();
	let mut reader = std::io::BufReader::new(file);
	let mut bytes = vec![];
	reader.read_to_end(&mut bytes).unwrap();
	let r = bytes[0] as usize;
	let mut filled = mat![false; r; r; r];
	for x in 0..r {
		for y in 0..r {
			for z in 0..r {
				let p = x * r * r + y * r + z;
				if bytes[1 + p / 8] >> (p % 8) & 1 != 0 {
					filled[x][y][z] = true;
				}
			}
		}
	}
	Model { r, filled }
}

pub mod bfs;
