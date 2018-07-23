use std::cmp::{max, min};
use std::collections::*;
use super::super::bfs::*;
use super::super::*;

use super::structs::{Bot, CommandSet};
use super::harmonizer::Harmonizer;
use super::util;

const MAX_N_BOTS: usize = 40;
const DEFAULT_MAX_N_BOTS_X: i32 = 6;
const DEFAULT_MAX_N_BOTS_Z: i32 = 6;

pub struct App {
    pub model: Model,
    pub bots: Vec<Bot>,
    pub fission_commands: Vec<Command>,
    pub fusion_commands: Vec<Command>,
    pub command_sets: Vec<CommandSet>,
    pub harmonizer: Harmonizer,
    // Hoge
    pub bot_grid_relps: Vec<Vec<P>>,
    pub bot_grid_bids: Vec<Vec<usize>>,
    pub session_absps: Vec<P>,
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
            bot_grid_relps: vec![vec![]],
            bot_grid_bids: vec![vec![]],
            session_absps: vec![],
        }
    }

    //
    // Session (= x and z coordinates are fixed, destroy along y axis)
    //

    fn destroy_layer(&mut self) {
        let bot_grid = &self.bot_grid_bids;
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
                        // eprintln!("DESTROY: {:?} {:?}", bot4[0].p, bot4[3].p);

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

    fn destroy_session(&mut self) {
        let (n_bots_x, n_bots_z) = (self.bot_grid_relps.len(), self.bot_grid_relps[0].len());

        // Confirm the bot formation
        let p0 = self.bots[self.bot_grid_bids[0][0]].p;
        for ix in 0..n_bots_x {
            for iz in 0..n_bots_z {
                let p = self.bots[self.bot_grid_bids[ix][iz]].p;
                assert_eq!(p.y, p0.y);
                assert_eq!(p, p0 + self.bot_grid_relps[ix][iz]);
            }
        }

        // Just go down
        loop {
            let p = self.bots[self.bot_grid_bids[0][0]].p;
            // eprintln!("{:?}", p);
            if p.y == 0 {
                break;
            }

            self.destroy_layer();

            if p.y == 1 {
                break;
            }
            self.move_down();
        }
    }

    //
    // Pre and post processing
    //

    pub fn fusion(&mut self) {
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
        // 常時harmonizeオン（デバッグ用）
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

    pub fn harmonize(&mut self) {
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
                    let insert_index;
                    if nxt_harmony == true {
                        insert_index = index;
                    } else {
                        insert_index = index + 1;
                    }
                    self.command_sets.insert(insert_index, CommandSet::new(n_bots));
                    self.command_sets[insert_index].flip_by_somebody();
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
    // Utils
    //

    pub fn get_trace(&self) -> Vec<Command> {
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

    pub fn get_bounding_box_lengths(&self) -> (i32, i32) {
        let bb = util::get_bounding_box(&self.model.filled);
        let bb_length_x = (bb.1.x - bb.0.x) + 1;
        let bb_length_z = (bb.1.z - bb.0.z) + 1;
        return (bb_length_x, bb_length_z);
    }

    pub fn get_bot_grid_total_lengths(&self) -> (i32, i32) {
        let bg = &self.bot_grid_relps;
        let p = bg[bg.len() - 1][bg[0].len() - 1];
        return (p.x + 1, p.z + 1); // NOTE: be careful about this +1 --- bots are not there!
    }

    //
    // Preparation
    //

    pub fn prepare_bot_grid(&mut self, n_bots_x: i32, n_bots_z: i32) {
        // bot grid = botの数と相対位置
        eprintln!("Bounding box: {:?}", util::get_bounding_box(&self.model.filled));
        let (bb_length_x, bb_length_z) = self.get_bounding_box_lengths();

        let n_bots_x = min(n_bots_x, (bb_length_x - 2) / CELL_LENGTH + 2);
        let n_bots_z = min(n_bots_z, (bb_length_z - 2) / CELL_LENGTH + 2);
        let n_bots_x = max(n_bots_x, (MAX_N_BOTS as i32) / n_bots_z);
        let n_bots_z = max(n_bots_z, (MAX_N_BOTS as i32) / n_bots_x);
        let n_bots_x = min(n_bots_x, (bb_length_x - 2) / CELL_LENGTH + 2);
        let n_bots_z = min(n_bots_z, (bb_length_z - 2) / CELL_LENGTH + 2);

        let bot_grid_relps: Vec<Vec<P>> = (0..n_bots_x).map(|ix| {
            (0..n_bots_z).map(|iz| {
                P::new(
                    min(bb_length_x - 1, ix * CELL_LENGTH),
                    0,
                    min(bb_length_z - 1, iz * CELL_LENGTH))
            }).collect()
        }).collect();

        self.bot_grid_relps = bot_grid_relps;

        eprintln!("Bot grid: {} X {} (actual size: {:?})", self.bot_grid_relps.len(), self.bot_grid_relps[0].len(), self.get_bot_grid_total_lengths());
        eprintln!("({:?})", self.bot_grid_relps);
        eprintln!();
    }

    pub fn prepare_session_schedule(&mut self) {
        // TODO: y should be determined for every session
        let bb = util::get_bounding_box(&self.model.filled);
        let max_filled_y = bb.1.y;

        let (bb_length_x, bb_length_z) = self.get_bounding_box_lengths();
        let (bg_length_x, bg_length_z) = self.get_bot_grid_total_lengths();
        eprintln!("{} {}", bb_length_x, bg_length_x);

        let n_sessions_x = (bb_length_x + bg_length_x - 1) / bg_length_x;
        let n_sessions_z = (bb_length_z + bg_length_z - 1) / bg_length_z;

        let mut session_absps = vec![];
        for ix in 0..n_sessions_x {
            for k in 0..n_sessions_z {
                let iz;
                if ix % 2 == 0 {
                    iz = k;
                } else {
                    iz = n_sessions_z - k - 1;
                }

                session_absps.push(
                    P::new(
                        bb.0.x +min(ix * bg_length_x, bb_length_x - bg_length_x),
                        max_filled_y + 1,
                        bb.0.z +min(iz * bg_length_z, bb_length_z - bg_length_z)))
            }
        }

        self.session_absps = session_absps;

        eprintln!("Sessions: x={}, z={} -> {}", n_sessions_x, n_sessions_z, self.session_absps.len());
        eprintln!("({:?})", self.session_absps);
        eprintln!();
    }

    pub fn fission(&mut self) {
        let (n_bots_x, n_bots_z) = (self.bot_grid_relps.len(), self.bot_grid_relps[0].len());
        let n_bots = n_bots_x * n_bots_z;

        // Positions
        let ps: Vec<P> = (0..n_bots)
            .map(|i| {
                let ix = i / n_bots_z;
                let iz = i % n_bots_z;
                self.session_absps[0] + self.bot_grid_relps[ix][iz]
            }).collect();

        let (ord, cmds) = fission_to(&self.model.filled, &ps);
        self.fission_commands = cmds;
        // let ord: Vec<usize> = (1..(n_bots + 1)).collect();s
        eprintln!("Ordering from fission: {:?}", ord);

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

        let bot_grid_bids = (0..n_bots_x)
            .map(|ix| {
                (0..n_bots_z)
                    .map(|iz| {
                        ord[ix * n_bots_z + iz] - 1 // ord is 1-indexed
                    })
                    .collect()
            })
            .collect();

        eprintln!("Bot grid BIDs: {:?}", bot_grid_bids);

        self.bot_grid_bids = bot_grid_bids;
    }

    pub fn destroy_all(&mut self) {
        for i in 0..self.session_absps.len() {
            // Transition
            let nxt_p0 = self.session_absps[i];
            let crr_p0 = self.bots[self.bot_grid_bids[0][0]].p;

            eprintln!("Session: {:?}", nxt_p0);

            if i == 0 {
                assert_eq!(crr_p0, nxt_p0);
            } else {
                eprintln!("Session transition: {:?} -> {:?}", crr_p0, nxt_p0);
                self.move_to_next_session(nxt_p0 - crr_p0);
            }

            self.destroy_session();
        }
        self.harmonizer.check_complete();
    }

    pub fn main(&mut self) {
        self.prepare_bot_grid(DEFAULT_MAX_N_BOTS_X, DEFAULT_MAX_N_BOTS_Z);
        self.prepare_session_schedule();
        self.fission();
        self.destroy_all();
        self.harmonize();
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