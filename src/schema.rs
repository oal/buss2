// @generated automatically by Diesel CLI.

diesel::table! {
    quays (id) {
        id -> Int4,
        name -> Text,
        lat -> Float8,
        lon -> Float8,
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

diesel::allow_tables_to_appear_in_same_query!(
    quays,
    stops,
);
