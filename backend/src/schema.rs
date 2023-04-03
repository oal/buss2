// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "direction_enum"))]
    pub struct DirectionEnum;
}

diesel::table! {
    estimated_calls (id) {
        id -> Int4,
        journey_id -> Int4,
        order_in_journey -> Int4,
        quay_id -> Int4,
        is_cancelled -> Bool,
        is_inaccurate -> Bool,
        target_arrival_time -> Nullable<Timestamptz>,
        target_departure_time -> Nullable<Timestamptz>,
        expected_arrival_time -> Nullable<Timestamptz>,
        expected_departure_time -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::DirectionEnum;

    journeys (id) {
        id -> Int4,
        route_id -> Int4,
        journey_ref -> Text,
        direction -> DirectionEnum,
    }
}

diesel::table! {
    quays (id) {
        id -> Int4,
        name -> Text,
        lat -> Float8,
        lon -> Float8,
        stop_id -> Int4,
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

diesel::joinable!(estimated_calls -> journeys (journey_id));
diesel::joinable!(estimated_calls -> quays (quay_id));
diesel::joinable!(journeys -> routes (route_id));
diesel::joinable!(quays -> stops (stop_id));

diesel::allow_tables_to_appear_in_same_query!(
    estimated_calls,
    journeys,
    quays,
    routes,
    stops,
);
