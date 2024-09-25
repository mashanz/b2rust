-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    id uuid DEFAULT gen_random_uuid(),
    name varchar,
    email varchar,
    password varchar,
    role varchar,
    created_at timestamp DEFAULT now()
);