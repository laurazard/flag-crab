use crate::usecases::usecase::UseCase;
use crate::domain::flag::Flag;
use crate::adapters::persistence::flag_repo::FlagRepo;

pub struct GetAllFlags {
    flag_repo: Box<dyn FlagRepo>
}

impl UseCase<Vec<Flag>> for GetAllFlags {

    fn invoke(&self) -> &Vec<Flag> {
        self.flag_repo.get_all_flags()
    }
}

impl GetAllFlags {
    pub fn new(flag_repo: Box<dyn FlagRepo>) -> Self {
        GetAllFlags {
            flag_repo
        }
    }
}