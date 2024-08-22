-- Add up migration script here
CREATE TABLE IF NOT EXISTS book (
    "id" int unique primary key,
    "title" varchar,
    "author" varchar,
    "publisher" varchar
)