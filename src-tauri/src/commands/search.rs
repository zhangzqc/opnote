use crate::db::models::SearchResult;
use crate::DbConn;
use rusqlite::params;

#[tauri::command]
pub fn search_nodes(
    state: tauri::State<'_, DbConn>,
    query: String,
) -> Result<Vec<SearchResult>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let fts_query = format!("{}*", query);
    let mut stmt = conn.prepare(
        "SELECT f.rowid, n.name, snippet(contents_fts, 1, '【', '】', '...', 32) as snippet, f.rank
         FROM contents_fts f
         JOIN nodes n ON n.id = f.rowid
         WHERE contents_fts MATCH ?1
         ORDER BY f.rank
         LIMIT 50"
    ).map_err(|e| e.to_string())?;

    let results = stmt.query_map(params![fts_query], |row| {
        Ok(SearchResult {
            node_id: row.get(0)?,
            name: row.get(1)?,
            snippet: row.get(2)?,
            rank: row.get(3)?,
        })
    }).map_err(|e| e.to_string())?
    .filter_map(|r| r.ok())
    .collect();

    Ok(results)
}
