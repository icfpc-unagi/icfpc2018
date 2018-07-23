extern crate wata;
use wata::*;
use std::collections::*;

fn main() {
    assert_eq!(std::env::args().nth(1).unwrap(), ""); // I am destroy-only solver
    let file = std::env::args().nth(2).unwrap();
    let model = wata::read(&file);
    let r = model.r;
    let filled2 = xz::any_y(&model.filled);
    let mut filled2_fix = mat![false; r-2; r-2];
    for x in 1..r-1 {
        for z in 1..r-1 {
            filled2_fix[x-1][z-1] = filled2[x][z]
        }
    }
    for (bx_fix, bz_fix, small_fix) in xz::shrink(&filled2_fix, 30) {
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

        let mut bot_needed = mat![0; rx+1; rz+1];
        for ix in 0..rx {
            for iz in 0..rz {
                let t = small[ix][iz] as i32;
                for a in 0..2 {
                    for b in 0..2 {
                        bot_needed[ix+a][iz+b] += t;
                    }
                }
            }
        }
        let mut n_bot = 0;
        /*
        let mut bot_pos = BTreeMap::new();
        {
            let mut orz = false;
            for ix in 0..=rx {
                for iz in 0..=rz {
                    orz |= bot_needed[ix][iz] == 4;
                    if bot_needed[ix][iz] > 0 {
                        let bot_pos = if ix == 0 {
                            0
                        } else if ix == rx {
                            r - 1
                        } else {
                        }

                        n_bot += 1;
                    }
                }
            }
            if orz || n_bot > 20 {
                continue;
            }
        }
        */
    }
}

