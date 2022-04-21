CREATE TABLE IF NOT EXISTS organizations (
    id       TEXT    NOT NULL,
    name     TEXT    NOT NULL,
    created  INTEGER NOT NULL,
    modified INTEGER NOT NULL,
    PRIMARY KEY (id)
);

CREATE INDEX idx_organizations_name ON organizations (name);

CREATE TABLE IF NOT EXISTS users (
    id       TEXT    NOT NULL,
    name     TEXT    NOT NULL,
    hash     TEXT    NOT NULL,
    state    TEXT    NOT NULL,
    created  INTEGER NOT NULL,
    modified INTEGER NOT NULL,
    org_id   TEXT    NOT NULL,
    FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE,
    PRIMARY KEY (id, org_id)
);

CREATE TABLE IF NOT EXISTS bases (
    id           TEXT NOT NULL,
    name         TEXT NOT NULL,
    manufacturer TEXT,
    created      INTEGER NOT NULL,
    modified     INTEGER NOT NULL,
    org_id       TEXT NOT NULL,
    FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS colorants (
    id           TEXT NOT NULL,
    name         TEXT NOT NULL,
    manufacturer TEXT,
    created      INTEGER NOT NULL,
    modified     INTEGER NOT NULL,
    org_id       TEXT NOT NULL,
    FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS formulas (
    id       TEXT NOT NULL,
    name     TEXT NOT NULL,
    number   TEXT NOT NULL,
    notes    TEXT,
    created  INTEGER NOT NULL,
    modified INTEGER NOT NULL,
    org_id   TEXT NOT NULL,
    FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE,
    PRIMARY KEY (id, org_id)
);

CREATE INDEX idx_formulas_name ON formulas (name);

CREATE TABLE IF NOT EXISTS formulas_bases (
    formula_id TEXT NOT NULL,
    base_id TEXT NOT NULL,
    amount TEXT NOT NULL,
    FOREIGN KEY (formula_id) REFERENCES formulas(id) ON DELETE CASCADE,
    FOREIGN KEY (base_id) REFERENCES bases(id) ON DELETE CASCADE,
);

CREATE INDEX idx_formulas_bases_id ON formulas_bases (formula_id);

CREATE TABLE IF NOT EXISTS formulas_colorants (
    formula_id TEXT NOT NULL,
    colorant_id TEXT NOT NULL,
    amount TEXT NOT NULL,
    FOREIGN KEY (formula_id) REFERENCES formulas(id) ON DELETE CASCADE,
    FOREIGN KEY (colorant_id) REFERENCES colorants(id) ON DELETE CASCADE,
);

CREATE INDEX idx_formulas_colorants_id ON formulas_colorants (formula_id);

