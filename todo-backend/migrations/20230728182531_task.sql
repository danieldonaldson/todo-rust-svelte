-- Add migration script here
CREATE TABLE tasks (
    id INTEGER PRIMARY KEY NOT NULL,
    description varchar(255) NOT NULL,
    done BOOLEAN NOT NULL DEFAULT 0
);