-- Add migration script here
ALTER TABLE transactions
    ADD COLUMN evse_id UUID NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000';

ALTER TABLE transactions
    ADD CONSTRAINT charger_id_fk
        FOREIGN KEY (charger_id)
            REFERENCES chargers (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE;