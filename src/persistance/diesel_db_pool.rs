use rocket::fairing::AdHoc;
use rocket_db_pools::{diesel::MysqlPool, Database};

#[derive(Database, Clone)]
#[database("diesel_mysql")]
pub struct Db(MysqlPool);

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Mysql stage", |rocket| async {
        rocket.attach(Db::init())
    })
}