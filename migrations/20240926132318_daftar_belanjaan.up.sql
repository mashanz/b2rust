-- Add up migration script here
CREATE TABLE IF NOT EXISTS daftar_belanjaan (
    id SERIAL PRIMARY KEY,
    name varchar,
    qty integer,
    price bigint
);