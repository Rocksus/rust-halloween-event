// @generated automatically by Diesel CLI.

diesel::table! {
    bookings (id) {
        id -> Int8,
        booking_id -> Int8,
        user_id -> Int8,
        created_at -> Timestamp,
    }
}

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
    bookings,
    events,
    users,
);
