CREATE TABLE IF NOT EXISTS nodes (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    type TEXT NOT NULL,
    payload BLOB,
    wasm_hash TEXT,
    permission TEXT CHECK(permission IN ('LOCKED', 'OPEN')) NOT NULL DEFAULT 'OPEN'
);

CREATE TABLE IF NOT EXISTS edges (
    from_id TEXT NOT NULL,
    to_id TEXT NOT NULL,
    contract TEXT,
    PRIMARY KEY (from_id, to_id),
    FOREIGN KEY(from_id) REFERENCES nodes(id),
    FOREIGN KEY(to_id) REFERENCES nodes(id)
);

CREATE TABLE IF NOT EXISTS ledger (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    actor TEXT NOT NULL,
    action TEXT NOT NULL,
    target_node TEXT,
    FOREIGN KEY(target_node) REFERENCES nodes(id)
);
