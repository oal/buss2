create table journeys
(
    id          serial primary key,
    route_id    integer references routes (id) on delete cascade not null,
    journey_ref text not null unique
);