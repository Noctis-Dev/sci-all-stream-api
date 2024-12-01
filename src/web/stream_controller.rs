use diesel::query_dsl::methods::SelectDsl;
use rocket::response::status::Created;
use rocket::{get, serde::json::Json};
use rocket_db_pools::diesel::AsyncConnection;
use rocket_db_pools::{diesel::prelude::RunQueryDsl, Connection};
use uuid::Uuid;
use crate::dto::stream::StreamRequest;
use crate::types::types::Result;

use crate::{
    dto::stream::Stream, 
    persistance::{diesel_db_pool::Db, schema::streams}
};

#[get("/")]
pub async fn get_streams(mut db: Connection<Db>) -> Result<Json<Vec<Stream>>> {
    let streams = streams::table
        .select(streams::all_columns)
        .load::<Stream>(&mut db).await?;

    Ok(Json(streams))
}

#[post("/", format = "application/json", data = "<payload>")]
pub async fn create_stream(mut db: Connection<Db>, payload: Json<StreamRequest>) -> Result<Created<Json<Stream>>> {
    let stream = Stream {
        uuid: Uuid::new_v4().to_string(),
        access_token: Uuid::new_v4().to_string(),
        user_uuid: payload.0.user_id.clone(),
        created_at: chrono::Utc::now().naive_utc().date(),
        finalized_at: None
    };

    let stream = db.transaction(|mut conn| Box::pin(async move {
        diesel::insert_into(streams::table)
            .values(&stream)
            .execute(&mut conn)
            .await?;

        Ok::<_, diesel::result::Error>(stream)
    })).await;

    Ok(Created::new("/").body(Json(stream?)))
}

