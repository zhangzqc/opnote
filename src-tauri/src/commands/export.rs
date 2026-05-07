use crate::DbConn;
use rusqlite::params;
use std::fs;

#[tauri::command]
pub fn export_node(
    state: tauri::State<'_, DbConn>,
    node_id: i64,
    format: String,
    path: String,
) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let name: String = conn.query_row(
        "SELECT name FROM nodes WHERE id = ?1",
        params![node_id],
        |r| r.get(0),
    ).map_err(|e| e.to_string())?;

    let content: String = conn.query_row(
        "SELECT content FROM contents WHERE node_id = ?1",
        params![node_id],
        |r| r.get(0),
    ).unwrap_or_default();

    let output = match format.as_str() {
        "markdown" | "md" => content,
        "html" => format!(
            "<!DOCTYPE html>\n<html><head><meta charset=\"utf-8\"><title>{}</title></head><body>\n{}\n</body></html>",
            name, content
        ),
        _ => return Err(format!("不支持的导出格式: {}", format)),
    };

    fs::write(&path, output).map_err(|e| e.to_string())?;
    Ok(())
}
