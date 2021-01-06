use crate::adapters::persistence::flag_repo::FlagRepo;
use crate::domain::flag::Flag;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub(crate) struct InMemoryFlagRepo {
    flags: Vec<Flag>,
    snapshots: HashMap<u32, HashMap<u32, Flag>>,
}

impl FlagRepo for InMemoryFlagRepo {
    fn new() -> InMemoryFlagRepo {
        InMemoryFlagRepo {
            flags: Vec::new(),
            snapshots: HashMap::new(),
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

    fn update_flag(&mut self, flag: Flag) {
        for i in 0..self.flags.len() {
            if self.flags[i].id == flag.id {
                self.archive_flag(flag.clone());
                self.flags[i] = flag;
                break;
            }
        }
    }

    fn get_by_id(&self, id: u32) -> Option<Flag> {
        for i in 0..self.flags.len() {
            if self.flags[i].id == id {
                return Some(self.flags[i].clone());
            }
        }
        return None;
    }

    fn get_all_flags(&self) -> Vec<Flag> {
        self.flags.clone()
    }

    // FIXME: I was lazy, but this should just return an Option
    fn get_flag_snapshots(&self, id: u32) -> HashMap<u32, Flag> {
        match self.snapshots.get(&id) {
            None => HashMap::new(),
            Some(s) => s.clone(),
        }
    }

    fn length(&self) -> usize {
        self.flags.len()
    }
}

impl InMemoryFlagRepo {
    fn archive_flag(&mut self, flag: Flag) {
        match self.snapshots.entry(flag.id) {
            Entry::Occupied(o) => {
                let len = o.get().keys().len() as u32;
                o.into_mut().insert(len, flag);
            }
            Entry::Vacant(_) => {
                self.snapshots
                    .entry(flag.id)
                    .or_insert(HashMap::new())
                    .insert(0, flag);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_length() {
        let mut flag_repo = InMemoryFlagRepo::new();
        assert_eq!(0, flag_repo.length());
        flag_repo
            .flags
            .push(Flag::new(23, String::from("test flag")));
        assert_eq!(1, flag_repo.length());
        flag_repo
            .flags
            .push(Flag::new(24, String::from("another test flag")));
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

        assert_eq!(0, flag_repo.length());
    }

    #[test]
    fn updates() {
        let mut flag_repo = InMemoryFlagRepo::new();
        let old_flag = Flag::new(7, String::from("old flag"));
        flag_repo.add_flag(old_flag.clone());

        let new_flag = Flag::new(7, String::from("new flag"));
        flag_repo.update_flag(new_flag);

        assert_eq!(String::from("new flag"), flag_repo.get_all_flags()[0].name);
        assert!(flag_repo.get_all_flags()[0].last_updated > old_flag.last_updated);
    }

    #[test]
    fn gets_by_id() {
        let mut flag_repo = InMemoryFlagRepo::new();
        let expected_flags = vec![
            Flag::new(13, String::from("tests")),
            Flag::new(17, String::from("other tests")),
            Flag::new(14, String::from("another test")),
        ];

        for flag in expected_flags.iter() {
            flag_repo.add_flag(flag.clone())
        }

        assert_eq!(
            String::from("other tests"),
            flag_repo.get_by_id(17).unwrap().name
        );
    }

    #[test]
    fn gets_all() {
        let mut flag_repo = InMemoryFlagRepo::new();
        let expected_flags = vec![
            Flag::new(13, String::from("tests")),
            Flag::new(17, String::from("other tests")),
            Flag::new(14, String::from("another test")),
        ];

        for flag in expected_flags.iter() {
            flag_repo.add_flag(flag.clone())
        }

        assert_eq!(expected_flags, *flag_repo.get_all_flags())
    }

    #[test]
    fn gets_snapshots() {
        let mut flag_repo = InMemoryFlagRepo::new();
        let flag = Flag::new(13, String::from("test"));
        flag_repo.add_flag(flag.clone());
        let mut updated_flag = flag.clone();
        updated_flag.enabled = false;

        flag_repo.update_flag(updated_flag);

        assert_eq!(flag, flag_repo.get_flag_snapshots(13)[&0]);
    }
}
