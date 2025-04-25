CREATE TABLE IF NOT EXISTS metadata
(
    id                  INTEGER PRIMARY KEY CHECK (id = 1),
    hostname            TEXT    NOT NULL,
    time_created_utc_ms INTEGER NOT NULL,
    frame_duration_ms   INTEGER NOT NULL DEFAULT 20,
    schema_version      TEXT    NOT NULL
);

CREATE TABLE IF NOT EXISTS streams
(
    id          INTEGER PRIMARY KEY,
    name        TEXT    NOT NULL,
    protocol    TEXT    NOT NULL
);

CREATE TABLE IF NOT EXISTS frames
(
    id          INTEGER PRIMARY KEY,
    time_from   INTEGER NOT NULL,
    time_to     INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS packets
(
    id                  INTEGER PRIMARY KEY,
    stream_id           INTEGER NOT NULL DEFAULT 0,
    frame_id            INTEGER NOT NULL,
    time_received       INTEGER NOT NULL,
    time_since_start_ms INTEGER NOT NULL,
    bytes               BLOB,
    FOREIGN KEY (stream_id)
        REFERENCES streams (id),
    FOREIGN KEY (frame_id)
        REFERENCES frames (id)
);

CREATE TABLE IF NOT EXISTS marker_type
(
    id      INTEGER PRIMARY KEY,
    name    TEXT NOT NULL
);

-- Fill built-in marker types
INSERT INTO marker_type (name)
VALUES  ('event'),
        ('interest'),
        ('dummy');

CREATE TABLE IF NOT EXISTS markers
(
    id          INTEGER PRIMARY KEY,
    marker_type INTEGER NOT NULL,
    label       TEXT NOT NULL,
    FOREIGN KEY (marker_type)
        REFERENCES marker_type (id)
            ON UPDATE SET DEFAULT
            ON DELETE SET DEFAULT
);
