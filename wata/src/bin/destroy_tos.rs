extern crate wata;
use wata::*;
use std::collections::*;

fn main() {
    assert_eq!(std::env::args().nth(1).unwrap(), ""); // I am destroy-only solver
    let file = std::env::args().nth(2).unwrap();
    let model = wata::read(&file);
    let r = model.r;
    let mut y_max = 0;
    for y in 0..r {
        for x in 0..r {
            for z in 0..r {
                if model.filled[x][y][z] {
                    y_max = y;
                }
            }
        }
    }
    let filled2 = xz::any_y(&model.filled);
    let mut filled2_fix = mat![false; r-2; r-2];
    for x in 1..r-1 {
        for z in 1..r-1 {
            filled2_fix[x-1][z-1] = filled2[x][z]
        }
    }
    for (bx_fix, bz_fix, small_fix) in xz::shrink(&filled2_fix, 30) {
        {
            let mut orz = false;

            for (b0, b1) in bx_fix.iter().zip(bx_fix[1..].iter()) {
                orz |= b1 - b0 <= 1;
            }
            for (b0, b1) in bz_fix.iter().zip(bz_fix[1..].iter()) {
                orz |= b1 - b0 <= 1;
            }
            /*
            if bx[ix+1] - bx[ix] <= 1 || bz[iz+1] - bz[iz] <= 1 {
                eprintln!("orz {} {}", bx[ix+1] - bx[ix], bz[iz+1] - bz[iz]);
                eprintln!("{:?} {}", bx, ix);
                orz = true;
            }
            */
            if orz {
                continue;
            }
        }
        /*
        eprintln!("({}, {})", bx, bz);
        for line in small.iter() {
            for &f in line.iter() {
                eprint!("{}", if f { "#" } else { "." });
            }
            eprintln!("");
        }
        */
        let mut bx = Vec::new();
        bx.push(0);
        bx.append(&mut bx_fix.iter().map(|&t| t+1).collect());
        bx.push(r);

        let mut bz = Vec::new();
        bz.push(0);
        bz.append(&mut bz_fix.iter().map(|&t| t+1).collect());
        bz.push(r);

        let rx = bx.len()-1;
        let rz = bz.len()-1;

        let mut small = mat![false; rx; rz];
        for ix in 1..rx-1 {
            for iz in 1..rz-1 {
                small[ix][iz] = small_fix[ix-1][iz-1];
            }
        }

        eprintln!("({:?}, {:?})", bx, bz);
        for line in small.iter() {
            for &f in line.iter() {
                eprint!("{}", if f { "#" } else { "." });
            }
            eprintln!("");
        }

        let mut bot_xz = BTreeMap::new();
        {
            let mut xz_tmp = BTreeMap::new();
            for ix in 1..rx {
                for iz in 1..rz {
                    for a in 0..2 {
                        for b in 0..2 {
                            if !small[ix-a][iz-b] {
                                xz_tmp.insert(
                                    (ix, iz),
                                    P::new((bx[ix]-a) as i32, 0, (bz[iz]-b) as i32));
                            }
                        }
                    }
                }
            }
            let mut orz = false;
            for ix in 1..rx {
                for iz in 1..rz {
                    let mut cnt = 0;
                    for a in 0..2 {
                        for b in 0..2 {
                            let t = small[ix-a][iz-b];
                            cnt += t as i32;
                        }
                    }
                    if cnt == 4 {
                        orz = true;
                    } else if cnt > 0 {
                        bot_xz.insert((ix, iz), xz_tmp[&(ix, iz)]);
                    }
                }
            }
            if orz || bot_xz.len() > 20 {
                continue;
            }
        }
        eprintln!("ok {:?}", bot_xz);

        {
            let xz_set: BTreeSet<_> = bot_xz.values().cloned().collect();
            if xz_set.len() < bot_xz.len() {
                continue;
            }
        }

        let mut y_high = 1.max(y_max as i32);
        let mut y_low = 0.max(y_high - 30 + 1);
        let mut bot_ps = Vec::new();
        bot_ps.append(&mut bot_xz.values().map(|&xz: &P| xz + P::new(0, y_low, 0)).collect());
        bot_ps.append(&mut bot_xz.values().map(|&xz: &P| xz + P::new(0, y_high, 0)).collect());

        let (bids, fission_cmds) = fission_to(&model.filled, &bot_ps);
        // let mut bids_low = BTreeMap::from_iter(bot_xz.keys().zip(bids[..bot_xz.len()].iter()));
        let mut bids_low = BTreeMap::new();
        for (ixz, bid) in bot_xz.keys().zip(bids[..bot_xz.len()].iter()) {
            bids_low.insert(ixz, bid);
        }
        let mut bids_high = BTreeMap::new();
        for (ixz, bid) in bot_xz.keys().zip(bids[bot_xz.len()..].iter()) {
            bids_high.insert(ixz, bid);
        }
        eprintln!("{:?}", bids);
        eprintln!("{:?}", bids_low);
        eprintln!("{:?}", bids_high);

        let mut sorted_bids = bids.clone();
        sorted_bids.sort();
        let sorted_bids = sorted_bids;

        let mut main_cmds = Vec::new();
        {
            let mut cmds = Vec::new();
            cmds.push(Command::Flip);
            for _ in 1..bids.len() {
                cmds.push(Command::Wait);
            }
            main_cmds.append(&mut cmds);
        }
        loop {
            for ix in 1..rx {
                for iz in 1..rz {
                    if small[ix][iz] {
                        let bx2 = [bx[ix] as i32, bx[ix+1] as i32 - 1];
                        // let by2 = [y_low, y_high];
                        let bz2 = [bz[iz] as i32, bz[iz+1] as i32 - 1];
                        let mut cmds = BTreeMap::new();
                        for a in 0..2 {
                            for b in 0..2 {
                                let nd = P::new(bx2[a], 0, bz2[b]) - bot_xz[&(ix+a, iz+b)];
                                {
                                    let bid = bids_low[&(ix+a, iz+b)];
                                    let fd = P::new(bx2[1-a] - bx2[a], y_high - y_low, bz2[1-b] - bz2[b]);
                                    cmds.insert(bid, Command::GVoid(nd, fd));
                                }
                                {
                                    let bid = bids_high[&(ix+a, iz+b)];
                                    let fd = P::new(bx2[1-a] - bx2[a], y_low - y_high, bz2[1-b] - bz2[b]);
                                    cmds.insert(bid, Command::GVoid(nd, fd));
                                }
                            }
                        }
                        eprintln!("{:?}", cmds);
                        let mut cmds = sorted_bids.iter().map(|bid| cmds.get(&bid).unwrap_or(&Command::Wait)).cloned().collect();
                        eprintln!("{:?}", cmds);
                        main_cmds.append(&mut cmds);
                    }
                }
            }
            if y_low == 0 {
                break;
            }
            let new_y_low = 0.max(y_low - 30);
            let mut y_down = y_low - new_y_low;
            y_low -= y_down;
            y_high -= y_down;
            while y_down > 0 {
                let dy = -(15.min(y_down));
                let mut cmds = vec![Command::SMove(P::new(0, dy, 0)); sorted_bids.len()];
                main_cmds.append(&mut cmds);
                y_down += dy;
            }
        }
        {
            let mut cmds = Vec::new();
            cmds.push(Command::Flip);
            for _ in 1..bids.len() {
                cmds.push(Command::Wait);
            }
            main_cmds.append(&mut cmds);
        }

        let mut positions = BTreeMap::new();
        for ixz in bot_xz.keys() {
            positions.insert(bids_low[ixz], bot_xz[ixz] + P::new(0, y_low, 0));
            positions.insert(bids_high[ixz], bot_xz[ixz] + P::new(0, y_high, 0));
        }
        let positions = positions.values().cloned().collect();
        let fusion_cmds = postproc::fusion_all(&mat![false; r; r; r], positions);
        eprintln!("{} {} {}", fission_cmds.len(), main_cmds.len(), fusion_cmds.len());
        for c in fission_cmds {
            println!("{}", c.to_string());
        }
        for c in main_cmds {
            println!("{}", c.to_string());
        }
        for c in fusion_cmds {
            println!("{}", c.to_string());
        }
        return;
    }
}

