use serde_json::to_string;
use super::util;

use rusqlite::Connection;
use std::path;
use super::error;


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
    let path = util::get_database_path(&util::get_database_name(database_name));
    let exists: bool = util::exists(&path);

    let conn = Connection::open(&path).expect(error::FAILURE_TO_OPEN);

    // If the database already exists then make sure we only init the tables
    // than are new.
    let pre_tables: Vec<String> = if exists {
        util::get_database_tables(&path.clone().to_path_buf())
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

    conn.close().expect(error::FAILURE_TO_CLOSE);
}

/// Provided the correct arguments this function will create a table that contains items that hold id, time
/// of insertion, and value.
pub fn create_table(database_name: &String, table_name: String, value_type: String) {
    let path = util::get_database_path(&util::get_database_name(database_name));
    let conn = Connection::open(path).expect(error::FAILURE_TO_OPEN);

    let command = format!(
        "CREATE TABLE {} (
                id    INTEGER PRIMARY KEY,
                data  {} NOT NULL,
                timestamp TEXT NOT NULL
            )",
        table_name, value_type
    );

    conn.execute(command.as_str(), ());

    conn.close().expect(error::FAILURE_TO_CLOSE);
}

/// Inserts data into a databases table when databases name and table is specified.
/// Data must follow users rules on their own.
pub fn insert_into_database(database_name: &String, table_name: String, value: String) {
    let path = util::get_database_path(&util::get_database_name(database_name));
    let conn = Connection::open(path).expect(error::FAILURE_TO_OPEN);
    
    let current_time = util::get_current_time();
    let command = format!("INSERT INTO {} (data, timestamp) VALUES (?, ?)", table_name);

    conn.execute(&command, &[&value, &current_time.to_rfc3339()])
        .expect(error::FAILURE_TO_INSERT);

    conn.close().expect(error::FAILURE_TO_CLOSE);
}
