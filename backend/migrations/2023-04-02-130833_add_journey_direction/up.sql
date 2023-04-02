create type direction_enum as enum ('inbound', 'outbound');

alter table journeys
    add column direction direction_enum not null default 'inbound';