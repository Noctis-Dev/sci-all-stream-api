use diesel::prelude::{Insertable, Queryable};

#[derive(Queryable, Insertable)]
#[diesel(table_name = streams)]
struct Stream {
    uuid: String,
    title: String,
    description: String,
}

diesel::table! {
    streams (uuid) {
        uuid -> Text,
        title -> Text,
        description -> Text,
    }
}
