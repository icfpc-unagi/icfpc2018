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

    fn fission(&mut self, nd: P, m: usize) -> Bot {
        let mut seeds_old = std::mem::replace(&mut self.seeds, BTreeSet::new()).into_iter();
        let bid = seeds_old.next().unwrap();
        // let seeds = seeds_old.take(m).collect();
        let mut seeds = BTreeSet::new();
        for _ in 0..m {
            seeds.insert(seeds_old.next().unwrap());
        }
        self.seeds = seeds_old.collect();
        Bot {
            bid,
            p: self.p + nd,
            seeds,
        }
    }

    fn fusion(&mut self, mut other: Bot) {
        self.seeds.insert(other.bid);
        self.seeds.append(&mut other.seeds);
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
                Fission(nd, m) => {
                    self.bots.insert(bot.fission(nd, m));
                }
                _ => {}
            }
            self.bots.insert(bot);
        }
    }
}