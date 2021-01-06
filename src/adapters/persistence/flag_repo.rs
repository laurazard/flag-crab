use crate::domain::flag::Flag;
use std::collections::HashMap;

pub trait FlagRepo: Send + Sync {
    fn new() -> Self
    where
        Self: Sized;
    fn add_flag(&mut self, flag: Flag);
    fn remove_flag(&mut self, id: u32);
    fn update_flag(&mut self, flag: Flag);
    fn get_by_id(&self, id: u32) -> Option<Flag>;
    fn get_all_flags(&self) -> Vec<Flag>;
    fn get_flag_snapshots(&self) -> HashMap<u32, Flag>;
    fn length(&self) -> usize;
}
