use crate::adapters::persistence::flag_repo::FlagRepo;
use crate::domain::flag::Flag;
use std::sync::{Mutex, Arc};

pub struct AddFlag {
    flag_repo: Arc<Mutex<dyn FlagRepo>>
}

impl AddFlag {
    pub(crate) fn new(flag_repo: Arc<Mutex<dyn FlagRepo>>) -> Self {
        AddFlag {
            flag_repo
        }
    }

    pub(crate) fn invoke(&mut self, flag: Flag) {
        let mut repo = self.flag_repo.lock().unwrap();
        repo.add_flag(flag)
    }
}