-- migrations/0001_notes.sql
CREATE TABLE IF NOT EXISTS notes (
    id      INTEGER PRIMARY KEY AUTOINCREMENT,
    titre   TEXT NOT NULL,
    contenu TEXT NOT NULL
);
