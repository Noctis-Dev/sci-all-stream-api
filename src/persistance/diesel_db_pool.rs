use rocket::{fairing::AdHoc, Build, Rocket};
use rocket_db_pools::{diesel::MysqlPool, Database};

#[derive(Database)]
#[database("diesel_mysql")]
struct Db(MysqlPool);

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Mysql stage", |rocket| async {
        rocket.attach(Db::init())
    })
}