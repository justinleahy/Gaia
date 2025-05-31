-- Add up migration script here
CREATE TABLE users (
    id UUID PRIMARY KEY,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE
);