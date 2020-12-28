#![feature(decl_macro)]

mod adapters;
mod usecases;
mod domain;

use crate::adapters::persistence::in_memory_flag_repo::InMemoryFlagRepo;
use crate::adapters::flag_crab::FlagCrab;
use crate::adapters::persistence::flag_repo::FlagRepo;

fn main() {
    let flag_repo = InMemoryFlagRepo::new();

    FlagCrab::start(Box::new(flag_repo));
}