// @generated automatically by Diesel CLI.

diesel::table! {
    estimated_calls (id) {
        id -> Int4,
        journey_id -> Nullable<Int4>,
        order_in_journey -> Int4,
        quay_id -> Nullable<Int4>,
        is_cancelled -> Nullable<Bool>,
        is_inaccurate -> Nullable<Bool>,
        target_arrival_time -> Nullable<Timestamptz>,
        target_departure_time -> Nullable<Timestamptz>,
        expected_arrival_time -> Nullable<Timestamptz>,
        expected_departure_time -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    journeys (id) {
        id -> Int4,
        route_id -> Nullable<Int4>,
        journey_ref -> Text,
    }
}

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
