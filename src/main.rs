pub mod web;
mod persistance;

#[macro_use] extern crate rocket;

use crate::web::controllers::stream_controller::get_streams;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/v1/streams", routes![get_streams])
        .attach(persistance::diesel_db_pool::stage())
}