use std::collections::HashMap;

use rocket::*;
use rocket::State;
use rocket_contrib::templates::Template;

use crate::usecases::get_all_flags::GetAllFlags;
use crate::domain::flag::Flag;

#[get("/")]
pub(crate) fn index(get_all_flags: State<GetAllFlags>) -> Template {
    let context: HashMap<&str, Vec<Flag>> = [("flags", get_all_flags.invoke())]
        .iter().cloned().collect();
    Template::render("new_home", &context)
}
