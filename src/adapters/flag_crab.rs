use rocket::*;
use std::collections::HashMap;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

use crate::adapters::persistence::flag_repo::FlagRepo;
use crate::domain::flag::Flag;
use crate::usecases::usecase::UseCase;
use crate::usecases::get_all_flags::GetAllFlags;

#[allow(dead_code)]
pub struct FlagCrab {
    flag_repo: Box<dyn FlagRepo>
}

#[get("/")]
fn index(get_all_flags: State<GetAllFlags>) -> Template {
    let context: HashMap<&str, &Vec<Flag>> = [("flags", get_all_flags.invoke())]
        .iter().cloned().collect();
    Template::render("new_home", &context)
}

#[get("/playground")]
fn fun_times(get_all_flags: State<GetAllFlags>) -> Template {
    let context: HashMap<&str, &Vec<Flag>> = [("flags", get_all_flags.invoke())]
        .iter().cloned().collect();
    Template::render("home", &context)
}

impl FlagCrab{

    pub fn start(flag_repo: Box<dyn FlagRepo>){

        let get_all_flags_usecase = GetAllFlags::new(flag_repo);

        rocket::ignite()
            .attach(Template::fairing())
            .manage(get_all_flags_usecase)
            .mount("/static", StaticFiles::from("templates/static"))
            .mount("/", routes![index, fun_times])
            .launch();
    }

}