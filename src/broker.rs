use std::sync::Arc;

use futures::StreamExt;
use rocket_db_pools::diesel::{
    prelude::RunQueryDsl, 
    AsyncConnection
};
use tokio::task;
use lapin::{
    options::BasicConsumeOptions, 
    types::FieldTable, 
    Connection, 
    ConnectionProperties
};
use rocket::{
    fairing::AdHoc, 
    serde::json::from_slice
};

use crate::{
    dto::stream::{
        Stream, 
        StreamEvent
    }, 
    persistance::{
        diesel_db_pool::Db, 
        schema::streams
    }
};

async fn rabbit_listener(db: Arc<Db>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = std::env::var("RABBITMQ_ADDR").unwrap_or_else(|_| "amqp://guest:guest@50.19.40.173:5672".into());
    let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;

    let channel = conn.create_channel().await?;
    let mut consumer = channel.basic_consume(
        "StreamEvent", 
        "my_consumer", 
        BasicConsumeOptions::default(),
        FieldTable::default()
    ).await?;

    while let Some(delivery) = consumer.next().await {
        let delivery = delivery?;
        let data = delivery.data.as_slice();

        match from_slice::<StreamEvent>(data) {
            Ok(event) => {
                println!("Received event: {:?}", event);

                let stream = Stream {
                    uuid: event.uuid.clone(),
                    access_token: event.access_token.clone(),
                    user_uuid: event.user_id.clone(),
                    created_at: chrono::Utc::now().naive_utc().date(),
                    finalized_at: None
                };

                let result = db.get().await?.transaction(|mut conn| Box::pin(async move {
                    diesel::insert_into(streams::table)
                        .values(&stream)
                        .execute(&mut conn)
                        .await?;

                    Ok::<_, diesel::result::Error>(stream)
                })).await?;

                println!("Inserted stream: {:?}", result);
            },
            Err(e) => {
                eprintln!("Failed to parse message: {:?}", e);
            }
        }
    }

    Ok(())
}

pub fn stage() -> AdHoc {
    AdHoc::on_liftoff("RabbitListener", | rocket | {
        Box::pin(async move {
            let db = rocket.state::<Db>().unwrap().clone();
            task::spawn(rabbit_listener(Arc::new(db)));
        })
    })
}