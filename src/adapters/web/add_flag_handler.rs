use std::collections::HashMap;
use std::sync::Mutex;

use rocket::request::Form;
use rocket::State;
use rocket::*;
use rocket_contrib::templates::Template;

use crate::domain::flag::Flag;
use crate::usecases::add_flag::AddFlag;
use crate::usecases::get_all_flags::GetAllFlags;

#[derive(FromForm)]
pub(crate) struct FlagInput {
    name: String,
    description: String,
    enabled: bool,
}

#[post("/", data = "<flag_input>")]
pub(crate) fn create_flag(
    flag_input: Form<FlagInput>,
    add_flag: State<Mutex<AddFlag>>,
    get_all_flags: State<GetAllFlags>,
) -> Template {
    let mut new_flag = Flag::new(0, flag_input.name.clone());
    new_flag.enabled = flag_input.enabled.clone();
    new_flag.description = flag_input.description.clone();
    {
        add_flag.lock().unwrap().invoke(new_flag);
    }
    let context: HashMap<&str, Vec<Flag>> = [("flags", get_all_flags.invoke())]
        .iter()
        .cloned()
        .collect();
    Template::render("home", context)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::persistence::flag_repo::FlagRepo;
    use crate::adapters::persistence::in_memory_flag_repo::InMemoryFlagRepo;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test() {
        let flag_repo: Box<dyn FlagRepo> = Box::new(InMemoryFlagRepo::new());
        let flag_repo_mutex = Arc::new(Mutex::new(flag_repo));
        let get_all_flags_usecase = GetAllFlags::new(Arc::clone(&flag_repo_mutex));
        let add_flag_usecase = AddFlag::new(Arc::clone(&flag_repo_mutex));

        // FIXME: find a way to do this without waiting a random amount of time for it to be ready
        thread::spawn(|| {
            rocket::ignite()
                .attach(Template::fairing())
                .manage(get_all_flags_usecase)
                .manage(Mutex::new(add_flag_usecase))
                .mount("/", routes![super::create_flag])
                .launch();
        });
        thread::sleep(Duration::from_millis(100));

        let params = [
            ("name", "a test flag"),
            ("description", "a test flag for unit tests"),
            ("enabled", "true"),
        ];
        reqwest::blocking::Client::new()
            .post("http://localhost:8000")
            .form(&params)
            .send();

        assert_eq!(
            flag_repo_mutex.lock().unwrap().get_all_flags()[0].name,
            String::from("a test flag")
        );
    }
}
