create table estimated_calls
(
    id                      serial primary key,

    journey_id              integer references journeys (id) on delete cascade not null,
    order_in_journey        integer                                            not null,

    quay_id                 integer references quays (id) on delete cascade    not null,

    is_cancelled            boolean default false                              not null,
    is_inaccurate           boolean default false                              not null,

    target_arrival_time     timestamp with time zone,
    target_departure_time   timestamp with time zone,
    expected_arrival_time   timestamp with time zone,
    expected_departure_time timestamp with time zone,

    unique (journey_id, order_in_journey)
)