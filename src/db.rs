use rusqlite::Connection;

use crate::settings::settings::get_settings_path;

pub fn open_connection() -> Connection {
    let path = get_settings_path().join("rook.db");
    Connection::open(path).expect("Failed to open database")
    // Implementation for opening a database connection
}
pub fn close_connection(conn: Connection) {
    // Implementation for closing a database connection
}

pub fn create_db(conn: Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS apps (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                exec TEXT NOT NULL,
                comment TEXT,
                terminal BOOLEAN NOT NULL,
                file_path TEXT NOT NULL,
                categories JSON DEFAULT ('[]'),
                mime_types JSON DEFAULT ('[]'),
                tag_ids TEXT
            )",
        [],
    )
    .expect("Failed to create table");
    conn.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS apps_fts USING fts5(
                name,
                categories,
                tag_ids,
                content='apps',
                content_rowid='id'
            )",
        [],
    )
    .expect("Failed to create FTS table");
    //
    //
    conn.execute(
        "CREATE TABLE IF NOT EXISTS history (
                id INTEGER PRIMARY KEY,
                query TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                result_ids TEXT 
            )",
        [],
    )
    .expect("Failed to create table");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                color TEXT,
                icon TEXT
            )",
        [],
    )
    .expect("Failed to create table");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS adjustments (
                id INTEGER PRIMARY KEY,
                result_id INTEGER NOT NULL,
                adjustment INTEGER NOT NULL
            )",
        [],
    )
    .expect("Failed to create table");
}
