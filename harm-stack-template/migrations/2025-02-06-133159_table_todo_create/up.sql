-- Your SQL goes here
CREATE TABLE todos (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT false
);
