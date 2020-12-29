use std::collections::HashMap;
use std::sync::Mutex;

use rocket::*;
use rocket::request::Form;
use rocket::State;
use rocket_contrib::templates::Template;

use crate::usecases::add_flag::AddFlag;
use crate::usecases::get_all_flags::GetAllFlags;
use crate::domain::flag::Flag;


#[derive(FromForm)]
pub(crate) struct FlagInput {
    id: u32,
    name: String,
}

#[post("/", data = "<flag_input>")]
pub(crate) fn create_flag(flag_input: Form<FlagInput>, add_flag: State<Mutex<AddFlag>>, get_all_flags: State<GetAllFlags>) -> Template {
    {
        add_flag.lock().unwrap().invoke(Flag::new(flag_input.id.clone(), flag_input.name.clone()));
    }
    let context: HashMap<&str, Vec<Flag>> = [("flags", get_all_flags.invoke())]
        .iter().cloned().collect();
    Template::render("home", context)
}
