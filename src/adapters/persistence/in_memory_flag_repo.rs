use crate::adapters::persistence::flag_repo::FlagRepo;
use crate::domain::flag::Flag;


pub(crate) struct InMemoryFlagRepo {
    flags: Vec<Flag>
}

impl FlagRepo for InMemoryFlagRepo {
    fn new() -> InMemoryFlagRepo {
        InMemoryFlagRepo {
            flags: Vec::new()
        }
    }

    fn add_flag(&mut self, mut flag: Flag) {
        if flag.id == 0 {
            flag.id = self.length() as u32 + 1;
        }

        self.flags.push(flag);
    }

    fn remove_flag(&mut self, id: u32) {
        self.flags.retain(|flag| flag.id != id)
    }

    fn get_all_flags(&self) -> Vec<Flag> {
        self.flags.clone()
    }

    fn length(&self) -> usize {
        self.flags.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_length() {
        let mut flag_repo = InMemoryFlagRepo::new();
        assert_eq!(0, flag_repo.length());
        flag_repo.flags.push(Flag::new(23, String::from("test flag")));
        assert_eq!(1, flag_repo.length());
        flag_repo.flags.push(Flag::new(24, String::from("another test flag")));
        assert_eq!(2, flag_repo.length());
        flag_repo.flags.pop();
        flag_repo.flags.pop();
        assert_eq!(0, flag_repo.length())
    }

    #[test]
    fn adds_new_flag() {
        let flag = Flag::new(0, String::from("Test flag"));
        let mut flag_repo = InMemoryFlagRepo::new();
        let expected_flag = Flag::new(1, String::from("Test flag"));

        flag_repo.add_flag(flag);

        assert_eq!(flag_repo.flags[0], expected_flag);
        assert_eq!(flag_repo.flags.len(), 1);
    }

    #[test]
    fn adds_existing_flag() {
        let flag = Flag::new(17, String::from("Test flag"));
        let mut flag_repo = InMemoryFlagRepo::new();
        let same_flag = flag.clone();

        flag_repo.add_flag(flag);

        assert_eq!(same_flag, flag_repo.flags[0]);
        assert_eq!(1, flag_repo.flags.len());
    }

    #[test]
    fn removes() {
        let flag = Flag::new(24, String::from("Test flag"));
        let mut flag_repo = InMemoryFlagRepo::new();
        flag_repo.flags.push(flag);

        assert_eq!(1, flag_repo.length());

        flag_repo.remove_flag(24);

        assert_eq!(0, flag_repo.length())
    }

    #[test]
    fn gets_all() {
        let mut flag_repo = InMemoryFlagRepo::new();
        let expected_flags = vec![
            Flag::new(13, String::from("tests")),
            Flag::new(17, String::from("other tests")),
            Flag::new(14, String::from("another test"))];

        for flag in expected_flags.iter() {
            flag_repo.add_flag(flag.clone())
        }

        assert_eq!(expected_flags, *flag_repo.get_all_flags())
    }
}