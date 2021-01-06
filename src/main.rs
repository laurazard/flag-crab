#![feature(proc_macro_hygiene, decl_macro)]
#![feature(entry_insert)]

mod adapters;
mod domain;
mod usecases;

use crate::adapters::flag_crab::FlagCrab;
use crate::adapters::persistence::flag_repo::FlagRepo;
use crate::adapters::persistence::in_memory_flag_repo::InMemoryFlagRepo;

fn main() {
    let flag_repo = InMemoryFlagRepo::new();

    FlagCrab::start(Box::new(flag_repo));
}
