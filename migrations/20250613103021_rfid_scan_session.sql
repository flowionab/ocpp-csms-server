-- Add migration script here
CREATE TABLE rfid_scan_sessions
(
    id                  UUID                                               NOT NULL PRIMARY KEY,
    charger_id          VARCHAR(64)                                        NOT NULL,
    rfid_uid_hex        VARCHAR(32),
    created_at          TIMESTAMP WITH TIME ZONE DEFAULT current_timestamp NOT NULL,
    tag_scanned_at      TIMESTAMP WITH TIME ZONE,
    expires_at          TIMESTAMP WITH TIME ZONE                           NOT NULL,
    ocpp_reservation_id INT                                                NOT NULL
);