use crate::adapters::persistence::flag_repo::FlagRepo;
use crate::domain::flag::Flag;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct GetFlag {
    pub flag_repo: Arc<Mutex<Box<dyn FlagRepo>>>,
}

impl GetFlag {
    pub fn new(flag_repo: Arc<Mutex<Box<dyn FlagRepo>>>) -> Self {
        GetFlag { flag_repo }
    }

    pub(crate) fn invoke(&self, id: u32) -> (Option<Flag>, HashMap<u32, Flag>) {
        let flag_repo = self.flag_repo.lock().unwrap();
        (flag_repo.get_by_id(id), flag_repo.get_flag_snapshots(id))
    }
}
