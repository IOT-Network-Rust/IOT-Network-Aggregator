//! Holds data about device
//!

use failure::{format_err, Error};
use rusqlite;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
/// Represents the types that a device sensor entry can be assigned
pub enum DataType {
    NUMBER,
    FILE,
}

impl DataType {
    pub fn from_sql_type(sql_type: String) -> Result<Self, Error> {
        let number_type = String::from("REAL");
        let text_type = String::from("TEXT");
        match sql_type {
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

#[derive(Deserialize, Serialize)]
pub struct SensorTable {
    name: String,
    data_type: DataType,
}

#[derive(Deserialize, Serialize)]
pub struct DataEntry {
    value: String,
    time: String,
}

/// Returns database name structure given device id
pub fn to_database_name(id: String) -> String {
    format!("{}.sqlite3", id)
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

    // Checking for execution error
    if let Err(e) = conn.execute(command.as_str(), ()) {
        return Err(format_err!("There was a problem creating table"));
    }

    Ok(())
}

/// Inserts a data point into sqlite3 database table
pub fn insert_value(
    conn: &rusqlite::Connection,
    table_name: String,
    value: &DataEntry,
) -> Result<(), Error> {
    // Structuring command
    let command: String = format!("INSERT INTO {} (data, timestamp) VALUES (?, ?)", table_name);

    // Checking for execution error
    if let Err(e) = conn.execute(&command, [&value.value, &value.time]) {
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

    let mut rows = stmt.query([]).unwrap();

    // Extract the first row (if any)
    if let Some(row) = rows.next().unwrap() {
        // Extract the first value (type of "data" column)
        let sql_type: String = row.get(0).unwrap();
        Ok(DataType::from_sql_type(sql_type).unwrap())
    } else {
        Err(format_err!("Could not find type"))
    }
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

    // Creating iterator over table
    let entry_iter = stmt.query_map([], |row| {
        Ok(DataEntry {
            value: row.get(0)?,
            time: row.get(1)?,
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
