-- Add migration script here
CREATE TABLE transactions
(
    id                  UUID                                  NOT NULL PRIMARY KEY,
    charger_id          VARCHAR(64)                           NOT NULL,
    ocpp_transaction_id VARCHAR(32)                           NOT NULL,
    start_time          TIMESTAMPTZ DEFAULT current_timestamp NOT NULL,
    end_time            TIMESTAMPTZ,
    watt_charged        INT         DEFAULT 0                 NOT NULL,
    is_authorized       BOOL        DEFAULT false             NOT NULL
);