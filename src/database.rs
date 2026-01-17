use derive_deref::{Deref, DerefMut};
use futures::sink::Send;
use rusqlite::{Connection, Result, params};
use std::fs;

#[derive(Debug)]
pub struct Database {
    path: String,
    connection: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let connection = Connection::open(path)?;
        log::info!("Database connected at path: {}", path);
        Ok(Database {
            path: path.to_string(),
            connection,
        })
    }
    pub fn initialise(&self) -> Result<()> {
        use crate::db::one;
        let migration_sql = one::MIGRATION;
        self.connection.execute_batch(&migration_sql)?;
        log::info!("Database initialised.");
        Ok(())
    }
    pub fn get_connection(&self) -> &Connection {
        &self.connection
    }
    pub fn get_connection_mut(&mut self) -> &mut Connection {
        &mut self.connection
    }
    pub fn start_transaction(&mut self) -> Result<rusqlite::Transaction> {
        self.connection.transaction()
    }

    pub fn insert_application(
        &mut self,
        transaction: &rusqlite::Transaction,
        name: &str,
        file_path: &str,
        file_type: &str,
        terminal: bool,
        modified_at: &str,
    ) -> Result<()> {
        self.connection.execute(
            "INSERT INTO applications (name, file_path, file_type, terminal, modified_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![name, file_path, file_type, terminal, modified_at],
        )?;
        Ok(())
    }
}
