create table stops
(
    id   serial primary key,
    name text             not null,
    lat  double precision not null,
    lon  double precision not null
);