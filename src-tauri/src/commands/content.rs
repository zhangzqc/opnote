use crate::db::models::{Content, SaveContentRequest};
use crate::DbConn;
use rusqlite::params;

#[tauri::command]
pub fn get_content(
    state: tauri::State<'_, DbConn>,
    node_id: i64,
) -> Result<Content, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.query_row(
        "SELECT node_id, content, format, word_count, char_count, updated_at FROM contents WHERE node_id = ?1",
        params![node_id],
        |row| Ok(Content {
            node_id: row.get(0)?,
            content: row.get(1)?,
            format: row.get(2)?,
            word_count: row.get(3)?,
            char_count: row.get(4)?,
            updated_at: row.get(5)?,
        }),
    ).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_content(
    state: tauri::State<'_, DbConn>,
    node_id: i64,
    content: String,
    format: String,
) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let word_count = content.split_whitespace().count() as i32;
    let char_count = content.chars().count() as i32;

    conn.execute(
        "INSERT INTO contents (node_id, content, format, word_count, char_count) VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(node_id) DO UPDATE SET content=excluded.content, format=excluded.format, word_count=excluded.word_count, char_count=excluded.char_count",
        params![node_id, content, format, word_count, char_count],
    ).map_err(|e| e.to_string())?;

    // Update FTS: get node name and re-index
    let name: String = conn.query_row(
        "SELECT name FROM nodes WHERE id = ?1",
        params![node_id],
        |r| r.get(0),
    ).unwrap_or_default();

    // FTS triggers handle this automatically, but we also update the name
    conn.execute(
        "INSERT INTO contents_fts(contents_fts, rowid, name, content) VALUES('delete', ?1, '', '')",
        params![node_id],
    ).ok();
    conn.execute(
        "INSERT INTO contents_fts(rowid, name, content) VALUES (?1, ?2, ?3)",
        params![node_id, name, content],
    ).map_err(|e| e.to_string())?;

    Ok(())
}
