use rocket::*;
use rocket::request::Form;
use std::collections::HashMap;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

use crate::adapters::persistence::flag_repo::FlagRepo;
use crate::domain::flag::Flag;
use crate::usecases::get_all_flags::GetAllFlags;
use crate::usecases::add_flag::AddFlag;
use std::sync::{Mutex, Arc};

pub struct FlagCrab {}

#[get("/")]
fn index(get_all_flags: State<GetAllFlags>) -> Template {
    let context: HashMap<&str, Vec<Flag>> = [("flags", get_all_flags.invoke())]
        .iter().cloned().collect();
    Template::render("new_home", &context)
}

#[derive(FromForm)]
struct FlagInput {
    id: u32,
    description: String,
}

#[post("/", data = "<flag_input>")]
fn create_flag(flag_input: Form<FlagInput>, add_flag: State<Mutex<AddFlag>>, get_all_flags: State<GetAllFlags>) -> Template {
    {
        add_flag.lock().unwrap().invoke(Flag::new(flag_input.id.clone(), flag_input.description.clone()));
    }
    let context: HashMap<&str, Vec<Flag>> = [("flags", get_all_flags.invoke())]
        .iter().cloned().collect();
    Template::render("new_home", context)
}

#[get("/playground")]
fn fun_times(get_all_flags: State<GetAllFlags>) -> Template {
    let context: HashMap<&str, Vec<Flag>> = [("flags", get_all_flags.invoke())]
        .iter().cloned().collect();
    Template::render("home", &context)
}

impl FlagCrab {
    pub fn start(flag_repo: Box<dyn FlagRepo>) {
        let flag_repo_mutex: Arc<Mutex<Box<dyn FlagRepo>>> = Arc::new(Mutex::new(flag_repo));

        let get_all_flags_usecase = GetAllFlags::new(Arc::clone(&flag_repo_mutex));
        let add_flag_usecase = AddFlag::new(Arc::clone(&flag_repo_mutex));

        rocket::ignite()
            .attach(Template::fairing())
            .manage(get_all_flags_usecase)
            .manage(Mutex::new(add_flag_usecase))
            .mount("/static", StaticFiles::from("templates/static"))
            .mount("/", routes![index, fun_times, create_flag])
            .launch();
    }
}