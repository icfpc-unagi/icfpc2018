
// extern crate wata;

/*
完了:
- まずはなんでも良いから全部破壊する (R ≦ 150)
    - 方向固定: 上から
    - Harmonics: とりあえず常時on
    - bot: 6 * 6 固定
    - session: 1発
- まずは何でも良いから全部破壊する（全R）
    - マルチセッション

Backlog:
- Harmonicsを必要なときだけにする
- bounding boxを真面目にやる
- 方向: 全通り試す
- bot: 5 * 8 とか色々試すようにする
- 1回の面の消しをどういう順番でやるか全部試す
- Harmonicsのオンオフを、余りbotが居ればそいつがやるようにする

そのうち
- 余るような小さいやつだったら1箇所にbotを2つおく
- flip
*/

use std::cmp::{max, min};
use std::collections::*;
use super::super::bfs::*;
use super::super::*;

use super::structs::{Bot, CommandSet};
use super::harmonizer::Harmonizer;

struct App {
    model: Model,
    bots: Vec<Bot>,
    fission_commands: Vec<Command>,
    fusion_commands: Vec<Command>,
    command_sets: Vec<CommandSet>,
    harmonizer: Harmonizer,
}

const CELL_LENGTH: i32 = 30;

impl App {
    pub fn new(model: &Model) -> App {
        App {
            model: model.clone(),
            bots: vec![
                Bot {
                    bid: 0,
                    p: P::new(0, 0, 0)
                };
                1
            ],
            fission_commands: vec![],
            fusion_commands: vec![],
            command_sets: vec![],
            harmonizer: Harmonizer::new(model),
        }
    }

    //
    // Session (= x and z coordinates are fixed, destroy along y axis)
    //

    fn destroy_layer(&mut self, bot_grid: &Vec<Vec<usize>>) {
        let (n_bots_x, n_bots_z) = (bot_grid.len(), bot_grid[0].len());

        for parity_x in 0..2 {
            for parity_z in 0..2 {
                let current_step = self.command_sets.len();

                let mut cs = CommandSet::new(self.bots.len());

                let mut ix = parity_x;
                while ix + 1 < n_bots_x {
                    let mut iz = parity_z;
                    while iz + 1 < n_bots_z {
                        let bot4 = [
                            &self.bots[bot_grid[ix + 0][iz + 0]],
                            &self.bots[bot_grid[ix + 0][iz + 1]],
                            &self.bots[bot_grid[ix + 1][iz + 0]],
                            &self.bots[bot_grid[ix + 1][iz + 1]],
                        ];
                        cs.gvoid_below_layer(&bot4);
                        self.harmonizer.gvoid_below_layer(&bot4, current_step);
                        iz += 2;
                    }
                    ix += 2;
                }

                // If all-wait, then skip
                if cs.is_all_wait() {
                    continue;
                }
                self.command_sets.push(cs);
            }
        }
    }

    fn move_down(&mut self) {
        let dif = P::new(0, -1, 0);
        self.command_sets.push(CommandSet::new_uniform(
            self.bots.len(),
            Command::SMove(dif),
        ));
        for bot in self.bots.iter_mut() {
            bot.p += dif;
        }
    }

    fn destroy_session(&mut self, bot_grid: &Vec<Vec<usize>>) {
        let n_bots_x = bot_grid.len();
        let n_bots_z = bot_grid[0].len();

        let p0 = self.bots[bot_grid[0][0]].p;
        for ix in 0..n_bots_x {
            for iz in 0..n_bots_z {
                let p = &self.bots[bot_grid[ix][iz]].p;
                assert_eq!(p.y, p0.y);
                assert_eq!(
                    p.x,
                    min(p0.x + (ix as i32) * CELL_LENGTH, (self.model.r - 1) as i32)
                );
                assert_eq!(
                    p.z,
                    min(p0.z + (iz as i32) * CELL_LENGTH, (self.model.r - 1) as i32)
                );
            }
        }

        loop {
            let p = self.bots[bot_grid[0][0]].p;
            if p.y == 0 {
                break;
            }

            self.destroy_layer(bot_grid);

            if p.y == 1 {
                break;
            }
            self.move_down();
        }
    }

    //
    // Pre and post processing
    //

    fn fission(&mut self, n_bots_x: usize, n_bots_z: usize) -> Vec<Vec<usize>> {
        let r = self.model.r as i32;
        let n_bots = n_bots_x * n_bots_z;

        // Positions
        let p0 = P::new(0, (r as i32) - 1, 0); // TODO: better starting point
        let ps: Vec<P> = (0..n_bots)
            .map(|i| {
                let ix = (i / n_bots_z) as i32;
                let iz = (i % n_bots_z) as i32;
                p0 + P::new(
                    min(CELL_LENGTH * ix, r - 1),
                    0,
                    min(CELL_LENGTH * iz, r - 1),
                )
            })
            .collect();

        let (ord, cmds) = fission_to(&self.model.filled, &ps);
        self.fission_commands = cmds;
        // let ord: Vec<usize> = (1..(n_bots + 1)).collect();
        eprintln!("{:?}", ord);

        self.bots = (0..n_bots)
            .map(|bid| {
                Bot {
                    bid,
                    p: P::new(-1, -1, -1), // Dummy
                }
            })
            .collect();
        for (&i, &p) in ord.iter().zip(ps.iter()) {
            self.bots[i - 1].p = p; // ord is 1-indexed
        }

        let bot_grid = (0..n_bots_x)
            .map(|ix| {
                (0..n_bots_z)
                    .map(|iz| {
                        ord[ix * n_bots_z + iz] - 1 // ord is 1-indexed
                    })
                    .collect()
            })
            .collect();
        return bot_grid;
    }

    fn fusion(&mut self) {
        let r = self.model.r;
        self.fusion_commands = postproc::fusion_all(
            &mat![false; r; r; r],
            self.bots.iter().map(|b| b.p).collect(),
        )
    }

    fn move_to_next_session(&mut self, mut p_diff: P) {
        let zero = P::new(0, 0, 0);

        let minmax5 = |x| max(min(x, 5), -5);
        let minmax15 = |x| max(min(x, 15), -15);

        eprintln!("{:?}", p_diff);
        while p_diff != zero {
            assert!(p_diff.y >= 0);
            let cmd;
            let p_move;

            if p_diff.y > 5 || (p_diff.x, p_diff.z) == (0, 0) {
                p_move = P::new(0, min(p_diff.y, 15), 0);
                cmd = Command::SMove(p_move);
            } else if p_diff.y > 0 {
                let p_move1 = P::new(0, p_diff.y, 0);
                let p_move2 = (if p_diff.x != 0 {
                    P::new(minmax5(p_diff.x), 0, 0)
                } else {
                    P::new(0, 0, minmax5(p_diff.z))
                });
                p_move = p_move1 + p_move2;
                cmd = Command::LMove(p_move1, p_move2)
            } else {
                if p_diff.x.abs() > 5 || p_diff.z == 0 {
                    p_move = P::new(minmax15(p_diff.x), 0, 0);
                    cmd = Command::SMove(p_move);
                } else if p_diff.z.abs() > 5 || p_diff.x == 0 {
                    p_move = P::new(0, 0, minmax15(p_diff.z));
                    cmd = Command::SMove(p_move);
                } else {
                    let p_move1 = P::new(p_diff.x, 0, 0);
                    let p_move2 = P::new(0, 0, p_diff.z);
                    p_move = p_move1 + p_move2;
                    cmd = Command::LMove(p_move1, p_move2);
                }
            }

            p_diff -= p_move;
            for bot in self.bots.iter_mut() {
                bot.p += p_move;
            }

            eprintln!("{:?} {:?} {:?}", p_move, p_diff, cmd);
            self.command_sets
                .push(CommandSet::new_uniform(self.bots.len(), cmd));
        }
    }

    fn harmonize_all(&mut self) {
        // 常時harmonizeオン
        let n_bots = self.bots.len();

        // Turn on harmonics
        if self.command_sets.first_mut().unwrap().is_all_busy() {
            self.command_sets.insert(0, CommandSet::new(n_bots));
        }
        self.command_sets.first_mut().unwrap().flip_by_somebody();

        // Turn off harmonics
        if self.command_sets.last_mut().unwrap().is_all_busy() {
            self.command_sets.push(CommandSet::new(n_bots));
        }
        self.command_sets.last_mut().unwrap().flip_by_somebody();
    }

    fn harmonize(&mut self) {
        // 賢いやつ
        let n_bots = self.bots.len();
        let n_steps = self.command_sets.len();
        let harmony_required = self.harmonizer.compute_harmony_requirement(n_steps);

        let mut index: usize = 0;
        for step in 0..n_steps {
            let crr_harmony = harmony_required[step];
            if step == 0 {
                assert_eq!(crr_harmony, false)
            }

            let nxt_harmony = if step + 1 < n_steps {
                harmony_required[step + 1]
            } else {
                false
            };

            if crr_harmony != nxt_harmony {
                eprintln!("Harmony flip: {} ({})", nxt_harmony, step);

                // Need flip!
                if self.command_sets[index].is_all_busy() {
                    self.command_sets.insert(index, CommandSet::new(n_bots));
                    self.command_sets[index].flip_by_somebody();
                    index += 1;
                } else {
                    self.command_sets[index].flip_by_somebody();
                }
            }
            index += 1;
        }

        eprintln!(
            "Harmony required steps: {} / {}",
            harmony_required.iter().filter(|b| **b).count(),
            harmony_required.len()
        );
    }

    //
    // Main
    //

    fn get_trace(&self) -> Vec<Command> {
        let mut all: Vec<Command> = vec![];
        for command in self.fission_commands.iter() {
            all.push(*command);
        }
        for command_set in self.command_sets.iter() {
            for command in command_set.commands.iter() {
                all.push(*command);
            }
        }
        for command in self.fusion_commands.iter() {
            all.push(*command)
        }
        return all;
    }

    pub fn main(&mut self) {
        // TODO: use a bounding box
        let r = self.model.r as i32;

        // TODO: hoge
        let n_bots_x = min(6, ((r as usize) - 1) / (CELL_LENGTH as usize) + 2);
        let n_bots_z = n_bots_x;
        let n_bots = n_bots_x * n_bots_z;
        let bot_grid = self.fission(n_bots_x, n_bots_z);
        eprintln!(
            "R: {}, Bot grid: {} X {} ({:?})",
            r, n_bots_x, n_bots_z, bot_grid
        );

        let session_x_size = min(r as i32, ((n_bots_x - 1) as i32) * CELL_LENGTH + 1);
        let session_z_size = min(r as i32, ((n_bots_z - 1) as i32) * CELL_LENGTH + 1);

        // TODO: more efficient way to schedule the order of sessions
        // TODO: don't use the same size for two sessions?
        let mut ix = 0;
        while ix * session_x_size < r {
            let session_x_offset = min(ix * session_x_size, r - session_x_size);

            let mut iz = 0;
            while iz * session_z_size < r {
                let session_z_offset = min(iz * session_z_size, r - session_z_size);

                if (session_x_offset, session_z_offset) != (0, 0) {
                    let p0_crr = self.bots[bot_grid[0][0]].p;
                    let p0_nxt = P::new(session_x_offset, r - 1, session_z_offset);
                    // TODO: r - 1じゃなくてちゃんとy座標をする

                    eprintln!(
                        "r={}, session_x_size={}, {}",
                        r, session_x_size, session_z_size
                    );
                    eprintln!("Session: {:?} -> {:?}", p0_crr, p0_nxt);

                    self.move_to_next_session(p0_nxt - p0_crr);
                }

                self.destroy_session(&bot_grid);

                iz += 1;
            }
            ix += 1;
        }

        self.harmonize();

        for (i, command_set) in self.command_sets.iter().enumerate() {
            // println!("[ STEP {} ]", i);
            // command_set.emit();
        }

        self.fusion();
    }
}

//
// Easy interface
//
pub fn destroy_large(model: Model) -> Vec<Command> {
    let mut app = App::new(&model);
    app.main();
    return app.get_trace();
}