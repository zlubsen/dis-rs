CREATE TABLE IF NOT EXISTS metadata
(
    id                  INTEGER PRIMARY KEY CHECK (id = 1),
    hostname            TEXT    NOT NULL,
    time_started_utc    INTEGER NOT NULL,
    frame_time_ms       INTEGER NOT NULL DEFAULT 20,
    schema_version      TEXT NOT NULL
);


