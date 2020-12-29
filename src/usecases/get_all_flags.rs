use crate::domain::flag::Flag;
use crate::adapters::persistence::flag_repo::FlagRepo;
use std::sync::{Mutex, Arc};

pub struct GetAllFlags {
    flag_repo: Arc<Mutex<Box<dyn FlagRepo>>>
}

impl GetAllFlags {
    pub fn new(flag_repo: Arc<Mutex<Box<dyn FlagRepo>>>) -> Self {
        GetAllFlags {
            flag_repo
        }
    }

    pub(crate) fn invoke(&self) -> Vec<Flag> {
        let repo = self.flag_repo.lock().unwrap();
        repo.get_all_flags()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::persistence::in_memory_flag_repo::InMemoryFlagRepo;

    #[test]
    fn test_invoke() {
        let flag = Flag::new(7, String::from("test flag"));
        let expected_flag = flag.clone();
        let mut repo = InMemoryFlagRepo::new();
        repo.add_flag(flag);
        let repo: Arc<Mutex<Box<dyn FlagRepo>>> = Arc::new(Mutex::new(Box::new(repo)));
        let usecase = GetAllFlags::new(Arc::clone(&repo));

        let actual_flags = usecase.invoke();

        assert_eq!(expected_flag, actual_flags[0]);
        assert_eq!(1, actual_flags.len());
    }

}