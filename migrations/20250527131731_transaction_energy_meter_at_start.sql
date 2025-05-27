-- Add migration script here
ALTER TABLE transactions
    ADD COLUMN energy_meter_at_start INT default NULL;