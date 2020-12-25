use crate::domain::flag::Flag;
use crate::adapters::persistence::flag_repo::FlagRepo;
use std::sync::{Mutex, Arc};

pub struct GetAllFlags {
    flag_repo: Arc<Mutex<dyn FlagRepo>>
}

impl GetAllFlags {

    pub fn new(flag_repo: Arc<Mutex<dyn FlagRepo>>) -> Self {
        GetAllFlags {
            flag_repo
        }
    }

    pub(crate) fn invoke(&self) -> Vec<Flag> {
        let repo = self.flag_repo.lock().unwrap();
        repo.get_all_flags()
    }
}