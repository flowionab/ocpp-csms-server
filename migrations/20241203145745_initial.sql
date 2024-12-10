-- Add migration script here
CREATE TABLE passwords
(
    id              UUID        NOT NULL PRIMARY KEY,
    charger_id      VARCHAR(64) NOT NULL,
    hashed_password TEXT        NOT NULL,
    created_at      TIMESTAMP DEFAULT current_timestamp,
    last_used_at    TIMESTAMP
);

CREATE TABLE chargers
(
    id                   VARCHAR(64) NOT NULL PRIMARY KEY,
    model                VARCHAR(64),
    vendor               VARCHAR(64),
    serial_number        VARCHAR(64),
    firmware_version     VARCHAR(64),
    iccid                VARCHAR(64),
    imsi                 VARCHAR(64),
    status               VARCHAR(32),
    ocpp1_6configuration TEXT,
    outlets              TEXT
);

CREATE TABLE rfid_tags
(
    id           UUID        NOT NULL PRIMARY KEY,
    rfid_hex     VARCHAR(28) NOT NULL,
    user_id      VARCHAR(32),
    created_at   TIMESTAMP DEFAULT current_timestamp,
    last_used_at TIMESTAMP
);

CREATE TABLE charger_connection_info
(
    id           VARCHAR(64)                           NOT NULL PRIMARY KEY,
    node_address TEXT                                  NOT NULL,
    is_online    BOOL                                  NOT NULL,
    last_seen    TIMESTAMPTZ DEFAULT current_timestamp NOT NULL
);