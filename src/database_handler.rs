use chrono::{DateTime, Local};

use rusqlite::Connection;
use std::path;

pub const DATA_FOLDER: &str = "dbs";
pub const FAILURE_TO_OPEN: &str = "Could Not Open Database";
pub const FAILURE_TO_CLOSE: &str = "Could Not Close The Database";
pub const FAILURE_TO_INSERT: &str = "Could Not Insert Data";

/// Gets local regional time
fn get_current_time() -> DateTime<Local> {
    Local::now()
}

/// Represents a table in a database
/// Tables only store three data points
/// ID
/// Value
/// Time
///
/// You are able to choose what the Values
/// type will be
pub struct Table {
    pub name: String,
    pub data_type: String,
}

impl Table {
    fn clone(&self) -> Table {
        Table {
            name: self.name.clone(),
            data_type: self.data_type.clone(),
        }
    }
}

/// Loads up a data base
/// If there are any tables that have not been created
/// it will create them
pub fn initialize_database(database_name: &String, mut tables: Vec<Table>) {
    let path_to_database = &path::Path::new(DATA_FOLDER).join(format!("{}.db", database_name));
    let exists: bool = path::Path::exists(path_to_database);

    let conn = Connection::open(path_to_database).expect(FAILURE_TO_OPEN);

    // If the database already exists then make sure we only init the tables
    // than are new.
    let pre_tables: Vec<String> = if exists {
        get_tables(database_name)
    } else {
        vec![]
    };
    let mut not_made_tables: Vec<Table> = vec![];
    for table in &tables {
        if !pre_tables.contains(&table.name) {
            not_made_tables.push(table.clone());
        }
    }

    for table in &tables {
        create_table(database_name, table.name.clone(), table.data_type.clone());
    }

    conn.close().expect(FAILURE_TO_CLOSE);
}

/// Provided the correct arguments this function will create a table that contains items that hold id, time
/// of insertion, and value.
pub fn create_table(database_name: &String, table_name: String, value_type: String) {
    let path_to_database = &path::Path::new(DATA_FOLDER).join(format!("{}.db", database_name));
    let conn = Connection::open(path_to_database).expect(FAILURE_TO_OPEN);

    let command = format!(
        "CREATE TABLE {} (
                id    INTEGER PRIMARY KEY,
                data  {} NOT NULL,
                timestamp TEXT NOT NULL
            )",
        table_name, value_type
    );

    conn.execute(command.as_str(), ());

    conn.close().expect(FAILURE_TO_CLOSE);
}

/// Returns a list containing the list of tables contained within the database
///
pub fn get_tables(database_name: &String) -> Vec<String> {
    let path_to_database = path::Path::new(DATA_FOLDER).join(format!("{}.db", database_name));
    let mut conn = Connection::open(path_to_database).expect(FAILURE_TO_OPEN);

    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table'")
        .unwrap();
    let table_iter = stmt
        .query_map([], |row| row.get(0))
        .unwrap()
        .map(|r| r.unwrap());

    let tables: Vec<String> = table_iter.collect();

    //close().expect(FAILURE_TO_CLOSE);

    tables
}

/// Inserts data into a databases table when databases name and table is specified.
/// Data must follow users rules on their own.
pub fn insert_into_database(database_name: &String, table_name: String, value: String) {
    let path_to_database = path::Path::new(DATA_FOLDER).join(format!("{}.db", database_name));
    let conn = Connection::open(path_to_database).expect(FAILURE_TO_OPEN);
    let current_time = get_current_time();
    let command = format!("INSERT INTO {} (data, timestamp) VALUES (?, ?)", table_name);

    conn.execute(&command, &[&value, &current_time.to_rfc3339()])
        .expect(FAILURE_TO_INSERT);

    conn.close().expect(FAILURE_TO_CLOSE);
}
