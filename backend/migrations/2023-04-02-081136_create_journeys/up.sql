create table journeys
(
    id          serial primary key,
    route_id    integer references routes (id) on delete cascade,
    journey_ref text not null unique
);