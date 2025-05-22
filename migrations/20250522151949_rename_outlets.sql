ALTER TABLE chargers
    RENAME COLUMN outlets to evses;

ALTER TABLE chargers
    DROP COLUMN status;
