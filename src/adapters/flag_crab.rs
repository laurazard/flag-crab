use std::sync::{Arc, Mutex};

use rocket::*;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use rocket_okapi::swagger_ui::*;

use crate::adapters::api::get_flag_api_handler;
use crate::adapters::persistence::flag_repo::FlagRepo;
use crate::adapters::web::add_flag_handler;
use crate::adapters::web::flag_detail_handler;
use crate::adapters::web::home_handler;
use crate::adapters::web::update_flag_handler;

use crate::usecases::add_flag::AddFlag;
use crate::usecases::get_all_flags::GetAllFlags;
use crate::usecases::get_flag::GetFlag;
use crate::usecases::update_flag::UpdateFlag;
use rocket_okapi::routes_with_openapi;

pub struct FlagCrab {}

impl FlagCrab {
    pub fn start(flag_repo: Box<dyn FlagRepo>) {
        let flag_repo_mutex: Arc<Mutex<Box<dyn FlagRepo>>> = Arc::new(Mutex::new(flag_repo));

        let get_all_flags_usecase = GetAllFlags::new(Arc::clone(&flag_repo_mutex));
        let add_flag_usecase = AddFlag::new(Arc::clone(&flag_repo_mutex));
        let update_flag_usecase = UpdateFlag::new(Arc::clone(&flag_repo_mutex));
        let get_flag_usecase = GetFlag::new(Arc::clone(&flag_repo_mutex));

        rocket::ignite()
            .attach(Template::fairing())
            .manage(get_all_flags_usecase)
            .manage(Mutex::new(add_flag_usecase))
            .manage(Mutex::new(update_flag_usecase))
            .manage(get_flag_usecase)
            .mount("/static", StaticFiles::from("templates/static"))
            .mount(
                "/",
                routes![
                    home_handler::index,
                    add_flag_handler::create_flag,
                    update_flag_handler::update_flag,
                    flag_detail_handler::get_flag,
                ],
            )
            .mount("/", routes_with_openapi![get_flag_api_handler::get_flag])
            .mount(
                "/swagger-ui/",
                make_swagger_ui(&SwaggerUIConfig {
                    url: "../openapi.json".to_owned(),
                    ..Default::default()
                }),
            )
            .launch();
    }
}
