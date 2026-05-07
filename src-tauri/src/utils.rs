use crate::DbConn;
use rusqlite::params;
use std::sync::Mutex;

/// Validate that moving a node under `parent_id` won't exceed max depth of 10
pub fn validate_node_depth(
    conn: &Mutex<rusqlite::Connection>,
    parent_id: i64,
) -> Result<i32, String> {
    let conn = conn.lock().map_err(|e| e.to_string())?;
    let mut depth = 0i32;
    let mut current = Some(parent_id);

    while let Some(cid) = current {
        let parent: Option<i64> = conn.query_row(
            "SELECT parent_id FROM nodes WHERE id = ?1",
            params![cid],
            |r| r.get(0),
        ).unwrap_or(None);
        if parent.is_some() {
            depth += 1;
            current = parent;
        } else {
            break;
        }
    }
    Ok(depth)
}
