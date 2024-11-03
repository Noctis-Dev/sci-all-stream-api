use rocket::get;

#[get("/get_all")]
pub fn get_streams() -> &'static str {
    "OK"
}