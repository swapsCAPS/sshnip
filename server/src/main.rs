#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;
use rocket::response::status;
use rocket::State;
use std::sync::{Arc, Mutex};

mod app_config;
mod request_guards;

#[post("/clip", data = "<data>")]
async fn post(
    _api_key: request_guards::ApiKey<'_>,
    data: &str,
    clip: &State<Arc<Mutex<String>>>,
) -> status::Created<String> {
    *clip.lock().unwrap() = data.to_string();
    status::Created::new("/clip").body(String::from("clipped"))
}

#[get("/clip")]
async fn get(_api_key: request_guards::ApiKey<'_>, clip: &State<Arc<Mutex<String>>>) -> String {
    clip.lock().unwrap().to_string()
}

#[launch]
fn server() -> _ {
    rocket::build()
        .attach(AdHoc::config::<app_config::AppConfig>())
        .manage(Arc::new(Mutex::new(String::from(""))))
        .mount("/", routes![get, post])
}

#[cfg(test)]
mod tests;
