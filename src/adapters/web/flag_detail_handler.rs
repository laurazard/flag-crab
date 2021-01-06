use std::collections::HashMap;

use rocket::State;
use rocket::*;
use rocket_contrib::templates::Template;
use serde::Serialize;

use crate::usecases::get_flag::GetFlag;

#[derive(Serialize)]
struct ThisContext {
    name: String,
    id: String,
    description: String,
    enabled: String,
    snapshots: HashMap<u32, String>,
}

#[get("/flag/<id>")]
pub(crate) fn get_flag(id: u32, get_flag: State<GetFlag>) -> Template {
    let (flag, snapshots) = get_flag.invoke(id);
    let flag = flag.unwrap();

    let mut serialized_snapshots = HashMap::new();

    for (i, snapshot) in snapshots {
        serialized_snapshots
            .entry(i)
            .or_insert(serde_json::to_string_pretty(&snapshot).unwrap());
    }

    let context = ThisContext {
        name: flag.name.clone(),
        id: flag.id.to_string(),
        description: flag.description.clone(),
        enabled: flag.enabled.to_string(),
        snapshots: serialized_snapshots,
    };

    Template::render("flag", context)
}
