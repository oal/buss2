create table routes
(
    id         serial primary key,
    short_name text not null,
    name       text not null
);