use rusqlite::{Connection, params};

pub fn init_db(conn: &Connection) -> Result<(), String> {
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
        .map_err(|e| e.to_string())?;

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS nodes (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            parent_id   INTEGER REFERENCES nodes(id) ON DELETE CASCADE,
            name        TEXT NOT NULL,
            node_type   TEXT NOT NULL DEFAULT 'rich_text',
            icon        TEXT,
            syntax      TEXT,
            sort_order  INTEGER NOT NULL DEFAULT 0,
            is_expanded INTEGER NOT NULL DEFAULT 1,
            created_at  TEXT NOT NULL DEFAULT (datetime('now','localtime')),
            updated_at  TEXT NOT NULL DEFAULT (datetime('now','localtime')),
            CHECK (node_type IN ('rich_text', 'code', 'folder'))
        );

        CREATE TABLE IF NOT EXISTS contents (
            node_id     INTEGER PRIMARY KEY REFERENCES nodes(id) ON DELETE CASCADE,
            content     TEXT NOT NULL DEFAULT '',
            format      TEXT NOT NULL DEFAULT 'markdown',
            word_count  INTEGER NOT NULL DEFAULT 0,
            char_count  INTEGER NOT NULL DEFAULT 0,
            updated_at  TEXT NOT NULL DEFAULT (datetime('now','localtime'))
        );

        CREATE VIRTUAL TABLE IF NOT EXISTS contents_fts USING fts5(
            name,
            content,
            content=contents,
            content_rowid=node_id,
            tokenize='unicode61'
        );

        CREATE TABLE IF NOT EXISTS tags (
            id    INTEGER PRIMARY KEY AUTOINCREMENT,
            name  TEXT NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS node_tags (
            node_id INTEGER NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
            tag_id  INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
            PRIMARY KEY (node_id, tag_id)
        );

        CREATE TRIGGER IF NOT EXISTS nodes_updated_at
            AFTER UPDATE ON nodes FOR EACH ROW
        BEGIN
            UPDATE nodes SET updated_at = datetime('now','localtime') WHERE id = NEW.id;
        END;

        CREATE TRIGGER IF NOT EXISTS contents_updated_at
            AFTER UPDATE ON contents FOR EACH ROW
        BEGIN
            UPDATE contents SET updated_at = datetime('now','localtime') WHERE node_id = NEW.node_id;
        END;

        CREATE TRIGGER IF NOT EXISTS contents_ai AFTER INSERT ON contents BEGIN
            INSERT INTO contents_fts(rowid, name, content) VALUES (NEW.node_id, '', NEW.content);
        END;

        CREATE TRIGGER IF NOT EXISTS contents_ad AFTER DELETE ON contents BEGIN
            INSERT INTO contents_fts(contents_fts, rowid, name, content) VALUES('delete', OLD.node_id, '', OLD.content);
        END;

        CREATE TRIGGER IF NOT EXISTS contents_au AFTER UPDATE ON contents BEGIN
            INSERT INTO contents_fts(contents_fts, rowid, name, content) VALUES('delete', OLD.node_id, '', OLD.content);
            INSERT INTO contents_fts(rowid, name, content) VALUES (NEW.node_id, '', NEW.content);
        END;
        "
    ).map_err(|e| e.to_string())?;

    // Seed root node if empty
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM nodes", [], |r| r.get(0))
        .map_err(|e| e.to_string())?;
    if count == 0 {
        conn.execute(
            "INSERT INTO nodes (name, node_type, icon, sort_order) VALUES (?1, 'folder', '📕', 0)",
            params!["运维知识库"],
        ).map_err(|e| e.to_string())?;
        let root_id = conn.last_insert_rowid();
        conn.execute(
            "INSERT INTO contents (node_id, content, format) VALUES (?1, '', 'markdown')",
            params![root_id],
        ).map_err(|e| e.to_string())?;
    }

    Ok(())
}
