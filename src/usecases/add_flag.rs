use crate::adapters::persistence::flag_repo::FlagRepo;
use crate::usecases::usecase::UseCase;

pub struct AddFlag {
    flag_repo: dyn FlagRepo
}

impl UseCase<bool> for AddFlag {
    fn invoke(&self) -> &bool {
        self.flag_repo.add_flag()
    }
}