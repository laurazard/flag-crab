use std::collections::HashMap;

use rocket::State;
use rocket::*;
use rocket_contrib::templates::Template;

use crate::domain::flag::Flag;
use crate::usecases::get_all_flags::GetAllFlags;

#[get("/")]
pub(crate) fn index(get_all_flags: State<GetAllFlags>) -> Template {
    let context: HashMap<&str, Vec<Flag>> = [("flags", get_all_flags.invoke())]
        .iter()
        .cloned()
        .collect();
    Template::render("home", &context)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::persistence::flag_repo::FlagRepo;
    use crate::adapters::persistence::in_memory_flag_repo::InMemoryFlagRepo;
    use crate::domain::flag::Flag;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test() {
        let mut flag_repo: Box<dyn FlagRepo> = Box::new(InMemoryFlagRepo::new());
        flag_repo.add_flag(Flag::new(1, String::from("A test flag name")));
        let flag_repo_mutex = Arc::new(Mutex::new(flag_repo));
        let get_all_flags_usecase = GetAllFlags::new(Arc::clone(&flag_repo_mutex));
        // FIXME: find a way to do this without waiting a random amount of time for it to be ready
        thread::spawn(|| {
            rocket::ignite()
                .attach(Template::fairing())
                .manage(get_all_flags_usecase)
                .mount("/", routes![super::index])
                .launch();
        });
        thread::sleep(Duration::from_millis(100));

        let params = [("enabled", "false")];
        let result = reqwest::blocking::get("http://localhost:8000/")
            .unwrap()
            .text()
            .unwrap();

        assert!(result.contains("Flag Crab"));
    }
}
