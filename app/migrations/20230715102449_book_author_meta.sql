-- Add migration script here
ALTER TABLE books ADD COLUMN author varchar(50) DEFAULT null;
ALTER TABLE books ADD COLUMN translator varchar(50) DEFAULT null;
