use crate::domain::flag::Flag;
use crate::usecases::get_flag::GetFlag;

use rocket::*;
use rocket_contrib::json::Json;
use rocket_okapi::openapi;

#[openapi]
#[get("/api/flag/<id>")]
pub fn get_flag(id: u32, get_flag_usecase: State<GetFlag>) -> Json<Flag> {
    Json(get_flag_usecase.invoke(id).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::persistence::flag_repo::FlagRepo;
    use crate::adapters::persistence::in_memory_flag_repo::InMemoryFlagRepo;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test() {
        let mut flag_repo: Box<dyn FlagRepo> = Box::new(InMemoryFlagRepo::new());
        flag_repo.add_flag(Flag::new(1, String::from("A test flag name")));
        let flag_repo_mutex = Arc::new(Mutex::new(flag_repo));
        let get_flag_usecase = GetFlag::new(Arc::clone(&flag_repo_mutex));
        // FIXME: find a way to do this without waiting a random amount of time for it to be ready
        thread::spawn(|| {
            let config = rocket::config::Config::build(rocket::config::Environment::Development)
                .port(8004)
                .unwrap();
            rocket::custom(config)
                .manage(get_flag_usecase)
                .mount("/", routes![super::get_flag])
                .launch();
        });
        thread::sleep(Duration::from_millis(100));

        let response = reqwest::blocking::get("http://localhost:8004/api/flag/1")
            .unwrap()
            .text()
            .unwrap();

        println!("{}", response);
    }
}
