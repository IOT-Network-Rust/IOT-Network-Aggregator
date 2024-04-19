//! Holds data about device
//!

use std::any;

use failure::{format_err, Error};
use rusqlite;
use serde_derive::{Deserialize, Serialize};
use chrono::Local;


#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
/// Represents the types that a device sensor entry can be assigned
pub enum DataType {
    NUMBER,
    FILE,
}

impl DataType {
    pub fn from_sql_type(sql_type: &String) -> Result<Self, Error> {
        let number_type = String::from("REAL");
        let text_type = String::from("TEXT");
        match sql_type.to_uppercase() {
            number_type => Ok(DataType::NUMBER),
            text_type => Ok(DataType::FILE),
            _ => Err(format_err!("Datatype was unexpected")),
        }
    }

    pub fn as_sql_type(&self) -> String {
        match self {
            Self::NUMBER => String::from("REAL"),
            Self::FILE => String::from("TEXT"),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SensorTable {
    pub name: String,
    pub data_type: DataType,
}

impl PartialEq for  SensorTable {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.data_type == other.data_type
    }
}

impl SensorTable {
    pub fn new(name: &String, data_type: &String) -> Self {
        let data_type = DataType::from_sql_type(data_type).unwrap();
        SensorTable {
            name: name.clone(),
            data_type,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct DataEntry {
    value: String,
    time: String,
}

impl DataEntry {
    pub fn new(value: String) -> Self {
        let time = get_current_time();
        DataEntry {
            value,
            time,
        }
    }
}

/// Returns string representing current time
pub fn get_current_time() -> String {
    Local::now().to_rfc3339()
}

/// Returns database name structure given device id
pub fn to_database_name(id: &String) -> String {
    format!("{}.db", id)
}

/// Creates a connection to a database
pub fn open_connection(folder: &String, id: &String) -> Result<rusqlite::Connection, Error> {
    let path = std::path::Path::new(folder).join(&to_database_name(id));
    if let Ok(conn) = rusqlite::Connection::open(path) {
        Ok(conn)
    } else {
        Err(format_err!("There was an error starting database connection"))
    }
}

/// Creates a table within sqlite3 database
pub fn create_table(conn: &rusqlite::Connection, table: &SensorTable) -> Result<(), Error> {
    // Structuring command
    let command = format!(
        "CREATE TABLE {} (
                id    INTEGER PRIMARY KEY,
                data  {} NOT NULL,
                timestamp TEXT NOT NULL
            )",
        table.name,
        table.data_type.as_sql_type()
    );
    println!("{}", command);
    // Checking for execution error
    if let Err(e) = conn.execute(command.as_str(), ()) {
        return Err(format_err!("There was a problem creating table"));
    }

    Ok(())
}

/// Inserts a data point into sqlite3 database table
pub fn insert_value(
    conn: &rusqlite::Connection,
    table_name: &String,
    value: &DataEntry,
) -> Result<(), Error> {
    // Structuring command
    let command: String = format!("INSERT INTO {} (data, timestamp) VALUES (?, ?)", table_name);

    // Checking for execution error
    if let Err(e) = conn.execute(&command, &[&value.value, &value.time]) {
        return Err(format_err!("There was a problem inserting into table"));
    }

    Ok(())
}

/// returns the datatype that value is in table
pub fn get_table_data_type(
    conn: &rusqlite::Connection,
    table_name: &String,
) -> Result<DataType, Error> {
    // TODO: Test code, implement error handling
    let command = format!("SELECT typeof(data) FROM {}", table_name);
    let mut stmt = conn.prepare(&command).unwrap();

    let mut row = stmt.query_row([], |row| {
        let data_type: String = row.get::<usize, String>(0).unwrap().to_string();
        Ok(DataType::from_sql_type(&data_type).unwrap())
    });

    Ok(row.unwrap())
}

/// Gets tables within device database
pub fn get_tables(conn: &rusqlite::Connection) -> Result<Vec<SensorTable>, Error> {
    // Preparing statement
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table'")?;

    // Execute the statement and collect results
    let table_names: Vec<String> = stmt
        .query_map([], |row| row.get(0))?
        .filter_map(|x| x.ok())
        .collect();

    // Iterate over table names to construct SensorTable instances
    let mut tables = Vec::new();
    for name in table_names {
        let data_type = get_table_data_type(conn, &name).unwrap();
        tables.push(SensorTable { name, data_type });
    }

    Ok(tables)
}

/// Gets entries within table
pub fn get_table_entries(
    conn: &rusqlite::Connection,
    table_name: &String,
) -> Result<Vec<DataEntry>, Error> {
    // Preparing statement
    let stmt = conn.prepare(&format!("SELECT data, timestamp FROM {}", table_name));
    if let Err(_) = stmt {
        return Err(format_err!("Could not process command"));
    };
    let mut stmt = stmt.unwrap();

    let table_type = get_table_data_type(conn, table_name).unwrap();

    // Creating iterator over table
    let entry_iter = stmt.query_map([], |row| {
        let value: String = match table_type {
            DataType::NUMBER => {
                row.get::<usize, f64>(0)?.to_string()
            }
            DataType::FILE => {
                row.get::<usize, String>(0)?
            }
        };
        let time:String = row.get::<usize, String>(1)?;
        Ok(DataEntry {
            value,
            time,
        })
    })?;

    // Gathering all data
    let mut entries = vec![];
    for entry in entry_iter {
        let entry = entry.unwrap();
        entries.push(entry);
    }

    Ok(entries)
}


pub fn init_database(id: &String, tables: &Vec<SensorTable>) -> Result<(), Error> {
    let conn = open_connection(&String::from("dbs"), id)?;
    
    let existing_tables = get_tables(&conn).unwrap();

    let mut tables_to_create = vec![];
    for table in tables {
        if !existing_tables.contains(&table) {
            tables_to_create.push(table);
        }
    }

    for table in tables_to_create.clone() {
        create_table(&conn, &table);
    }

    // Add one database entry so we can access typing in the future
    for table in tables_to_create {
        let value = String::from(if table.data_type == DataType::NUMBER {"0"} else {""});
        insert_value(&conn, &table.name, &DataEntry::new(value));
    }

    conn.close().unwrap();

    Ok(())
}