CREATE TABLE IF NOT EXISTS organizations (
    id       TEXT    PRIMARY KEY,
    name     TEXT    NOT NULL,
    created  INTEGER NOT NULL,
    modified INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
    id       TEXT    PRIMARY KEY,
    name     TEXT    NOT NULL,
    hash     TEXT    NOT NULL,
    state    TEXT    NOT NULL,
    created  INTEGER NOT NULL,
    modified INTEGER NOT NULL,
    org_id   TEXT    NOT NULL,
    FOREIGN KEY (org_id) REFERENCES organizations(id)
);


