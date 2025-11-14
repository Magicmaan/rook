pub fn query_apps_fts(conn: &Connection, query: &str) -> rusqlite::Result<Vec<i64>> {
    let mut stmt = conn.prepare(
        "SELECT rowid FROM apps_fts 
         WHERE apps_fts MATCH ?1
         ORDER BY rank",
    )?;
    let app_ids_iter = stmt.query_map(params![query], |row| row.get(0))?;

    let mut app_ids = Vec::new();
    for app_id in app_ids_iter {
        app_ids.push(app_id?);
    }
    Ok(app_ids)
}
