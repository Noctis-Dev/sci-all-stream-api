use chrono::NaiveDate;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

use crate::persistance::schema::streams;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = streams)]
pub struct Stream {
    pub uuid: String,
    pub title: String,
    pub description: Option<String>,
    pub access_token: String,
    pub user_uuid: String,
    pub created_at: NaiveDate,
    pub finalized_at: Option<NaiveDate>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct StreamEvent {
    pub uuid: String,
    pub title: String,
    pub description: Option<String>,
    pub user_id: String,
    pub access_token: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct StreamRequest {
    pub title: String, 
    pub description: Option<String>,
    pub user_id: String,
}