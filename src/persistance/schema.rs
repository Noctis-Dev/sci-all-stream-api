// @generated automatically by Diesel CLI.

diesel::table! {
    streams (uuid) {
        #[max_length = 36]
        uuid -> Varchar,
        #[max_length = 36]
        access_token -> Varchar,
        user_uuid -> Varchar,
        created_at -> Date,
        finalized_at -> Nullable<Date>,
    }
}
