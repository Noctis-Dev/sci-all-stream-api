pub mod web;
pub mod dto;
pub mod types;
pub mod broker;

mod persistance;

#[macro_use] extern crate rocket;

use web::stream_controller::get_streams;

#[get("/health")]
pub fn health() -> &'static str {
    "OK"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/v1/", routes![health])
        .mount("/api/v1/stream", routes![get_streams])
        .attach(persistance::diesel_db_pool::stage())
        .attach(broker::stage())
}