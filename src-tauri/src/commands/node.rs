use crate::db::models::{Node, TreeNode};
use crate::DbConn;
use rusqlite::params;
use std::sync::Mutex;

#[tauri::command]
pub fn get_node_tree(state: tauri::State<'_, DbConn>) -> Result<Vec<TreeNode>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, parent_id, name, node_type, icon, syntax, sort_order, is_expanded, created_at, updated_at FROM nodes ORDER BY sort_order ASC, id ASC"
    ).map_err(|e| e.to_string())?;

    let nodes: Vec<Node> = stmt.query_map([], |row| {
        Ok(Node {
            id: row.get(0)?,
            parent_id: row.get(1)?,
            name: row.get(2)?,
            node_type: row.get(3)?,
            icon: row.get(4)?,
            syntax: row.get(5)?,
            sort_order: row.get(6)?,
            is_expanded: row.get::<_, i32>(7)? != 0,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    }).map_err(|e| e.to_string())?
    .filter_map(|r| r.ok())
    .collect();

    Ok(build_tree(&nodes, None))
}

fn build_tree(nodes: &[Node], parent_id: Option<i64>) -> Vec<TreeNode> {
    let mut children: Vec<TreeNode> = nodes
        .iter()
        .filter(|n| n.parent_id == parent_id)
        .map(|n| {
            let mut tn: TreeNode = n.clone().into();
            tn.children = build_tree(nodes, Some(n.id));
            tn
        })
        .collect();
    children.sort_by_key(|n| n.sort_order);
    children
}

#[tauri::command]
pub fn create_node(
    state: tauri::State<'_, DbConn>,
    parent_id: Option<i64>,
    name: String,
    node_type: String,
) -> Result<Node, String> {
    // Validate depth
    if let Some(pid) = parent_id {
        let depth = get_depth(&state, pid)?;
        if depth >= 10 {
            return Err("节点深度不能超过 10 级".to_string());
        }
    }

    let conn = state.0.lock().map_err(|e| e.to_string())?;

    // Get next sort_order for this parent
    let max_order: i32 = conn.query_row(
        "SELECT COALESCE(MAX(sort_order), -1) FROM nodes WHERE parent_id IS ?1",
        params![parent_id],
        |r| r.get(0),
    ).unwrap_or(-1);

    let icon = match node_type.as_str() {
        "code" => Some("💻".to_string()),
        "folder" => Some("📂".to_string()),
        _ => Some("📄".to_string()),
    };

    conn.execute(
        "INSERT INTO nodes (parent_id, name, node_type, icon, sort_order) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![parent_id, name, node_type, icon, max_order + 1],
    ).map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();

    // Create content row for non-folder nodes
    if node_type != "folder" {
        let format = if node_type == "code" { "plain" } else { "markdown" };
        conn.execute(
            "INSERT INTO contents (node_id, content, format) VALUES (?1, '', ?2)",
            params![id, format],
        ).map_err(|e| e.to_string())?;
    }

    let node = conn.query_row(
        "SELECT id, parent_id, name, node_type, icon, syntax, sort_order, is_expanded, created_at, updated_at FROM nodes WHERE id = ?1",
        params![id],
        |row| Ok(Node {
            id: row.get(0)?,
            parent_id: row.get(1)?,
            name: row.get(2)?,
            node_type: row.get(3)?,
            icon: row.get(4)?,
            syntax: row.get(5)?,
            sort_order: row.get(6)?,
            is_expanded: row.get::<_, i32>(7)? != 0,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
        }),
    ).map_err(|e| e.to_string())?;

    Ok(node)
}

#[tauri::command]
pub fn update_node(
    state: tauri::State<'_, DbConn>,
    id: i64,
    name: Option<String>,
    icon: Option<String>,
    syntax: Option<String>,
) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    if let Some(n) = name {
        conn.execute("UPDATE nodes SET name = ?1 WHERE id = ?2", params![n, id])
            .map_err(|e| e.to_string())?;
    }
    if let Some(i) = icon {
        conn.execute("UPDATE nodes SET icon = ?1 WHERE id = ?2", params![i, id])
            .map_err(|e| e.to_string())?;
    }
    if let Some(s) = syntax {
        conn.execute("UPDATE nodes SET syntax = ?1 WHERE id = ?2", params![s, id])
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn delete_node(state: tauri::State<'_, DbConn>, id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    // CASCADE will handle children, contents, node_tags
    // But FTS needs manual cleanup for children
    let child_ids = get_all_descendant_ids(&conn, id);
    for cid in &child_ids {
        let _ = conn.execute("DELETE FROM contents_fts WHERE rowid = ?1", params![cid]);
    }
    let _ = conn.execute("DELETE FROM contents_fts WHERE rowid = ?1", params![id]);
    conn.execute("DELETE FROM nodes WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn get_all_descendant_ids(conn: &rusqlite::Connection, parent_id: i64) -> Vec<i64> {
    let mut ids = Vec::new();
    let mut stack = vec![parent_id];
    while let Some(pid) = stack.pop() {
        if let Ok(mut stmt) = conn.prepare("SELECT id FROM nodes WHERE parent_id = ?1") {
            let rows: Vec<i64> = stmt.query_map(params![pid], |r| r.get(0))
                .unwrap_or_else(|_| panic!("query failed"))
                .filter_map(|r| r.ok())
                .collect();
            stack.extend(&rows);
            ids.extend(rows);
        }
    }
    ids
}

#[tauri::command]
pub fn move_node(
    state: tauri::State<'_, DbConn>,
    id: i64,
    parent_id: Option<i64>,
    sort_order: i32,
) -> Result<(), String> {
    // Validate depth: compute depth of new parent + current node's own depth
    if let Some(pid) = parent_id {
        let parent_depth = get_depth(&state, pid)?;
        // The node will be at parent_depth + 1, its children add more
        let node_max_child_depth = get_max_child_depth(&state, id)?;
        if parent_depth + 1 + node_max_child_depth > 10 {
            return Err("移动后节点深度将超过 10 级".to_string());
        }
    }

    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE nodes SET parent_id = ?1, sort_order = ?2 WHERE id = ?3",
        params![parent_id, sort_order, id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

/// Get the depth of a node (root=0, child=1, etc.)
fn get_depth(state: &tauri::State<'_, DbConn>, node_id: i64) -> Result<i32, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut depth = 0;
    let mut current_id = Some(node_id);
    while let Some(cid) = current_id {
        let parent: Option<i64> = conn.query_row(
            "SELECT parent_id FROM nodes WHERE id = ?1",
            params![cid],
            |r| r.get(0),
        ).unwrap_or(None);
        if parent.is_some() {
            depth += 1;
            current_id = parent;
        } else {
            break;
        }
    }
    Ok(depth)
}

/// Get the max depth of children below this node (0 if no children)
fn get_max_child_depth(state: &tauri::State<'_, DbConn>, node_id: i64) -> Result<i32, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut max_depth = 0i32;
    let mut current_level = vec![node_id];
    let mut current_depth = 0;
    while !current_level.is_empty() && current_depth < 10 {
        let mut next_level = Vec::new();
        for cid in &current_level {
            if let Ok(mut stmt) = conn.prepare("SELECT id FROM nodes WHERE parent_id = ?1") {
                let children: Vec<i64> = stmt.query_map(params![cid], |r| r.get(0))
                    .unwrap_or_else(|_| panic!("query failed"))
                    .filter_map(|r| r.ok())
                    .collect();
                next_level.extend(children);
            }
        }
        if !next_level.is_empty() {
            current_depth += 1;
            max_depth = max_depth.max(current_depth);
            current_level = next_level;
        } else {
            break;
        }
    }
    Ok(max_depth)
}
