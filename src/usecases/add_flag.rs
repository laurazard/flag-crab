use crate::adapters::persistence::flag_repo::FlagRepo;
use crate::domain::flag::Flag;
use std::sync::{Arc, Mutex};

pub struct AddFlag {
    flag_repo: Arc<Mutex<Box<dyn FlagRepo>>>,
}

impl AddFlag {
    pub(crate) fn new(flag_repo: Arc<Mutex<Box<dyn FlagRepo>>>) -> Self {
        AddFlag { flag_repo }
    }

    pub(crate) fn invoke(&mut self, flag: Flag) {
        self.flag_repo.lock().unwrap().add_flag(flag);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::persistence::in_memory_flag_repo::InMemoryFlagRepo;

    #[test]
    fn test_invoke() {
        let repo: Arc<Mutex<Box<dyn FlagRepo>>> =
            Arc::new(Mutex::new(Box::new(InMemoryFlagRepo::new())));
        let mut usecase = AddFlag::new(Arc::clone(&repo));
        let flag = Flag::new(7, String::from("test flag"));
        let expected_flag = flag.clone();

        usecase.invoke(flag);

        assert_eq!(expected_flag, repo.lock().unwrap().get_all_flags()[0]);
        assert_eq!(1, repo.lock().unwrap().length());
    }
}
