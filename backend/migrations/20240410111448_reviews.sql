-- Add migration script here
create table reviews (
    id bigserial PRIMARY KEY,
    name VARCHAR NOT NULL,
    review VARCHAR NOT NULL
);
