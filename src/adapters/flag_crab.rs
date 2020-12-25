use rocket::*;
use std::collections::HashMap;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

use crate::adapters::persistence::flag_repo::FlagRepo;
use crate::adapters::persistence::in_memory_flag_repo::InMemoryFlagRepo;
use crate::domain::flag::Flag;
use crate::usecases::get_all_flags::GetAllFlags;
use crate::usecases::add_flag::AddFlag;
use std::sync::{Mutex, Arc};

#[allow(dead_code)]
pub struct FlagCrab {
    flag_repo: Box<dyn FlagRepo>
}

#[get("/")]
fn index(get_all_flags: State<GetAllFlags>) -> Template {
    let context: HashMap<&str, Vec<Flag>> = [("flags", get_all_flags.invoke())]
        .iter().cloned().collect();
    Template::render("new_home", &context)
}

#[get("/playground")]
fn fun_times(get_all_flags: State<GetAllFlags>) -> Template {
    let context: HashMap<&str, Vec<Flag>> = [("flags", get_all_flags.invoke())]
        .iter().cloned().collect();
    Template::render("home", &context)
}

impl FlagCrab {
    #[allow(dead_code)]
    pub fn start(_: Box<dyn FlagRepo>) {
        let mutex: Arc<Mutex<dyn FlagRepo>> = Arc::new(Mutex::new(InMemoryFlagRepo::new()));

        // instantiate some crappy data to make sure something works
        {
            let mut mutable_repo_reference = mutex.lock().unwrap();
            mutable_repo_reference.add_flag(Flag::new(17, String::from("test")));
        }

        let get_all_flags_usecase = GetAllFlags::new(Arc::clone(&mutex));
        let _add_flag = AddFlag::new(Arc::clone(&mutex));

        rocket::ignite()
            .attach(Template::fairing())
            .manage(get_all_flags_usecase)
            .mount("/static", StaticFiles::from("templates/static"))
            .mount("/", routes![index, fun_times])
            .launch();
    }
}