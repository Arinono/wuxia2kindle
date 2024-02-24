-- Add migration script here
ALTER TABLE books ADD COLUMN cover text DEFAULT NULL;
