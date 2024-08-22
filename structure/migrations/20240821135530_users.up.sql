-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    "id" int UNIQUE PRIMARY KEY,
    "username" varchar
)