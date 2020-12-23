#![feature(decl_macro)]

mod adapters;
mod usecases;
mod domain;

use crate::adapters::persistence::in_memory_flag_repo::InMemoryFlagRepo;
use crate::adapters::flag_crab::FlagCrab;
use crate::adapters::persistence::flag_repo::FlagRepo;
use crate::domain::flag::Flag;

fn main() {
    let mut flag_repo = InMemoryFlagRepo::new();
    flag_repo.add_flag(Flag::new(17, String::from("test")));

    FlagCrab::start(Box::new(flag_repo));
}