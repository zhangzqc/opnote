use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub name: String,
    pub node_type: String,
    pub icon: Option<String>,
    pub syntax: Option<String>,
    pub sort_order: i32,
    pub is_expanded: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeNode {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub name: String,
    pub node_type: String,
    pub icon: Option<String>,
    pub syntax: Option<String>,
    pub sort_order: i32,
    pub is_expanded: bool,
    pub children: Vec<TreeNode>,
}

impl From<Node> for TreeNode {
    fn from(n: Node) -> Self {
        TreeNode {
            id: n.id,
            parent_id: n.parent_id,
            name: n.name,
            node_type: n.node_type,
            icon: n.icon,
            syntax: n.syntax,
            sort_order: n.sort_order,
            is_expanded: n.is_expanded,
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub node_id: i64,
    pub content: String,
    pub format: String,
    pub word_count: i32,
    pub char_count: i32,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub node_id: i64,
    pub name: String,
    pub snippet: String,
    pub rank: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNodeRequest {
    pub parent_id: Option<i64>,
    pub name: String,
    pub node_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveContentRequest {
    pub node_id: i64,
    pub content: String,
    pub format: String,
}
