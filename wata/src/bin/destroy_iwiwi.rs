#![allow(unused)]
extern crate wata;

/*
Backlog:
- まずはなんでも良いから全部破壊する
    - 方向固定: 上から
    - Harmonics: とりあえず常時on
    - bot: 6 * 6 固定
- bounding boxを真面目にやる
- Harmonicsを必要なときだけにする
- 方向: 全通り試す
- bot: 5 * 8 とか色々試すようにする
- 1回の面の消しをどういう順番でやるか全部試す
- Harmonicsのオンオフを、余りbotが居ればそいつがやるようにする

そのうち
- 余るような小さいやつだったら1箇所にbotを2つおく
*/

use wata::*;
use wata::bfs::*;
use std::cmp::min;
use std::collections::*;

#[derive(Clone, Debug)]
struct Solution {
    score: i64,
    trace: Vec<Command>,
}

#[derive(Copy, Clone, Debug)]
struct Bot {
    bid: usize,
    p: P,
}

/*
struct Trace {
    model: Model,
    commands: Vec<Vec<Command>>,
    harmonics: bool,
    bots: Vec<Bot>,
}

impl Trace {
    fn new(model: Model) -> Trace {
        Trace {
            model,
            commands: vec![],
            harmonics: false,
            bots: vec![Bot { bid: 0, p: P::new(0, 0, 0)}; 1],
        }
    }

    fn fission(&mut self, n: usize, ) {
        // Should be called first
        assert_eq!(self.commands.len(), 0);

        for t in 0..(n - 1) {
            let mut step = vec![];
            for _ in 0..t {
                step.push(Command::Wait);
            }
            step.push(Command::Fission(P::new(1, 0, 0), 18 - t as usize));
            self.commands.push(step);
        }
        self.bots = (0..n).map(|i| Bot { bid: i, p: P::new(i as i32, 0, 0) }).collect()
    }

    fn flip_on(&mut self) {

    }

    fn flip_off(&mut self) {

    }

    fn get_flattened_trace() {
        let mut a = vec![];
        for step in self.
    }
}

fn destroy(model: Model, n1: usize, n2: usize) -> Solution {
    let mut trace: Trace = Trace::new(model);
    trace.fission(n1 * n2);

    return Solution {

    }
}
*/

//
// Commands invoked in a single time step by the bots
//

#[derive(Clone, Debug)]
struct CommandSet {
    commands: Vec<Command>,
}

impl CommandSet {
    fn new(n_bots: usize) -> CommandSet {
        CommandSet {
            commands: vec![Command::Wait; n_bots]
        }
    }

    fn new_uniform(n_bots: usize, command: Command) -> CommandSet {
        CommandSet {
            commands: vec![command; n_bots]
        }
    }

    fn is_all_wait(&self) -> bool {
        return self.commands.iter().all(|&cmd| cmd == Command::Wait);
    }

    fn gvoid_below_layer(&mut self, bots: [&Bot; 4]) {
        // TODO: 常に真下ではなく斜めを使ってわずかに稼ぐか？（優先度低い）
        let nd = P::new(0, -1, 0);

        for i in 0..4 {
            let b1 = bots[i];
            let b2 = bots[i ^ 3];
            assert_eq!(self.commands[b1.bid], Command::Wait);
            self.commands[b1.bid] = Command::GVoid(
                nd,
                b2.p - b1.p
            )
        }
    }

    fn emit(&self) {
        for command in self.commands.iter() {
            println!("{}", command.to_string());
        }
    }
}


struct App {
    model: Model,
    bots: Vec<Bot>,
    command_sets: Vec<CommandSet>,
}

const CELL_LENGTH: i32 = 30;

impl App {
    fn new(model: &Model) -> App {
        App {
            model: model.clone(),
            bots: vec![Bot { bid: 0, p: P::new(0, 0, 0)}; 1],
            command_sets: vec![],
        }
    }

    fn destroy_layer(&mut self, bot_grid: &Vec<Vec<usize>>) {
        let (n_bots_x, n_bots_z) = (bot_grid.len(), bot_grid[0].len());

        for parity_x in 0..2 {
            for parity_z in 0..2 {
                let mut cs = CommandSet::new(self.bots.len());

                let mut ix = parity_x;
                let mut iz = parity_z;

                while ix + 1 < n_bots_x {
                    while iz + 1 < n_bots_z {
                        cs.gvoid_below_layer([
                            &self.bots[bot_grid[ix + 0][iz + 0]],
                            &self.bots[bot_grid[ix + 0][iz + 1]],
                            &self.bots[bot_grid[ix + 1][iz + 0]],
                            &self.bots[bot_grid[ix + 1][iz + 1]],
                        ]);
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
        self.command_sets.push(
            CommandSet::new_uniform(
                self.bots.len(), Command::SMove(dif)));
        for bot in self.bots.iter_mut() {
            bot.p += dif;
        }
    }

    fn destroy_session(&mut self, bot_grid: &Vec<Vec<usize>>) {
        // Session: x and z coordinates are fixed. Destroy along y axis.

        let n_bots_x = bot_grid.len();
        let n_bots_z = bot_grid[0].len();

        // TODO: Confirm the initial bot positions
        /*
        for ix in 0..n_bots_x {
            for iz in 0..n_bots_z {
                let q = p + P::new((ix as i32) * CELL_LENGTH, 0, (iz as i32) * CELL_LENGTH);
                let b = &self.bots[bot_grid[ix][iz]];
                assert_eq!(q, b.p);
            }
        }
        */

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

    fn fission(&mut self, n_bots_x: usize, n_bots_z: usize) -> Vec<Vec<usize>> {
        let r = self.model.r as i32;
        let n_bots = n_bots_x * n_bots_z;

        // Positions
        let p0 = P::new(0, (r as i32) - 1, 0);  // TODO: better starting point
        let ps: Vec<P> = (0..n_bots).map(|i| {
            let ix = (i / n_bots_z) as i32;
            let iz = (i % n_bots_z) as i32;
            p0 + P::new(min(CELL_LENGTH * ix, r - 1), 0, min(CELL_LENGTH * iz, r - 1))
        }).collect();

        let (ord, cmds) = fission_to(&self.model.filled, &ps);
        // let ord: Vec<usize> = (0..n_bots).collect();
        // TODO: print command

        self.bots = (0..n_bots).map(|bid| {
            Bot {
                bid,
                p: P::new(-1, -1, -1)  // Dummy
            }
        }).collect();
        for (&i, &p) in ord.iter().zip(ps.iter()) {
            self.bots[i].p = p;
        }

        let bot_grid = (0..n_bots_x).map(|ix| {
            (0..n_bots_z).map(|iz| {
                ord[ix * n_bots_z + iz]
            }).collect()
        }).collect();
        return bot_grid;
    }

    fn emit(&self) {
        for command_set in self.command_sets.iter() {
            command_set.emit();
        }
    }

    fn main(&mut self) {
        let r = self.model.r;
        // TODO: hoge
        let n_bots_x = min(6, (r - 1) / (CELL_LENGTH as usize) + 2);
        let n_bots_z = n_bots_x;
        eprintln!("R: {}, Bot grid: {} X {}", r, n_bots_x, n_bots_z);

        let bot_grid = self.fission(n_bots_x, n_bots_z);
        eprintln!("{:?}", bot_grid);
        eprintln!("{:?}", self.bots);

        self.destroy_session(&bot_grid);

        for (i, command_set) in self.command_sets.iter().enumerate() {
            println!("[ STEP {} ]", i);
            command_set.emit();
        }

        // eprintln!("{:?}", self.command_sets);
        // self.emit();
    }
}

/*
fn destroy_session(
    model: Model, Vec<Vec<usize>>,
) {

}
*/

fn main() {
    let file = std::env::args().nth(1).unwrap();
    let model = wata::read(&file);

    let mut app = App::new(&model);
    app.main();
}
