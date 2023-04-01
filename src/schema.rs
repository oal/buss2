// @generated automatically by Diesel CLI.

diesel::table! {
    quays (id) {
        id -> Int4,
        name -> Text,
        lat -> Float8,
        lon -> Float8,
        stop_id -> Nullable<Int4>,
    }
}

diesel::table! {
    routes (id) {
        id -> Int4,
        short_name -> Text,
        name -> Text,
    }
}

diesel::table! {
    stops (id) {
        id -> Int4,
        name -> Text,
        lat -> Float8,
        lon -> Float8,
    }
}

diesel::joinable!(quays -> stops (stop_id));

diesel::allow_tables_to_appear_in_same_query!(
    quays,
    routes,
    stops,
);
