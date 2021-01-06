use crate::adapters::persistence::flag_repo::FlagRepo;
use chrono::Local;
use std::sync::{Arc, Mutex};

pub struct UpdateFlag {
    flag_repo: Arc<Mutex<Box<dyn FlagRepo>>>,
}

impl UpdateFlag {
    pub fn new(flag_repo: Arc<Mutex<Box<dyn FlagRepo>>>) -> Self {
        UpdateFlag { flag_repo }
    }

    pub fn invoke(&mut self, flag_id: u32, enabled: bool) {
        let mut flag_repo = self.flag_repo.lock().unwrap();
        let flag_option = flag_repo.get_by_id(flag_id);

        match flag_option {
            Some(mut flag) => {
                flag.enabled = enabled;
                // TODO: consider moving the update last_updated logic to repo?
                flag.last_updated = Local::now();
                flag_repo.update_flag(flag);
            }
            None => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::persistence::in_memory_flag_repo::InMemoryFlagRepo;
    use crate::domain::flag::Flag;

    #[test]
    fn invokes() {
        let mut flag_repo = InMemoryFlagRepo::new();
        let enabled_flag = Flag::new(4, String::from("old flag"));
        flag_repo.add_flag(enabled_flag.clone());
        let mutex: Arc<Mutex<Box<dyn FlagRepo>>> = Arc::new(Mutex::new(Box::new(flag_repo)));
        let mut update_flag = UpdateFlag::new(Arc::clone(&mutex));

        update_flag.invoke(4, false);

        assert_eq!(false, mutex.lock().unwrap().get_all_flags()[0].enabled);
        assert!(mutex.lock().unwrap().get_all_flags()[0].last_updated > enabled_flag.last_updated);
    }
}
