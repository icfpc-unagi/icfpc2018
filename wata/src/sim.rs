#![allow(unused)]
use *;
use Command::*;
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Bot {
    bid: usize,  // should be sorted by (unique) bid
    p: P,
    seeds: BTreeSet<usize>,
}

impl Bot {
    fn new() -> Bot {
        Bot {
            bid: 1,
            p: P::new(0, 0, 0),
            seeds: (2..=20).collect(),
        }
    }
}

#[derive(Clone, Debug)]
struct SimState {
    // energy: i64,
    // harmonics: bool,
    matrix: V3<bool>,
    bots: BTreeSet<Bot>,
}

impl SimState {
    fn new(r: usize) -> SimState {
        let bot = Bot::new();
        let mut bots = BTreeSet::new();
        bots.insert(bot);
        SimState {
            matrix: mat![false; r; r; r],
            bots,
        }
    }

    fn step(&mut self, cmds: Vec<Command>) {
        let bots = std::mem::replace(&mut self.bots, BTreeSet::new());
        assert!(bots.len() == cmds.len());
        for (mut bot, cmd) in bots.into_iter().zip(cmds) {
            match cmd {
                SMove(d) => {bot.p += d}
                LMove(d1, d2) => {bot.p += d1 + d2}
                _ => {}
            }
            self.bots.insert(bot);
        }
    }
}
