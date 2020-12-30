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
