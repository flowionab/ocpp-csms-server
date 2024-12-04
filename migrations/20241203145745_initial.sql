-- Add migration script here
CREATE TABLE passwords (
    id UUID NOT NULL PRIMARY KEY,
    charger_id VARCHAR(64) NOT NULL,
    hashed_password TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT current_timestamp,
    last_used_at TIMESTAMP
);

CREATE TABLE chargers (
  id VARCHAR(64) NOT NULL PRIMARY KEY,
  model VARCHAR(64),
  vendor VARCHAR(64),
  serial_number VARCHAR(64),
  firmware_version VARCHAR(64),
  iccid VARCHAR(64),
  imsi VARCHAR(64)
);