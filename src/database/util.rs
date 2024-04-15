use super::error;
use chrono::{DateTime, Local};
use rusqlite::Connection;
use std::path::{Path, PathBuf};

const DATABASE_FOLDER: &str = "dbs";

/// Given a path to a database this function will return all
/// tables within this database
pub fn get_database_tables(path: &PathBuf) -> Vec<String> {
    let conn = Connection::open(path).expect(error::FAILURE_TO_OPEN);

    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table'")
        .unwrap();
    let table_iter = stmt
        .query_map([], |row| row.get(0))
        .unwrap()
        .map(|r| r.unwrap());

    let tables: Vec<String> = table_iter.collect();

    //conn.close().expect(error::FAILURE_TO_CLOSE);
    tables
}

/// Returns a dateTime object representing the current time
/// when the function is called
pub fn get_current_time() -> DateTime<Local> {
    Local::now()
}

/// Generates the name for a database given
/// a id for that item
/// This algo assumes the id is unique
pub fn get_database_name(id: &String) -> PathBuf {
    Path::new(format!("{}.db", id).as_str()).to_path_buf()
}

/// Returns the actual path of the database given a path
pub fn get_database_path(path: &PathBuf) -> PathBuf {
    Path::new(DATABASE_FOLDER).join(path)
}

pub fn exists(path: &PathBuf) -> bool {
    Path::exists(path)
}

pub fn open_connection(path: &PathBuf) -> rusqlite::Result<Connection> {
    Connection::open(get_database_path(path))
}
