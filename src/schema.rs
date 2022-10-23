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

diesel::table! {
    users (id) {
        id -> Int8,
        username -> Varchar,
        hash -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    events,
    users,
);
