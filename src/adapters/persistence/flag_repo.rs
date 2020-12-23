use crate::domain::flag::Flag;

pub trait FlagRepo : Send + Sync {
    fn new() -> Self where Self: Sized;
    fn add_flag(&mut self, flag: Flag);
    fn remove_flag(&mut self, id: u32);
    fn get_all_flags(&self) -> &Vec<Flag>;
    fn length(&self) -> usize;
}