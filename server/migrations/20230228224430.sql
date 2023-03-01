CREATE TABLE IF NOT EXISTS repositories (
    id          INTEGER PRIMARY KEY NOT NULL,
    name        TEXT                NOT NULL,
    description TEXT                NOT NULL,
    url         TEXT                NOT NULL,
    created_at  TEXT                NOT NULL,
    updated_at  TEXT                NOT NULL,
    pushed_at   TEXT                NOT NULL,
    pipelines   TEXT                NOT NULL,
);