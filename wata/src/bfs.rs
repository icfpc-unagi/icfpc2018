use ::*;

pub struct BFS {
	pub cost: V3<usize>,
}

impl BFS {
	pub fn new(r: usize) -> BFS {
		BFS { cost: mat![0; r; r; r] }
	}
	pub fn bfs<G: FnMut(P) -> bool>(filled: &V3<bool>, goal: G) -> Option<P> {
		unimplemented!()
	}
	pub fn restore(t: P) -> Vec<Command> {
		unimplemented!()
	}
}

