#![allow(unused)]
use *;
use std::collections::*;


pub fn fusion_all(matrix: &V3<bool>, positions: Vec<P>) -> Vec<Command> {
    let mut return_cmds = Vec::new();
    let r = matrix.len();
    eprintln!("{:?}", r);
    for x in 0..r {
        for y in 0..r {
            for z in 0..r {
                if x == 0 || x + 1 == r || y + 1 == r || z == 0 || z + 1 == r {
                    assert!(!matrix[x][y][z]);
                }
            }
        }
    }
    let mut cmdss: Vec<VecDeque<Command>> = Vec::new();
    let mut bfs = bfs::BFS::new(r);
    {
        let filled_func = |p: P| { matrix[p] };
        let goal_func = |p: P| { p.x == 0 && p.y == 0 && p.z == 0 };
        for &pos in positions.iter() {
            let ret = bfs.bfs(filled_func, &vec![pos], goal_func);
            eprintln!("{:?} -> {:?}", pos, ret);
            if let None = ret {
                for &t in bfs.touched.iter() {
                    eprintln!("{:?}", t);
                }
            }
            let cmds = bfs.restore(ret.unwrap());
            bfs.clear();
            cmdss.push(cmds.into_iter().collect());
        }
        eprintln!("{:?}", cmdss);
    }

    let mut positions = positions;

    let mut occupied = InitV3::new(false, r);
    // while positions.len() > 1 && positions[0].mlen() != 0 {
    loop {
        occupied.init();

        let mut step_cmds = Vec::new();

        for &pos in positions.iter() {
            occupied[pos] = true;
        }

        eprintln!("{:?}", positions);
        for (pos, mut cmds) in positions.iter_mut().zip(cmdss.iter_mut()) {
            let cmd = cmds.pop_front().unwrap_or(Command::Wait);
            let mut orz = false;
            for (p, cmd_done, cmd_remain) in path(*pos, cmd) {
                if occupied[p] {
                    cmds.push_front(cmd_remain);
                    step_cmds.push(cmd_done);
                    orz = true;
                    break;
                }
                occupied[p] = true;
                *pos = p;
            }
            if !orz {
                step_cmds.push(cmd);
            }
        }

        let mut remove_bids = Vec::new();

        let mut waiting_pos = BTreeSet::new();
        for (i, &pos) in positions.iter().enumerate() {
            if step_cmds[i] == Command::Wait {
                waiting_pos.insert(pos);
            }
        }
        while let Some((p1, p2)) = pop_near_pair(&mut waiting_pos) {
            waiting_pos.remove(&p1);
            waiting_pos.remove(&p2);
            // these bid* are not true but positions are sorted by true bid
            let bid1 = positions.iter().position(|&p| p == p1).unwrap();
            let bid2 = positions.iter().position(|&p| p == p2).unwrap();
            step_cmds[bid1] = Command::FusionP(p2 - p1);
            step_cmds[bid2] = Command::FusionS(p1 - p2);
            remove_bids.push(bid2);
        }

        if step_cmds.iter().all(|&cmd| cmd == Command::Wait) {
            break;
        }
        return_cmds.append(&mut step_cmds);

        remove_bids.sort();
        for bid in remove_bids.into_iter().rev() {
            positions.remove(bid);
            cmdss.remove(bid);
        }
    }
    return_cmds.push(Command::Halt);

    return_cmds
}


fn pop_near_pair(mut poss: &mut BTreeSet<P>) -> Option<(P, P)> {
    match get_near_pair(&poss) {
        Some((p1, p2)) => {
            poss.remove(&p1);
            poss.remove(&p2);
            if p1.mlen() <= p2.mlen() {
                Some((p1, p2))
            } else {
                Some((p2, p1))
            }
        },
        None => None,
    }
}


fn get_near_pair(poss: &BTreeSet<P>) -> Option<(P, P)> {
    for &p1 in poss.iter() {
        for p2 in p1.near(9999) {
            if poss.contains(&p2) {
                return Some((p1, p2));
            }
        }
    }
    return None;
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
