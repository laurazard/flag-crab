use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Flag {
    id: u32,
    description: String
}

#[allow(dead_code)]
impl Flag {
    pub fn new(id: u32, description: String) -> Flag {
        Flag {
            id,
            description
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

impl PartialEq for Flag {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn equality() {
        let flag_a = Flag::new(7, String::from("a description"));
        let flag_b = Flag::new(7, String::from("a different description"));

        assert_eq!(flag_a, flag_b);
    }
}