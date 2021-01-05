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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::persistence::flag_repo::FlagRepo;
    use crate::adapters::persistence::in_memory_flag_repo::InMemoryFlagRepo;
    use crate::domain::flag::Flag;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test() {
        let mut flag_repo: Box<dyn FlagRepo> = Box::new(InMemoryFlagRepo::new());
        flag_repo.add_flag(Flag::new(1, String::from("A test flag name")));
        let flag_repo_mutex = Arc::new(Mutex::new(flag_repo));
        let update_flag_usecase = UpdateFlag::new(Arc::clone(&flag_repo_mutex));

        // FIXME: find a way to do this without waiting a random amount of time for it to be ready
        thread::spawn(|| {
            rocket::ignite()
                .manage(Mutex::new(update_flag_usecase))
                .mount("/", routes![super::update_flag])
                .launch();
        });
        thread::sleep(Duration::from_millis(100));

        let params = [("enabled", "false")];
        reqwest::blocking::Client::new()
            .post("http://localhost:8000/flag/1")
            .form(&params)
            .send();

        assert_eq!(
            false,
            flag_repo_mutex.lock().unwrap().get_all_flags()[0].enabled
        );
    }
}
