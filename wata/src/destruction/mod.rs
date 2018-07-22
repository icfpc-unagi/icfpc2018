#![allow(unused)]

pub mod strategy_small;
pub mod strategy_large;

use super::{Model, Command};

pub fn destroy(model: Model) -> Vec<Command> {
    if model.r <= 30 {
        strategy_small::destroy_small(model)
    } else {
        strategy_large::destroy_large(model)
    }
}
