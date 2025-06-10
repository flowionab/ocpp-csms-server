-- Add migration script here
ALTER TABLE chargers
    ADD COLUMN settings TEXT default '{}' NOT NULL;