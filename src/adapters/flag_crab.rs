use std::sync::{Mutex, Arc};

use rocket::*;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

use crate::adapters::persistence::flag_repo::FlagRepo;
use crate::adapters::web::add_flag_handler;
use crate::adapters::web::home_handler;
use crate::usecases::get_all_flags::GetAllFlags;
use crate::usecases::add_flag::AddFlag;

pub struct FlagCrab {}

impl FlagCrab {
    pub fn start(flag_repo: Box<dyn FlagRepo>) {
        let flag_repo_mutex: Arc<Mutex<Box<dyn FlagRepo>>> = Arc::new(Mutex::new(flag_repo));

        let get_all_flags_usecase = GetAllFlags::new(Arc::clone(&flag_repo_mutex));
        let add_flag_usecase = AddFlag::new(Arc::clone(&flag_repo_mutex));

        rocket::ignite()
            .attach(Template::fairing())
            .manage(get_all_flags_usecase)
            .manage(Mutex::new(add_flag_usecase))
            .mount("/static", StaticFiles::from("templates/static"))
            .mount("/", routes![home_handler::index, add_flag_handler::create_flag])
            .launch();
    }
}