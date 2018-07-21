#![allow(unused)]
use *;
use std::collections::*;


fn fusion_all(matrix: V3<bool>, positions: Vec<P>) {
    let r = matrix.len();
    let mut cmdss: Vec<VecDeque<Command>> = Vec::new();
    {
        let filled_func = |p: P| { matrix[p] };
        let goal_func = |p: P| { p.x == 0 && p.y == 0 && p.z == 0 };
        for &pos in positions.iter() {
            let mut bfs = bfs::BFS::new(r);
            let ret = bfs.bfs(filled_func, &vec![pos], goal_func);
            eprintln!("{:?}", ret);
            let cmds = bfs.restore(ret.unwrap());
            cmdss.push(cmds.into_iter().collect());
        }
        eprintln!("{:?}", cmdss);
    }

    let mut sim = sim::SimState::from_positions(matrix, positions);
    loop {
        let mut step_cmds = Vec::new();
        for cmds in cmdss.iter_mut() {
            step_cmds.push(
                match cmds.pop_front() {
                    None => Command::Wait,
                    Some(cmd) => cmd
                }
            );
        }
        if step_cmds.iter().all(|&v| v == Command::Wait) {
            break;
        }
        sim.step_approx(step_cmds);
    }
}
