PRAGMA journal_mode = WAL;
PRAGMA busy_timeout = 5000;
PRAGMA foreign_keys = ON;
PRAGMA strict = ON;

CREATE TABLE IF NOT EXISTS organizations (
    id       TEXT    NOT NULL,
    name     TEXT    NOT NULL,
    created  INTEGER NOT NULL,
    modified INTEGER NOT NULL,
    PRIMARY KEY (id)
) STRICT;

CREATE INDEX idx_organizations_name ON organizations (name);

CREATE TABLE IF NOT EXISTS users (
    name     TEXT    NOT NULL,
    hash     TEXT    NOT NULL,
    state    TEXT    NOT NULL,
    created  INTEGER NOT NULL,
    modified INTEGER NOT NULL,
    org_id   TEXT    NOT NULL,
    FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE,
    PRIMARY KEY (name, org_id)
) STRICT;

CREATE TABLE IF NOT EXISTS bases (
    id           TEXT NOT NULL,
    name         TEXT NOT NULL,
    manufacturer TEXT,
    created      INTEGER NOT NULL,
    modified     INTEGER NOT NULL,
    org_id       TEXT NOT NULL,
    FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE,
    PRIMARY KEY (id)
) STRICT;

CREATE TABLE IF NOT EXISTS colorants (
    id           TEXT NOT NULL,
    name         TEXT NOT NULL,
    manufacturer TEXT,
    created      INTEGER NOT NULL,
    modified     INTEGER NOT NULL,
    org_id       TEXT NOT NULL,
    FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE,
    PRIMARY KEY (id)
) STRICT;

CREATE TABLE IF NOT EXISTS formulas (
    id       TEXT NOT NULL,
    name     TEXT NOT NULL,
    number   TEXT,
    notes    TEXT,
    created  INTEGER NOT NULL,
    modified INTEGER NOT NULL,
    org_id   TEXT NOT NULL,
    FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE,
    PRIMARY KEY (id, org_id)
) STRICT;

CREATE INDEX idx_formulas_name ON formulas (name);

-- Formulas only have one base, but a base might be used by multiple colorants.
CREATE TABLE IF NOT EXISTS formulas_bases (
    formula_id TEXT NOT NULL,
    base_id    TEXT NOT NULL,
    org_id     TEXT NOT NULL,
    amount     TEXT NOT NULL,
    FOREIGN KEY (formula_id) REFERENCES formulas(id) ON DELETE CASCADE,
    FOREIGN KEY (base_id) REFERENCES bases(id) ON DELETE CASCADE,
    FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE,
    PRIMARY KEY (formula_id, base_id, org_id)
) STRICT;

CREATE INDEX idx_formulas_bases_id ON formulas_bases (formula_id);

-- Formulas might have many colorants.
-- Colorants may be used for many formulas.
CREATE TABLE IF NOT EXISTS formulas_colorants (
    formula_id  TEXT NOT NULL,
    colorant_id TEXT NOT NULL,
    org_id      TEXT NOT NULL,
    amount      TEXT NOT NULL,
    FOREIGN KEY (formula_id) REFERENCES formulas(id) ON DELETE CASCADE,
    FOREIGN KEY (colorant_id) REFERENCES colorants(id) ON DELETE CASCADE,
    FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE,
    PRIMARY KEY (formula_id, colorant_id, org_id)
) STRICT;

CREATE INDEX idx_formulas_colorants_id ON formulas_colorants (formula_id);

CREATE TABLE IF NOT EXISTS contractors (
    id       TEXT NOT NULL,
    name     TEXT NOT NULL,
    contact  TEXT,
    created  INTEGER NOT NULL,
    modified INTEGER NOT NULL,
    org_id   TEXT NOT NULL,
    FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE,
    PRIMARY KEY (id, org_id)
) STRICT;

CREATE TABLE IF NOT EXISTS jobs (
    id            TEXT NOT NULL,
    name          TEXT NOT NULL,
    address       TEXT,
    contact       TEXT,
    notes         TEXT,
    created       INTEGER NOT NULL,
    modified      INTEGER NOT NULL,
    contractor_id TEXT NOT NULL,
    org_id        TEXT NOT NULL,
    FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE,
    FOREIGN KEY (contractor_id) REFERENCES contractors(id) ON DELETE CASCADE,
    PRIMARY KEY (id, org_id, contractor_id)
) STRICT;

-- One job may have multiple formulas.
-- Multiple formulas may be used with many jobs.
CREATE TABLE IF NOT EXISTS jobs_formulas (
    job_id      TEXT NOT NULL,
    formula_id  TEXT NOT NULL,
    org_id      TEXT NOT NULL,
    FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE CASCADE,
    FOREIGN KEY (formula_id) REFERENCES formulas(id) ON DELETE CASCADE,
    FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE,
    PRIMARY KEY (job_id, formula_id, org_id)
) STRICT;

CREATE TABLE IF NOT EXISTS api_tokens (
    encrypted_token     TEXT    NOT NULL,
    created             INTEGER NOT NULL,
    duration            INTEGER NOT NULL,
    org_id              TEXT    NOT NULL,
    username            TEXT    NOT NULL,
    FOREIGN KEY (username) REFERENCES users(name) ON DELETE CASCADE,
    FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE,
    PRIMARY KEY (encrypted_token, org_id)
) STRICT;
