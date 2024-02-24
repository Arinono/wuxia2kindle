-- Add migration script here
ALTER TABLE users ADD token VARCHAR(64) NULL;
