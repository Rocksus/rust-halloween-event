// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Int8,
        name -> Varchar,
        description -> Text,
        event_date -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
