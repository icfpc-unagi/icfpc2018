#![allow(unused)]
use *;


fn fusion_all(matrix: V3<bool>, positions: Vec<P>) {
    let r = matrix.len();
    let mut cmdss = Vec::new();
    {
        let filled_func = |p: P| { matrix[p] };
        let goal_func = |p: P| { p.x == 0 && p.y == 0 && p.z == 0 };
        for &pos in positions.iter() {
            let mut bfs = bfs::BFS::new(r);
            let ret = bfs.bfs(filled_func, &vec![pos], goal_func);
            eprintln!("{:?}", ret);
            let cmds = bfs.restore(ret.unwrap());
            cmdss.push(cmds);
        }
        eprintln!("{:?}", cmdss);
    }

    let mut sim = sim::SimState::from_positions(matrix, positions);
}
