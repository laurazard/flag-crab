use crate::adapters::persistence::flag_repo::FlagRepo;
use crate::domain::flag::Flag;

pub struct AddFlag {
    flag_repo: dyn FlagRepo
}

impl AddFlag {
    fn invoke(&mut self, flag: Flag) {
        self.flag_repo.add_flag(flag)
    }
}