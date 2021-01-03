use std::sync::Mutex;

use rocket::request::Form;
use rocket::State;
use rocket::*;

use crate::usecases::update_flag::UpdateFlag;
use rocket::response::Redirect;

#[derive(FromForm)]
pub(crate) struct FlagUpdateInput {
    enabled: bool,
}

#[post("/flag/<id>", data = "<input>")]
pub(crate) fn update_flag(
    id: u32,
    input: Form<FlagUpdateInput>,
    update_flag_usecase: State<Mutex<UpdateFlag>>,
) -> Redirect {
    {
        update_flag_usecase
            .lock()
            .unwrap()
            .invoke(id, input.enabled);
    }
    Redirect::to(format!("/flag/{}", id))
}
