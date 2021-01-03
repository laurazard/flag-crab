use std::collections::HashMap;

use rocket::State;
use rocket::*;
use rocket_contrib::templates::Template;

use crate::usecases::get_flag::GetFlag;

#[get("/flag/<id>")]
pub(crate) fn get_flag(id: u32, get_flag: State<GetFlag>) -> Template {
    let flag = get_flag.invoke(id).unwrap();
    let context: HashMap<&str, String> = [
        ("name", flag.name.clone()),
        ("id", flag.id.to_string()),
        ("description", flag.description.clone()),
        ("enabled", flag.enabled.to_string()),
        ("json", serde_json::to_string_pretty(&flag).unwrap()),
    ]
    .iter()
    .cloned()
    .collect();
    println!("{:?}", context);
    Template::render("flag", context)
}
