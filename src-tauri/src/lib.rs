mod commands;
mod db;
mod utils;

use std::sync::Mutex;
use rusqlite::Connection;
use tauri::Manager;

pub struct DbConn(pub Mutex<Connection>);

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_dir = app.path().app_data_dir().expect("failed to resolve app data dir");
            std::fs::create_dir_all(&app_dir).expect("failed to create app data dir");
            let db_path = app_dir.join("opnote.db");
            let conn = Connection::open(&db_path)
                .unwrap_or_else(|e| panic!("failed to open database at {:?}: {}", db_path, e));
            db::init::init_db(&conn).expect("failed to initialize database");
            app.manage(DbConn(Mutex::new(conn)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::node::get_node_tree,
            commands::node::create_node,
            commands::node::update_node,
            commands::node::delete_node,
            commands::node::move_node,
            commands::content::get_content,
            commands::content::save_content,
            commands::search::search_nodes,
            commands::export::export_node,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
