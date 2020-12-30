use chrono::{DateTime, Local};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Flag {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub last_updated: DateTime<Local>,
    pub enabled: bool,
}

impl Flag {
    pub fn new(id: u32, name: String) -> Flag {
        Flag {
            id,
            name,
            description: String::from(""),
            last_updated: Local::now(),
            enabled: true,
        }
    }
}

impl PartialEq for Flag {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn equality() {
        let flag_a = Flag::new(7, String::from("a description"));
        let flag_b = Flag::new(7, String::from("a different description"));

        assert_eq!(flag_a, flag_b);
    }
}
