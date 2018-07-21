#![allow(unused)]
use *;
use std::collections::*;


pub fn fusion_all(matrix: V3<bool>, positions: Vec<P>) -> Vec<Command> {
    let mut return_cmds = Vec::new();
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

    let mut positions = positions;

    //let mut sim = sim::SimState::from_positions(matrix, positions);
    let mut occupied = InitV3::new(false, r);
    loop {
        occupied.init();
        /*
        let mut step_cmds = Vec::new();
        for cmds in cmdss.iter_mut() {
            step_cmds.push(cmds.pop_front().unwrap_or(Command::Wait));
        }
        if step_cmds.iter().all(|&v| v == Command::Wait) {
            break;
        }
        */

        for &pos in positions.iter() {
            occupied[pos] = true;
        }

        let mut all_orz = true;
        for (mut pos, mut cmds) in positions.iter_mut().zip(cmdss.iter_mut()) {
            let cmd = cmds.pop_front().unwrap_or(Command::Wait);
            let mut orz = false;
            for (p, cmd_done, cmd_remain) in path(*pos, cmd) {
                if occupied[p] {
                    cmds.push_front(cmd_remain);
                    return_cmds.push(cmd_done);
                    orz = true;
                    break;
                }
                occupied[p] = true;
                *pos = p;
            }
            if !orz {
                return_cmds.push(cmd);
                all_orz = false;
            }
        }
        if all_orz {
            break;
        }
    }

    let n = positions.len();
    let mut sorted_pos = positions.clone();
    sorted_pos.sort_by_key(|p| p.mlen());
    for &pos in sorted_pos[1..n].iter().rev() {
        eprintln!("{:?}", pos);
        // these bid_* are not true but positions are sorted by true bid
        let bid_s = positions.iter().position(|&p| p == pos).unwrap();
        let bid_p = positions.iter().position(|&p| (p - pos).mlen() == 1).unwrap();
        let pos_p = positions[bid_p];
        let mut cmds = vec![Command::Wait; positions.len()];
        cmds[bid_p] = Command::FusionP(pos - pos_p);
        cmds[bid_s] = Command::FusionS(pos_p - pos);
        return_cmds.append(&mut cmds);
        positions.remove(bid_s);
    }

    return_cmds
}


fn path(mut p: P, mut cmd: Command) -> Vec<(P, Command, Command)> {
    // (next_pos, cmd_done, cmd_remain)
    let mut ret = Vec::new();
    let mut cmd_done = Command::Wait;
    while let Command::LMove(d1, d2) = cmd {
        let v = d1 / d1.mlen();
        p += v;
        ret.push((p, cmd_done, cmd));
        let d1 = d1 - v;
        cmd = if d1.mlen() == 0 {
            Command::SMove(d2)
        } else {
            Command::LMove(d1, d2)
        };
        cmd_done = cmd_push(cmd_done, v);
    }
    while let Command::SMove(d1) = cmd {
        let v = d1 / d1.mlen();
        p += v;
        ret.push((p, cmd_done, cmd));
        let d1 = d1 - v;
        cmd = if d1.mlen() == 0 {
            Command::Wait
        } else {
            Command::SMove(d1)
        };
        cmd_done = cmd_push(cmd_done, v);
    }
    ret
}


fn cmd_push(cmd: Command, d: P) -> Command {
    match cmd {
        Command::Wait => Command::SMove(d),
        Command::SMove(d1) => if d == d1/d1.mlen() {
            Command::SMove(d1 + d)
        } else {
            Command::LMove(d1, d)
        },
        Command::LMove(d1, d2) => Command::LMove(d1, d2 + d),
        _ => panic!()
    }
}
