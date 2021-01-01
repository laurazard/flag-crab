use std::collections::HashMap;
use std::sync::Mutex;

use rocket::request::Form;
use rocket::State;
use rocket::*;
use rocket_contrib::templates::Template;

use crate::domain::flag::Flag;
use crate::usecases::get_all_flags::GetAllFlags;
use crate::usecases::update_flag::UpdateFlag;

#[derive(FromForm)]
pub(crate) struct FlagUpdateInput {
    id: u32,
    enabled: bool,
}

#[put("/", data = "<input>")]
pub(crate) fn update_flag(
    input: Form<FlagUpdateInput>,
    update_flag_usecase: State<Mutex<UpdateFlag>>,
    get_all_flags: State<GetAllFlags>,
) -> Template {
    {
        update_flag_usecase
            .lock()
            .unwrap()
            .invoke(input.id, input.enabled);
    }
    let context: HashMap<&str, Vec<Flag>> = [("flags", get_all_flags.invoke())]
        .iter()
        .cloned()
        .collect();
    Template::render("home", context)
}
