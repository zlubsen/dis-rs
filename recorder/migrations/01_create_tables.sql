CREATE TABLE IF NOT EXISTS metadata
(
    id                  INTEGER PRIMARY KEY CHECK (id = 0),
    hostname            TEXT    NOT NULL,
    system_time_started INTEGER NOT NULL,
    frame_time_ms       INTEGER NOT NULL DEFAULT 20
--     id          INTEGER PRIMARY KEY NOT NULL,
--     description TEXT                NOT NULL,
--     done        BOOLEAN             NOT NULL DEFAULT 0
);
